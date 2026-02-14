#[doc = "ABTRAINCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abtraincntl(pub u32);
impl Abtraincntl {
    #[must_use]
    #[inline(always)]
    pub const fn abtinqtime(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_abtinqtime(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abtinqload(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_abtinqload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abtinqstartvalue(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_abtinqstartvalue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abtinqen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_abtinqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abtpagetime(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_abtpagetime(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abtpageload(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_abtpageload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abtpagestartvalue(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_abtpagestartvalue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abtpageen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_abtpageen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Abtraincntl {
    #[inline(always)]
    fn default() -> Abtraincntl {
        Abtraincntl(0)
    }
}
impl core::fmt::Debug for Abtraincntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Abtraincntl")
            .field("abtinqtime", &self.abtinqtime())
            .field("abtinqload", &self.abtinqload())
            .field("abtinqstartvalue", &self.abtinqstartvalue())
            .field("abtinqen", &self.abtinqen())
            .field("abtpagetime", &self.abtpagetime())
            .field("abtpageload", &self.abtpageload())
            .field("abtpagestartvalue", &self.abtpagestartvalue())
            .field("abtpageen", &self.abtpageen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Abtraincntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Abtraincntl {{ abtinqtime: {=u16:?}, abtinqload: {=bool:?}, abtinqstartvalue: {=bool:?}, abtinqen: {=bool:?}, abtpagetime: {=u16:?}, abtpageload: {=bool:?}, abtpagestartvalue: {=bool:?}, abtpageen: {=bool:?} }}" , self . abtinqtime () , self . abtinqload () , self . abtinqstartvalue () , self . abtinqen () , self . abtpagetime () , self . abtpageload () , self . abtpagestartvalue () , self . abtpageen ())
    }
}
#[doc = "ACTFIFOSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Actfifostat(pub u32);
impl Actfifostat {
    #[must_use]
    #[inline(always)]
    pub const fn startactintstat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_startactintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn endactintstat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_endactintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn skipactintstat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_skipactintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txintstat(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxintstat(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn isotxintstat(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isotxintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn isorxintstat(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isorxintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn actflag(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_actflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn current_et_idx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_current_et_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn skip_et_idx(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_skip_et_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Actfifostat {
    #[inline(always)]
    fn default() -> Actfifostat {
        Actfifostat(0)
    }
}
impl core::fmt::Debug for Actfifostat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Actfifostat")
            .field("startactintstat", &self.startactintstat())
            .field("endactintstat", &self.endactintstat())
            .field("skipactintstat", &self.skipactintstat())
            .field("txintstat", &self.txintstat())
            .field("rxintstat", &self.rxintstat())
            .field("isotxintstat", &self.isotxintstat())
            .field("isorxintstat", &self.isorxintstat())
            .field("actflag", &self.actflag())
            .field("current_et_idx", &self.current_et_idx())
            .field("skip_et_idx", &self.skip_et_idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Actfifostat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Actfifostat {{ startactintstat: {=bool:?}, endactintstat: {=bool:?}, skipactintstat: {=bool:?}, txintstat: {=bool:?}, rxintstat: {=bool:?}, isotxintstat: {=bool:?}, isorxintstat: {=bool:?}, actflag: {=bool:?}, current_et_idx: {=u8:?}, skip_et_idx: {=u8:?} }}" , self . startactintstat () , self . endactintstat () , self . skipactintstat () , self . txintstat () , self . rxintstat () , self . isotxintstat () , self . isorxintstat () , self . actflag () , self . current_et_idx () , self . skip_et_idx ())
    }
}
#[doc = "ACTSCANCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Actscancntl(pub u32);
impl Actscancntl {
    #[must_use]
    #[inline(always)]
    pub const fn upperlimit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_upperlimit(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn backoff(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_backoff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for Actscancntl {
    #[inline(always)]
    fn default() -> Actscancntl {
        Actscancntl(0)
    }
}
impl core::fmt::Debug for Actscancntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Actscancntl")
            .field("upperlimit", &self.upperlimit())
            .field("backoff", &self.backoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Actscancntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Actscancntl {{ upperlimit: {=u16:?}, backoff: {=u16:?} }}",
            self.upperlimit(),
            self.backoff()
        )
    }
}
#[doc = "ACTSCHCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Actschcntl(pub u32);
impl Actschcntl {
    #[must_use]
    #[inline(always)]
    pub const fn entry_idx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_entry_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn start_act(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_start_act(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Actschcntl {
    #[inline(always)]
    fn default() -> Actschcntl {
        Actschcntl(0)
    }
}
impl core::fmt::Debug for Actschcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Actschcntl")
            .field("entry_idx", &self.entry_idx())
            .field("start_act", &self.start_act())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Actschcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Actschcntl {{ entry_idx: {=u8:?}, start_act: {=bool:?} }}",
            self.entry_idx(),
            self.start_act()
        )
    }
}
#[doc = "ADVTIM"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Advtim(pub u32);
impl Advtim {
    #[must_use]
    #[inline(always)]
    pub const fn advint(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_advint(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_auxptr_thr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_auxptr_thr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_auxptr_thr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_auxptr_thr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Advtim {
    #[inline(always)]
    fn default() -> Advtim {
        Advtim(0)
    }
}
impl core::fmt::Debug for Advtim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Advtim")
            .field("advint", &self.advint())
            .field("rx_auxptr_thr", &self.rx_auxptr_thr())
            .field("tx_auxptr_thr", &self.tx_auxptr_thr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Advtim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Advtim {{ advint: {=u16:?}, rx_auxptr_thr: {=u8:?}, tx_auxptr_thr: {=u8:?} }}",
            self.advint(),
            self.rx_auxptr_thr(),
            self.tx_auxptr_thr()
        )
    }
}
#[doc = "AESCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aescntl(pub u32);
impl Aescntl {
    #[must_use]
    #[inline(always)]
    pub const fn aes_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_aes_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn aes_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_aes_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_polar_pwr_val(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_force_polar_pwr_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_polar_pwr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_polar_pwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_iq_pwr_val(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_force_iq_pwr_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_iq_pwr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_iq_pwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Aescntl {
    #[inline(always)]
    fn default() -> Aescntl {
        Aescntl(0)
    }
}
impl core::fmt::Debug for Aescntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aescntl")
            .field("aes_start", &self.aes_start())
            .field("aes_mode", &self.aes_mode())
            .field("force_polar_pwr_val", &self.force_polar_pwr_val())
            .field("force_polar_pwr", &self.force_polar_pwr())
            .field("force_iq_pwr_val", &self.force_iq_pwr_val())
            .field("force_iq_pwr", &self.force_iq_pwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aescntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aescntl {{ aes_start: {=bool:?}, aes_mode: {=bool:?}, force_polar_pwr_val: {=u8:?}, force_polar_pwr: {=bool:?}, force_iq_pwr_val: {=u8:?}, force_iq_pwr: {=bool:?} }}" , self . aes_start () , self . aes_mode () , self . force_polar_pwr_val () , self . force_polar_pwr () , self . force_iq_pwr_val () , self . force_iq_pwr ())
    }
}
#[doc = "AESKEY127_96"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aeskey12796(pub u32);
impl Aeskey12796 {
    #[must_use]
    #[inline(always)]
    pub const fn aeskey127_96(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_aeskey127_96(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Aeskey12796 {
    #[inline(always)]
    fn default() -> Aeskey12796 {
        Aeskey12796(0)
    }
}
impl core::fmt::Debug for Aeskey12796 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aeskey12796")
            .field("aeskey127_96", &self.aeskey127_96())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aeskey12796 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aeskey12796 {{ aeskey127_96: {=u32:?} }}",
            self.aeskey127_96()
        )
    }
}
#[doc = "AESKEY31_0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aeskey310(pub u32);
impl Aeskey310 {
    #[must_use]
    #[inline(always)]
    pub const fn aeskey31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_aeskey31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Aeskey310 {
    #[inline(always)]
    fn default() -> Aeskey310 {
        Aeskey310(0)
    }
}
impl core::fmt::Debug for Aeskey310 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aeskey310")
            .field("aeskey31_0", &self.aeskey31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aeskey310 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Aeskey310 {{ aeskey31_0: {=u32:?} }}", self.aeskey31_0())
    }
}
#[doc = "AESKEY63_32"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aeskey6332(pub u32);
impl Aeskey6332 {
    #[must_use]
    #[inline(always)]
    pub const fn aeskey63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_aeskey63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Aeskey6332 {
    #[inline(always)]
    fn default() -> Aeskey6332 {
        Aeskey6332(0)
    }
}
impl core::fmt::Debug for Aeskey6332 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aeskey6332")
            .field("aeskey63_32", &self.aeskey63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aeskey6332 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aeskey6332 {{ aeskey63_32: {=u32:?} }}",
            self.aeskey63_32()
        )
    }
}
#[doc = "AESKEY95_64"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aeskey9564(pub u32);
impl Aeskey9564 {
    #[must_use]
    #[inline(always)]
    pub const fn aeskey95_64(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_aeskey95_64(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Aeskey9564 {
    #[inline(always)]
    fn default() -> Aeskey9564 {
        Aeskey9564(0)
    }
}
impl core::fmt::Debug for Aeskey9564 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aeskey9564")
            .field("aeskey95_64", &self.aeskey95_64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aeskey9564 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aeskey9564 {{ aeskey95_64: {=u32:?} }}",
            self.aeskey95_64()
        )
    }
}
#[doc = "AUDIOCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiocntl0(pub u32);
impl Audiocntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn cvsd_bitoder0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cvsd_bitoder0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cvsden0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cvsden0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn aulaw_code0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_aulaw_code0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn aulawen0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_aulawen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sample_type0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sample_type0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn linear_format0(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_linear_format0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
}
impl Default for Audiocntl0 {
    #[inline(always)]
    fn default() -> Audiocntl0 {
        Audiocntl0(0)
    }
}
impl core::fmt::Debug for Audiocntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiocntl0")
            .field("cvsd_bitoder0", &self.cvsd_bitoder0())
            .field("cvsden0", &self.cvsden0())
            .field("aulaw_code0", &self.aulaw_code0())
            .field("aulawen0", &self.aulawen0())
            .field("sample_type0", &self.sample_type0())
            .field("linear_format0", &self.linear_format0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiocntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Audiocntl0 {{ cvsd_bitoder0: {=bool:?}, cvsden0: {=bool:?}, aulaw_code0: {=u8:?}, aulawen0: {=bool:?}, sample_type0: {=u8:?}, linear_format0: {=u8:?} }}" , self . cvsd_bitoder0 () , self . cvsden0 () , self . aulaw_code0 () , self . aulawen0 () , self . sample_type0 () , self . linear_format0 ())
    }
}
#[doc = "AUDIOCNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiocntl1(pub u32);
impl Audiocntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn cvsd_bitoder1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cvsd_bitoder1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cvsden1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cvsden1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn aulaw_code1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_aulaw_code1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn aulawen1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_aulawen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sample_type1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sample_type1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn linear_format1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_linear_format1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
}
impl Default for Audiocntl1 {
    #[inline(always)]
    fn default() -> Audiocntl1 {
        Audiocntl1(0)
    }
}
impl core::fmt::Debug for Audiocntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiocntl1")
            .field("cvsd_bitoder1", &self.cvsd_bitoder1())
            .field("cvsden1", &self.cvsden1())
            .field("aulaw_code1", &self.aulaw_code1())
            .field("aulawen1", &self.aulawen1())
            .field("sample_type1", &self.sample_type1())
            .field("linear_format1", &self.linear_format1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiocntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Audiocntl1 {{ cvsd_bitoder1: {=bool:?}, cvsden1: {=bool:?}, aulaw_code1: {=u8:?}, aulawen1: {=bool:?}, sample_type1: {=u8:?}, linear_format1: {=u8:?} }}" , self . cvsd_bitoder1 () , self . cvsden1 () , self . aulaw_code1 () , self . aulawen1 () , self . sample_type1 () , self . linear_format1 ())
    }
}
#[doc = "AUDIOCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiocntl2(pub u32);
impl Audiocntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn cvsd_bitoder2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cvsd_bitoder2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cvsden2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cvsden2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn aulaw_code2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_aulaw_code2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn aulawen2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_aulawen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sample_type2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sample_type2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn linear_format2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_linear_format2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
}
impl Default for Audiocntl2 {
    #[inline(always)]
    fn default() -> Audiocntl2 {
        Audiocntl2(0)
    }
}
impl core::fmt::Debug for Audiocntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiocntl2")
            .field("cvsd_bitoder2", &self.cvsd_bitoder2())
            .field("cvsden2", &self.cvsden2())
            .field("aulaw_code2", &self.aulaw_code2())
            .field("aulawen2", &self.aulawen2())
            .field("sample_type2", &self.sample_type2())
            .field("linear_format2", &self.linear_format2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiocntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Audiocntl2 {{ cvsd_bitoder2: {=bool:?}, cvsden2: {=bool:?}, aulaw_code2: {=u8:?}, aulawen2: {=bool:?}, sample_type2: {=u8:?}, linear_format2: {=u8:?} }}" , self . cvsd_bitoder2 () , self . cvsden2 () , self . aulaw_code2 () , self . aulawen2 () , self . sample_type2 () , self . linear_format2 ())
    }
}
#[doc = "BLECOEXIFCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blecoexifcntl0(pub u32);
impl Blecoexifcntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn wlancoex_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_wlancoex_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn syncgen_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_syncgen_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwscoex_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwscoex_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwswci_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwswci_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlanrxmsk(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlanrxmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlantxmsk(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlantxmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwsrxmsk(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwsrxmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwstxmsk(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwstxmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwsrxfreqmsk(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwsrxfreqmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwstxfreqmsk(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwstxfreqmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlctxpriomode(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlctxpriomode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlcrxpriomode(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcrxpriomode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwsscanfreqmsk(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwsscanfreqmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
}
impl Default for Blecoexifcntl0 {
    #[inline(always)]
    fn default() -> Blecoexifcntl0 {
        Blecoexifcntl0(0)
    }
}
impl core::fmt::Debug for Blecoexifcntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blecoexifcntl0")
            .field("wlancoex_en", &self.wlancoex_en())
            .field("syncgen_en", &self.syncgen_en())
            .field("mwscoex_en", &self.mwscoex_en())
            .field("mwswci_en", &self.mwswci_en())
            .field("wlanrxmsk", &self.wlanrxmsk())
            .field("wlantxmsk", &self.wlantxmsk())
            .field("mwsrxmsk", &self.mwsrxmsk())
            .field("mwstxmsk", &self.mwstxmsk())
            .field("mwsrxfreqmsk", &self.mwsrxfreqmsk())
            .field("mwstxfreqmsk", &self.mwstxfreqmsk())
            .field("wlctxpriomode", &self.wlctxpriomode())
            .field("wlcrxpriomode", &self.wlcrxpriomode())
            .field("mwsscanfreqmsk", &self.mwsscanfreqmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blecoexifcntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blecoexifcntl0 {{ wlancoex_en: {=bool:?}, syncgen_en: {=bool:?}, mwscoex_en: {=bool:?}, mwswci_en: {=bool:?}, wlanrxmsk: {=u8:?}, wlantxmsk: {=u8:?}, mwsrxmsk: {=u8:?}, mwstxmsk: {=u8:?}, mwsrxfreqmsk: {=u8:?}, mwstxfreqmsk: {=u8:?}, wlctxpriomode: {=u8:?}, wlcrxpriomode: {=u8:?}, mwsscanfreqmsk: {=u8:?} }}" , self . wlancoex_en () , self . syncgen_en () , self . mwscoex_en () , self . mwswci_en () , self . wlanrxmsk () , self . wlantxmsk () , self . mwsrxmsk () , self . mwstxmsk () , self . mwsrxfreqmsk () , self . mwstxfreqmsk () , self . wlctxpriomode () , self . wlcrxpriomode () , self . mwsscanfreqmsk ())
    }
}
#[doc = "BLECOEXIFCNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blecoexifcntl1(pub u32);
impl Blecoexifcntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn wlcpdelay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcpdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlcpduration(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcpduration(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlcptxthr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcptxthr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlcprxthr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcprxthr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Blecoexifcntl1 {
    #[inline(always)]
    fn default() -> Blecoexifcntl1 {
        Blecoexifcntl1(0)
    }
}
impl core::fmt::Debug for Blecoexifcntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blecoexifcntl1")
            .field("wlcpdelay", &self.wlcpdelay())
            .field("wlcpduration", &self.wlcpduration())
            .field("wlcptxthr", &self.wlcptxthr())
            .field("wlcprxthr", &self.wlcprxthr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blecoexifcntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blecoexifcntl1 {{ wlcpdelay: {=u8:?}, wlcpduration: {=u8:?}, wlcptxthr: {=u8:?}, wlcprxthr: {=u8:?} }}" , self . wlcpdelay () , self . wlcpduration () , self . wlcptxthr () , self . wlcprxthr ())
    }
}
#[doc = "BLECOEXIFCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blecoexifcntl2(pub u32);
impl Blecoexifcntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_ant_delay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_ant_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_ant_delay(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_ant_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Blecoexifcntl2 {
    #[inline(always)]
    fn default() -> Blecoexifcntl2 {
        Blecoexifcntl2(0)
    }
}
impl core::fmt::Debug for Blecoexifcntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blecoexifcntl2")
            .field("tx_ant_delay", &self.tx_ant_delay())
            .field("rx_ant_delay", &self.rx_ant_delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blecoexifcntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Blecoexifcntl2 {{ tx_ant_delay: {=u8:?}, rx_ant_delay: {=u8:?} }}",
            self.tx_ant_delay(),
            self.rx_ant_delay()
        )
    }
}
#[doc = "BLECURRENTRXDESCPTR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blecurrentrxdescptr(pub u32);
impl Blecurrentrxdescptr {
    #[must_use]
    #[inline(always)]
    pub const fn currentrxdescptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_currentrxdescptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Blecurrentrxdescptr {
    #[inline(always)]
    fn default() -> Blecurrentrxdescptr {
        Blecurrentrxdescptr(0)
    }
}
impl core::fmt::Debug for Blecurrentrxdescptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blecurrentrxdescptr")
            .field("currentrxdescptr", &self.currentrxdescptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blecurrentrxdescptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Blecurrentrxdescptr {{ currentrxdescptr: {=u16:?} }}",
            self.currentrxdescptr()
        )
    }
}
#[doc = "BLEDEBUGADDMAX"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bledebugaddmax(pub u32);
impl Bledebugaddmax {
    #[must_use]
    #[inline(always)]
    pub const fn em_addmax(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_em_addmax(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reg_addmax(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_reg_addmax(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Bledebugaddmax {
    #[inline(always)]
    fn default() -> Bledebugaddmax {
        Bledebugaddmax(0)
    }
}
impl core::fmt::Debug for Bledebugaddmax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bledebugaddmax")
            .field("em_addmax", &self.em_addmax())
            .field("reg_addmax", &self.reg_addmax())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bledebugaddmax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bledebugaddmax {{ em_addmax: {=u16:?}, reg_addmax: {=u16:?} }}",
            self.em_addmax(),
            self.reg_addmax()
        )
    }
}
#[doc = "BLEDEBUGADDMIN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bledebugaddmin(pub u32);
impl Bledebugaddmin {
    #[must_use]
    #[inline(always)]
    pub const fn em_addmin(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_em_addmin(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reg_addmin(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_reg_addmin(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Bledebugaddmin {
    #[inline(always)]
    fn default() -> Bledebugaddmin {
        Bledebugaddmin(0)
    }
}
impl core::fmt::Debug for Bledebugaddmin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bledebugaddmin")
            .field("em_addmin", &self.em_addmin())
            .field("reg_addmin", &self.reg_addmin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bledebugaddmin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bledebugaddmin {{ em_addmin: {=u16:?}, reg_addmin: {=u16:?} }}",
            self.em_addmin(),
            self.reg_addmin()
        )
    }
}
#[doc = "BLEDIAGCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blediagcntl(pub u32);
impl Blediagcntl {
    #[must_use]
    #[inline(always)]
    pub const fn diag0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag0_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Blediagcntl {
    #[inline(always)]
    fn default() -> Blediagcntl {
        Blediagcntl(0)
    }
}
impl core::fmt::Debug for Blediagcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blediagcntl")
            .field("diag0", &self.diag0())
            .field("diag0_en", &self.diag0_en())
            .field("diag1", &self.diag1())
            .field("diag1_en", &self.diag1_en())
            .field("diag2", &self.diag2())
            .field("diag2_en", &self.diag2_en())
            .field("diag3", &self.diag3())
            .field("diag3_en", &self.diag3_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blediagcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blediagcntl {{ diag0: {=u8:?}, diag0_en: {=bool:?}, diag1: {=u8:?}, diag1_en: {=bool:?}, diag2: {=u8:?}, diag2_en: {=bool:?}, diag3: {=u8:?}, diag3_en: {=bool:?} }}" , self . diag0 () , self . diag0_en () , self . diag1 () , self . diag1_en () , self . diag2 () , self . diag2_en () , self . diag3 () , self . diag3_en ())
    }
}
#[doc = "BLEDIAGSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blediagstat(pub u32);
impl Blediagstat {
    #[must_use]
    #[inline(always)]
    pub const fn diag0stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag0stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1stat(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag1stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2stat(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag2stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3stat(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag3stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Blediagstat {
    #[inline(always)]
    fn default() -> Blediagstat {
        Blediagstat(0)
    }
}
impl core::fmt::Debug for Blediagstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blediagstat")
            .field("diag0stat", &self.diag0stat())
            .field("diag1stat", &self.diag1stat())
            .field("diag2stat", &self.diag2stat())
            .field("diag3stat", &self.diag3stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blediagstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blediagstat {{ diag0stat: {=u8:?}, diag1stat: {=u8:?}, diag2stat: {=u8:?}, diag3stat: {=u8:?} }}" , self . diag0stat () , self . diag1stat () , self . diag2stat () , self . diag3stat ())
    }
}
#[doc = "BLEERRORTYPESTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleerrortypestat(pub u32);
impl Bleerrortypestat {
    #[must_use]
    #[inline(always)]
    pub const fn txcrypt_error(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txcrypt_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxcrypt_error(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxcrypt_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pktcntl_emacc_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pktcntl_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn radio_emacc_error(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_radio_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn act_schdl_entry_error(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_act_schdl_entry_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn act_schdl_apfm_error(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_act_schdl_apfm_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn evt_cntl_apfm_error(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_evt_cntl_apfm_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn list_error(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_list_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ifs_underrun(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ifs_underrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adv_underrun(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adv_underrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn llchmap_error(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_llchmap_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn csformat_error(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_csformat_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txdesc_empty_error(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txdesc_empty_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxdesc_empty_error(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxdesc_empty_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txdata_ptr_error(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txdata_ptr_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxdata_ptr_error(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxdata_ptr_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ral_error(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ral_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ral_underrun(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ral_underrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tmafs_underrun(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tmafs_underrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txaeheader_ptr_error(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txaeheader_ptr_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn phy_error(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_phy_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn fifointovf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_fifointovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dfcntl_emacc_error(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dfcntl_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn freqsel_error(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_freqsel_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Bleerrortypestat {
    #[inline(always)]
    fn default() -> Bleerrortypestat {
        Bleerrortypestat(0)
    }
}
impl core::fmt::Debug for Bleerrortypestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleerrortypestat")
            .field("txcrypt_error", &self.txcrypt_error())
            .field("rxcrypt_error", &self.rxcrypt_error())
            .field("pktcntl_emacc_error", &self.pktcntl_emacc_error())
            .field("radio_emacc_error", &self.radio_emacc_error())
            .field("act_schdl_entry_error", &self.act_schdl_entry_error())
            .field("act_schdl_apfm_error", &self.act_schdl_apfm_error())
            .field("evt_cntl_apfm_error", &self.evt_cntl_apfm_error())
            .field("list_error", &self.list_error())
            .field("ifs_underrun", &self.ifs_underrun())
            .field("adv_underrun", &self.adv_underrun())
            .field("llchmap_error", &self.llchmap_error())
            .field("csformat_error", &self.csformat_error())
            .field("txdesc_empty_error", &self.txdesc_empty_error())
            .field("rxdesc_empty_error", &self.rxdesc_empty_error())
            .field("txdata_ptr_error", &self.txdata_ptr_error())
            .field("rxdata_ptr_error", &self.rxdata_ptr_error())
            .field("ral_error", &self.ral_error())
            .field("ral_underrun", &self.ral_underrun())
            .field("tmafs_underrun", &self.tmafs_underrun())
            .field("txaeheader_ptr_error", &self.txaeheader_ptr_error())
            .field("phy_error", &self.phy_error())
            .field("fifointovf", &self.fifointovf())
            .field("dfcntl_emacc_error", &self.dfcntl_emacc_error())
            .field("freqsel_error", &self.freqsel_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleerrortypestat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bleerrortypestat {{ txcrypt_error: {=bool:?}, rxcrypt_error: {=bool:?}, pktcntl_emacc_error: {=bool:?}, radio_emacc_error: {=bool:?}, act_schdl_entry_error: {=bool:?}, act_schdl_apfm_error: {=bool:?}, evt_cntl_apfm_error: {=bool:?}, list_error: {=bool:?}, ifs_underrun: {=bool:?}, adv_underrun: {=bool:?}, llchmap_error: {=bool:?}, csformat_error: {=bool:?}, txdesc_empty_error: {=bool:?}, rxdesc_empty_error: {=bool:?}, txdata_ptr_error: {=bool:?}, rxdata_ptr_error: {=bool:?}, ral_error: {=bool:?}, ral_underrun: {=bool:?}, tmafs_underrun: {=bool:?}, txaeheader_ptr_error: {=bool:?}, phy_error: {=bool:?}, fifointovf: {=bool:?}, dfcntl_emacc_error: {=bool:?}, freqsel_error: {=bool:?} }}" , self . txcrypt_error () , self . rxcrypt_error () , self . pktcntl_emacc_error () , self . radio_emacc_error () , self . act_schdl_entry_error () , self . act_schdl_apfm_error () , self . evt_cntl_apfm_error () , self . list_error () , self . ifs_underrun () , self . adv_underrun () , self . llchmap_error () , self . csformat_error () , self . txdesc_empty_error () , self . rxdesc_empty_error () , self . txdata_ptr_error () , self . rxdata_ptr_error () , self . ral_error () , self . ral_underrun () , self . tmafs_underrun () , self . txaeheader_ptr_error () , self . phy_error () , self . fifointovf () , self . dfcntl_emacc_error () , self . freqsel_error ())
    }
}
#[doc = "BLEINTACK0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleintack0(pub u32);
impl Bleintack0 {
    #[must_use]
    #[inline(always)]
    pub const fn hopintack(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hopintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn errorintack(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Bleintack0 {
    #[inline(always)]
    fn default() -> Bleintack0 {
        Bleintack0(0)
    }
}
impl core::fmt::Debug for Bleintack0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleintack0")
            .field("hopintack", &self.hopintack())
            .field("errorintack", &self.errorintack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleintack0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bleintack0 {{ hopintack: {=bool:?}, errorintack: {=bool:?} }}",
            self.hopintack(),
            self.errorintack()
        )
    }
}
#[doc = "BLEINTCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleintcntl0(pub u32);
impl Bleintcntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn startevtintmsk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_startevtintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn endevtintmsk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_endevtintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn skipevtintmsk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_skipevtintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txintmsk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxintmsk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn isotxintmsk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isotxintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn isorxintmsk(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isorxintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hopintmsk(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hopintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn errorintmsk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Bleintcntl0 {
    #[inline(always)]
    fn default() -> Bleintcntl0 {
        Bleintcntl0(0)
    }
}
impl core::fmt::Debug for Bleintcntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleintcntl0")
            .field("startevtintmsk", &self.startevtintmsk())
            .field("endevtintmsk", &self.endevtintmsk())
            .field("skipevtintmsk", &self.skipevtintmsk())
            .field("txintmsk", &self.txintmsk())
            .field("rxintmsk", &self.rxintmsk())
            .field("isotxintmsk", &self.isotxintmsk())
            .field("isorxintmsk", &self.isorxintmsk())
            .field("hopintmsk", &self.hopintmsk())
            .field("errorintmsk", &self.errorintmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleintcntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bleintcntl0 {{ startevtintmsk: {=bool:?}, endevtintmsk: {=bool:?}, skipevtintmsk: {=bool:?}, txintmsk: {=bool:?}, rxintmsk: {=bool:?}, isotxintmsk: {=bool:?}, isorxintmsk: {=bool:?}, hopintmsk: {=bool:?}, errorintmsk: {=bool:?} }}" , self . startevtintmsk () , self . endevtintmsk () , self . skipevtintmsk () , self . txintmsk () , self . rxintmsk () , self . isotxintmsk () , self . isorxintmsk () , self . hopintmsk () , self . errorintmsk ())
    }
}
#[doc = "BLEINTSTAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleintstat0(pub u32);
impl Bleintstat0 {
    #[must_use]
    #[inline(always)]
    pub const fn hopintstat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hopintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn errorintstat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Bleintstat0 {
    #[inline(always)]
    fn default() -> Bleintstat0 {
        Bleintstat0(0)
    }
}
impl core::fmt::Debug for Bleintstat0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleintstat0")
            .field("hopintstat", &self.hopintstat())
            .field("errorintstat", &self.errorintstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleintstat0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bleintstat0 {{ hopintstat: {=bool:?}, errorintstat: {=bool:?} }}",
            self.hopintstat(),
            self.errorintstat()
        )
    }
}
#[doc = "BLEMPRIO0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blemprio0(pub u32);
impl Blemprio0 {
    #[must_use]
    #[inline(always)]
    pub const fn blem0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Blemprio0 {
    #[inline(always)]
    fn default() -> Blemprio0 {
        Blemprio0(0)
    }
}
impl core::fmt::Debug for Blemprio0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blemprio0")
            .field("blem0", &self.blem0())
            .field("blem1", &self.blem1())
            .field("blem2", &self.blem2())
            .field("blem3", &self.blem3())
            .field("blem4", &self.blem4())
            .field("blem5", &self.blem5())
            .field("blem6", &self.blem6())
            .field("blem7", &self.blem7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blemprio0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blemprio0 {{ blem0: {=u8:?}, blem1: {=u8:?}, blem2: {=u8:?}, blem3: {=u8:?}, blem4: {=u8:?}, blem5: {=u8:?}, blem6: {=u8:?}, blem7: {=u8:?} }}" , self . blem0 () , self . blem1 () , self . blem2 () , self . blem3 () , self . blem4 () , self . blem5 () , self . blem6 () , self . blem7 ())
    }
}
#[doc = "BLEMPRIO1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blemprio1(pub u32);
impl Blemprio1 {
    #[must_use]
    #[inline(always)]
    pub const fn blem8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Blemprio1 {
    #[inline(always)]
    fn default() -> Blemprio1 {
        Blemprio1(0)
    }
}
impl core::fmt::Debug for Blemprio1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blemprio1")
            .field("blem8", &self.blem8())
            .field("blem9", &self.blem9())
            .field("blem10", &self.blem10())
            .field("blem11", &self.blem11())
            .field("blem12", &self.blem12())
            .field("blem13", &self.blem13())
            .field("blem14", &self.blem14())
            .field("blem15", &self.blem15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blemprio1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blemprio1 {{ blem8: {=u8:?}, blem9: {=u8:?}, blem10: {=u8:?}, blem11: {=u8:?}, blem12: {=u8:?}, blem13: {=u8:?}, blem14: {=u8:?}, blem15: {=u8:?} }}" , self . blem8 () , self . blem9 () , self . blem10 () , self . blem11 () , self . blem12 () , self . blem13 () , self . blem14 () , self . blem15 ())
    }
}
#[doc = "BLEMPRIO2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blemprio2(pub u32);
impl Blemprio2 {
    #[must_use]
    #[inline(always)]
    pub const fn blem16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem17(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blem18(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blem18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blemdefault(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_blemdefault(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Blemprio2 {
    #[inline(always)]
    fn default() -> Blemprio2 {
        Blemprio2(0)
    }
}
impl core::fmt::Debug for Blemprio2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blemprio2")
            .field("blem16", &self.blem16())
            .field("blem17", &self.blem17())
            .field("blem18", &self.blem18())
            .field("blemdefault", &self.blemdefault())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blemprio2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blemprio2 {{ blem16: {=u8:?}, blem17: {=u8:?}, blem18: {=u8:?}, blemdefault: {=u8:?} }}" , self . blem16 () , self . blem17 () , self . blem18 () , self . blemdefault ())
    }
}
#[doc = "BLERADIOCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiocntl2(pub u32);
impl Bleradiocntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn freqtable_ptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_freqtable_ptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn phymsk(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_phymsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
}
impl Default for Bleradiocntl2 {
    #[inline(always)]
    fn default() -> Bleradiocntl2 {
        Bleradiocntl2(0)
    }
}
impl core::fmt::Debug for Bleradiocntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiocntl2")
            .field("freqtable_ptr", &self.freqtable_ptr())
            .field("phymsk", &self.phymsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiocntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bleradiocntl2 {{ freqtable_ptr: {=u16:?}, phymsk: {=u8:?} }}",
            self.freqtable_ptr(),
            self.phymsk()
        )
    }
}
#[doc = "BLERADIOCNTL3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiocntl3(pub u32);
impl Bleradiocntl3 {
    #[must_use]
    #[inline(always)]
    pub const fn txrate0cfg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txrate0cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txrate1cfg(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txrate1cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txrate2cfg(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txrate2cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txrate3cfg(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txrate3cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxrate0cfg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxrate0cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxrate1cfg(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxrate1cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxrate2cfg(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxrate2cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxrate3cfg(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxrate3cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Bleradiocntl3 {
    #[inline(always)]
    fn default() -> Bleradiocntl3 {
        Bleradiocntl3(0)
    }
}
impl core::fmt::Debug for Bleradiocntl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiocntl3")
            .field("txrate0cfg", &self.txrate0cfg())
            .field("txrate1cfg", &self.txrate1cfg())
            .field("txrate2cfg", &self.txrate2cfg())
            .field("txrate3cfg", &self.txrate3cfg())
            .field("rxrate0cfg", &self.rxrate0cfg())
            .field("rxrate1cfg", &self.rxrate1cfg())
            .field("rxrate2cfg", &self.rxrate2cfg())
            .field("rxrate3cfg", &self.rxrate3cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiocntl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bleradiocntl3 {{ txrate0cfg: {=u8:?}, txrate1cfg: {=u8:?}, txrate2cfg: {=u8:?}, txrate3cfg: {=u8:?}, rxrate0cfg: {=u8:?}, rxrate1cfg: {=u8:?}, rxrate2cfg: {=u8:?}, rxrate3cfg: {=u8:?} }}" , self . txrate0cfg () , self . txrate1cfg () , self . txrate2cfg () , self . txrate3cfg () , self . rxrate0cfg () , self . rxrate1cfg () , self . rxrate2cfg () , self . rxrate3cfg ())
    }
}
#[doc = "BLERADIOPWRUPDN0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiopwrupdn0(pub u32);
impl Bleradiopwrupdn0 {
    #[must_use]
    #[inline(always)]
    pub const fn txpwrup0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrup0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txpwrdn0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrdn0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpwrup0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxpwrup0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sync_position0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sync_position0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Bleradiopwrupdn0 {
    #[inline(always)]
    fn default() -> Bleradiopwrupdn0 {
        Bleradiopwrupdn0(0)
    }
}
impl core::fmt::Debug for Bleradiopwrupdn0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiopwrupdn0")
            .field("txpwrup0", &self.txpwrup0())
            .field("txpwrdn0", &self.txpwrdn0())
            .field("rxpwrup0", &self.rxpwrup0())
            .field("sync_position0", &self.sync_position0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiopwrupdn0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bleradiopwrupdn0 {{ txpwrup0: {=u8:?}, txpwrdn0: {=u8:?}, rxpwrup0: {=u8:?}, sync_position0: {=u8:?} }}" , self . txpwrup0 () , self . txpwrdn0 () , self . rxpwrup0 () , self . sync_position0 ())
    }
}
#[doc = "BLERADIOPWRUPDN1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiopwrupdn1(pub u32);
impl Bleradiopwrupdn1 {
    #[must_use]
    #[inline(always)]
    pub const fn txpwrup1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrup1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txpwrdn1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrdn1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpwrup1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxpwrup1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sync_position1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sync_position1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Bleradiopwrupdn1 {
    #[inline(always)]
    fn default() -> Bleradiopwrupdn1 {
        Bleradiopwrupdn1(0)
    }
}
impl core::fmt::Debug for Bleradiopwrupdn1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiopwrupdn1")
            .field("txpwrup1", &self.txpwrup1())
            .field("txpwrdn1", &self.txpwrdn1())
            .field("rxpwrup1", &self.rxpwrup1())
            .field("sync_position1", &self.sync_position1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiopwrupdn1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bleradiopwrupdn1 {{ txpwrup1: {=u8:?}, txpwrdn1: {=u8:?}, rxpwrup1: {=u8:?}, sync_position1: {=u8:?} }}" , self . txpwrup1 () , self . txpwrdn1 () , self . rxpwrup1 () , self . sync_position1 ())
    }
}
#[doc = "BLERADIOPWRUPDN2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiopwrupdn2(pub u32);
impl Bleradiopwrupdn2 {
    #[must_use]
    #[inline(always)]
    pub const fn txpwrup2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrup2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txpwrdn2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrdn2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpwrup2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxpwrup2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sync_position2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sync_position2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Bleradiopwrupdn2 {
    #[inline(always)]
    fn default() -> Bleradiopwrupdn2 {
        Bleradiopwrupdn2(0)
    }
}
impl core::fmt::Debug for Bleradiopwrupdn2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiopwrupdn2")
            .field("txpwrup2", &self.txpwrup2())
            .field("txpwrdn2", &self.txpwrdn2())
            .field("rxpwrup2", &self.rxpwrup2())
            .field("sync_position2", &self.sync_position2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiopwrupdn2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bleradiopwrupdn2 {{ txpwrup2: {=u8:?}, txpwrdn2: {=u8:?}, rxpwrup2: {=u8:?}, sync_position2: {=u8:?} }}" , self . txpwrup2 () , self . txpwrdn2 () , self . rxpwrup2 () , self . sync_position2 ())
    }
}
#[doc = "BLERADIOPWRUPDN3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiopwrupdn3(pub u32);
impl Bleradiopwrupdn3 {
    #[must_use]
    #[inline(always)]
    pub const fn txpwrup3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrup3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txpwrdn3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrdn3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for Bleradiopwrupdn3 {
    #[inline(always)]
    fn default() -> Bleradiopwrupdn3 {
        Bleradiopwrupdn3(0)
    }
}
impl core::fmt::Debug for Bleradiopwrupdn3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiopwrupdn3")
            .field("txpwrup3", &self.txpwrup3())
            .field("txpwrdn3", &self.txpwrdn3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiopwrupdn3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bleradiopwrupdn3 {{ txpwrup3: {=u8:?}, txpwrdn3: {=u8:?} }}",
            self.txpwrup3(),
            self.txpwrdn3()
        )
    }
}
#[doc = "BLERADIOTXRXTIM0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiotxrxtim0(pub u32);
impl Bleradiotxrxtim0 {
    #[must_use]
    #[inline(always)]
    pub const fn txpathdly0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpathdly0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpathdly0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxpathdly0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rfrxtmda0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rfrxtmda0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Bleradiotxrxtim0 {
    #[inline(always)]
    fn default() -> Bleradiotxrxtim0 {
        Bleradiotxrxtim0(0)
    }
}
impl core::fmt::Debug for Bleradiotxrxtim0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiotxrxtim0")
            .field("txpathdly0", &self.txpathdly0())
            .field("rxpathdly0", &self.rxpathdly0())
            .field("rfrxtmda0", &self.rfrxtmda0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiotxrxtim0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bleradiotxrxtim0 {{ txpathdly0: {=u8:?}, rxpathdly0: {=u8:?}, rfrxtmda0: {=u8:?} }}",
            self.txpathdly0(),
            self.rxpathdly0(),
            self.rfrxtmda0()
        )
    }
}
#[doc = "BLERADIOTXRXTIM1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiotxrxtim1(pub u32);
impl Bleradiotxrxtim1 {
    #[must_use]
    #[inline(always)]
    pub const fn txpathdly1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpathdly1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpathdly1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxpathdly1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rfrxtmda1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rfrxtmda1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Bleradiotxrxtim1 {
    #[inline(always)]
    fn default() -> Bleradiotxrxtim1 {
        Bleradiotxrxtim1(0)
    }
}
impl core::fmt::Debug for Bleradiotxrxtim1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiotxrxtim1")
            .field("txpathdly1", &self.txpathdly1())
            .field("rxpathdly1", &self.rxpathdly1())
            .field("rfrxtmda1", &self.rfrxtmda1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiotxrxtim1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bleradiotxrxtim1 {{ txpathdly1: {=u8:?}, rxpathdly1: {=u8:?}, rfrxtmda1: {=u8:?} }}",
            self.txpathdly1(),
            self.rxpathdly1(),
            self.rfrxtmda1()
        )
    }
}
#[doc = "BLERADIOTXRXTIM2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiotxrxtim2(pub u32);
impl Bleradiotxrxtim2 {
    #[must_use]
    #[inline(always)]
    pub const fn txpathdly2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpathdly2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpathdly2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxpathdly2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rfrxtmda2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rfrxtmda2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxflushpathdly2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxflushpathdly2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Bleradiotxrxtim2 {
    #[inline(always)]
    fn default() -> Bleradiotxrxtim2 {
        Bleradiotxrxtim2(0)
    }
}
impl core::fmt::Debug for Bleradiotxrxtim2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiotxrxtim2")
            .field("txpathdly2", &self.txpathdly2())
            .field("rxpathdly2", &self.rxpathdly2())
            .field("rfrxtmda2", &self.rfrxtmda2())
            .field("rxflushpathdly2", &self.rxflushpathdly2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiotxrxtim2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bleradiotxrxtim2 {{ txpathdly2: {=u8:?}, rxpathdly2: {=u8:?}, rfrxtmda2: {=u8:?}, rxflushpathdly2: {=u8:?} }}" , self . txpathdly2 () , self . rxpathdly2 () , self . rfrxtmda2 () , self . rxflushpathdly2 ())
    }
}
#[doc = "BLERADIOTXRXTIM3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleradiotxrxtim3(pub u32);
impl Bleradiotxrxtim3 {
    #[must_use]
    #[inline(always)]
    pub const fn txpathdly3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpathdly3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rfrxtmda3(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rfrxtmda3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxflushpathdly3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxflushpathdly3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Bleradiotxrxtim3 {
    #[inline(always)]
    fn default() -> Bleradiotxrxtim3 {
        Bleradiotxrxtim3(0)
    }
}
impl core::fmt::Debug for Bleradiotxrxtim3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleradiotxrxtim3")
            .field("txpathdly3", &self.txpathdly3())
            .field("rfrxtmda3", &self.rfrxtmda3())
            .field("rxflushpathdly3", &self.rxflushpathdly3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleradiotxrxtim3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bleradiotxrxtim3 {{ txpathdly3: {=u8:?}, rfrxtmda3: {=u8:?}, rxflushpathdly3: {=u8:?} }}" , self . txpathdly3 () , self . rfrxtmda3 () , self . rxflushpathdly3 ())
    }
}
#[doc = "BLERFTESTCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blerftestcntl(pub u32);
impl Blerftestcntl {
    #[must_use]
    #[inline(always)]
    pub const fn txlength(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txpktcnten(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txpktcnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txpldsrc(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txpldsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn prbstype(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_prbstype(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txlengthsrc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txlengthsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn infinitetx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_infinitetx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn percount_mode(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_percount_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpktcnten(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxpktcnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn infiniterx(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_infiniterx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Blerftestcntl {
    #[inline(always)]
    fn default() -> Blerftestcntl {
        Blerftestcntl(0)
    }
}
impl core::fmt::Debug for Blerftestcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blerftestcntl")
            .field("txlength", &self.txlength())
            .field("txpktcnten", &self.txpktcnten())
            .field("txpldsrc", &self.txpldsrc())
            .field("prbstype", &self.prbstype())
            .field("txlengthsrc", &self.txlengthsrc())
            .field("infinitetx", &self.infinitetx())
            .field("percount_mode", &self.percount_mode())
            .field("rxpktcnten", &self.rxpktcnten())
            .field("infiniterx", &self.infiniterx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blerftestcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blerftestcntl {{ txlength: {=u8:?}, txpktcnten: {=bool:?}, txpldsrc: {=bool:?}, prbstype: {=bool:?}, txlengthsrc: {=bool:?}, infinitetx: {=bool:?}, percount_mode: {=u8:?}, rxpktcnten: {=bool:?}, infiniterx: {=bool:?} }}" , self . txlength () , self . txpktcnten () , self . txpldsrc () , self . prbstype () , self . txlengthsrc () , self . infinitetx () , self . percount_mode () , self . rxpktcnten () , self . infiniterx ())
    }
}
#[doc = "BLERFTESTRXSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blerftestrxstat(pub u32);
impl Blerftestrxstat {
    #[must_use]
    #[inline(always)]
    pub const fn rxpktcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_rxpktcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Blerftestrxstat {
    #[inline(always)]
    fn default() -> Blerftestrxstat {
        Blerftestrxstat(0)
    }
}
impl core::fmt::Debug for Blerftestrxstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blerftestrxstat")
            .field("rxpktcnt", &self.rxpktcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blerftestrxstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Blerftestrxstat {{ rxpktcnt: {=u32:?} }}",
            self.rxpktcnt()
        )
    }
}
#[doc = "BLERFTESTTXSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blerftesttxstat(pub u32);
impl Blerftesttxstat {
    #[must_use]
    #[inline(always)]
    pub const fn txpktcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_txpktcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Blerftesttxstat {
    #[inline(always)]
    fn default() -> Blerftesttxstat {
        Blerftesttxstat(0)
    }
}
impl core::fmt::Debug for Blerftesttxstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blerftesttxstat")
            .field("txpktcnt", &self.txpktcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blerftesttxstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Blerftesttxstat {{ txpktcnt: {=u32:?} }}",
            self.txpktcnt()
        )
    }
}
#[doc = "BLESWPROFILING"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleswprofiling(pub u32);
impl Bleswprofiling {
    #[must_use]
    #[inline(always)]
    pub const fn swprof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_swprof(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bleswprofiling {
    #[inline(always)]
    fn default() -> Bleswprofiling {
        Bleswprofiling(0)
    }
}
impl core::fmt::Debug for Bleswprofiling {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleswprofiling")
            .field("swprof", &self.swprof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleswprofiling {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bleswprofiling {{ swprof: {=u32:?} }}", self.swprof())
    }
}
#[doc = "BLEVERSION"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bleversion(pub u32);
impl Bleversion {
    #[must_use]
    #[inline(always)]
    pub const fn build_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_build_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn upg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_upg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn typ(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_typ(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Bleversion {
    #[inline(always)]
    fn default() -> Bleversion {
        Bleversion(0)
    }
}
impl core::fmt::Debug for Bleversion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bleversion")
            .field("build_num", &self.build_num())
            .field("upg", &self.upg())
            .field("rel", &self.rel())
            .field("typ", &self.typ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bleversion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bleversion {{ build_num: {=u8:?}, upg: {=u8:?}, rel: {=u8:?}, typ: {=u8:?} }}",
            self.build_num(),
            self.upg(),
            self.rel(),
            self.typ()
        )
    }
}
#[doc = "BTCOEXIFCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btcoexifcntl0(pub u32);
impl Btcoexifcntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn wlancoex_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_wlancoex_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn syncgen_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_syncgen_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwscoex_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwscoex_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwswci_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwswci_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlanrxmsk(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlanrxmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlantxmsk(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlantxmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwsrxmsk(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwsrxmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwstxmsk(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwstxmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwsrxfreqmsk(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwsrxfreqmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwstxfreqmsk(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwstxfreqmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlctxpriomode(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlctxpriomode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlcrxpriomode(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcrxpriomode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwsscanfreqmsk(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mwsscanfreqmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pageeknudgeinc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pageeknudgeinc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn inqknudgeinc(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_inqknudgeinc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Btcoexifcntl0 {
    #[inline(always)]
    fn default() -> Btcoexifcntl0 {
        Btcoexifcntl0(0)
    }
}
impl core::fmt::Debug for Btcoexifcntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btcoexifcntl0")
            .field("wlancoex_en", &self.wlancoex_en())
            .field("syncgen_en", &self.syncgen_en())
            .field("mwscoex_en", &self.mwscoex_en())
            .field("mwswci_en", &self.mwswci_en())
            .field("wlanrxmsk", &self.wlanrxmsk())
            .field("wlantxmsk", &self.wlantxmsk())
            .field("mwsrxmsk", &self.mwsrxmsk())
            .field("mwstxmsk", &self.mwstxmsk())
            .field("mwsrxfreqmsk", &self.mwsrxfreqmsk())
            .field("mwstxfreqmsk", &self.mwstxfreqmsk())
            .field("wlctxpriomode", &self.wlctxpriomode())
            .field("wlcrxpriomode", &self.wlcrxpriomode())
            .field("mwsscanfreqmsk", &self.mwsscanfreqmsk())
            .field("pageeknudgeinc", &self.pageeknudgeinc())
            .field("inqknudgeinc", &self.inqknudgeinc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btcoexifcntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btcoexifcntl0 {{ wlancoex_en: {=bool:?}, syncgen_en: {=bool:?}, mwscoex_en: {=bool:?}, mwswci_en: {=bool:?}, wlanrxmsk: {=u8:?}, wlantxmsk: {=u8:?}, mwsrxmsk: {=u8:?}, mwstxmsk: {=u8:?}, mwsrxfreqmsk: {=u8:?}, mwstxfreqmsk: {=u8:?}, wlctxpriomode: {=u8:?}, wlcrxpriomode: {=u8:?}, mwsscanfreqmsk: {=u8:?}, pageeknudgeinc: {=u8:?}, inqknudgeinc: {=u8:?} }}" , self . wlancoex_en () , self . syncgen_en () , self . mwscoex_en () , self . mwswci_en () , self . wlanrxmsk () , self . wlantxmsk () , self . mwsrxmsk () , self . mwstxmsk () , self . mwsrxfreqmsk () , self . mwstxfreqmsk () , self . wlctxpriomode () , self . wlcrxpriomode () , self . mwsscanfreqmsk () , self . pageeknudgeinc () , self . inqknudgeinc ())
    }
}
#[doc = "BTCOEXIFCNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btcoexifcntl1(pub u32);
impl Btcoexifcntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn wlcpdelay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcpdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlcpduration(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcpduration(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlcptxthr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcptxthr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wlcprxthr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wlcprxthr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Btcoexifcntl1 {
    #[inline(always)]
    fn default() -> Btcoexifcntl1 {
        Btcoexifcntl1(0)
    }
}
impl core::fmt::Debug for Btcoexifcntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btcoexifcntl1")
            .field("wlcpdelay", &self.wlcpdelay())
            .field("wlcpduration", &self.wlcpduration())
            .field("wlcptxthr", &self.wlcptxthr())
            .field("wlcprxthr", &self.wlcprxthr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btcoexifcntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btcoexifcntl1 {{ wlcpdelay: {=u8:?}, wlcpduration: {=u8:?}, wlcptxthr: {=u8:?}, wlcprxthr: {=u8:?} }}" , self . wlcpdelay () , self . wlcpduration () , self . wlcptxthr () , self . wlcprxthr ())
    }
}
#[doc = "BTCOEXIFCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btcoexifcntl2(pub u32);
impl Btcoexifcntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_ant_delay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_ant_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_ant_delay(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_ant_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_force_val(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pta_force_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_actsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pta_actsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_actmode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_actmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_actpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_actpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_priomode(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_priomode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_abortmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_abortmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_wlanpol(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_wlanpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_aborttx(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_aborttx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_abortrx(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_abortrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_masktx(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_masktx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_maskrx(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_maskrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_wlantx(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_wlantx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_wlanrx(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pta_wlanrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pta_force_en(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pta_force_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Btcoexifcntl2 {
    #[inline(always)]
    fn default() -> Btcoexifcntl2 {
        Btcoexifcntl2(0)
    }
}
impl core::fmt::Debug for Btcoexifcntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btcoexifcntl2")
            .field("tx_ant_delay", &self.tx_ant_delay())
            .field("rx_ant_delay", &self.rx_ant_delay())
            .field("pta_force_val", &self.pta_force_val())
            .field("pta_actsel", &self.pta_actsel())
            .field("pta_actmode", &self.pta_actmode())
            .field("pta_actpol", &self.pta_actpol())
            .field("pta_priomode", &self.pta_priomode())
            .field("pta_abortmode", &self.pta_abortmode())
            .field("pta_wlanpol", &self.pta_wlanpol())
            .field("pta_aborttx", &self.pta_aborttx())
            .field("pta_abortrx", &self.pta_abortrx())
            .field("pta_masktx", &self.pta_masktx())
            .field("pta_maskrx", &self.pta_maskrx())
            .field("pta_wlantx", &self.pta_wlantx())
            .field("pta_wlanrx", &self.pta_wlanrx())
            .field("pta_force_en", &self.pta_force_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btcoexifcntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btcoexifcntl2 {{ tx_ant_delay: {=u8:?}, rx_ant_delay: {=u8:?}, pta_force_val: {=u8:?}, pta_actsel: {=u8:?}, pta_actmode: {=bool:?}, pta_actpol: {=bool:?}, pta_priomode: {=bool:?}, pta_abortmode: {=bool:?}, pta_wlanpol: {=bool:?}, pta_aborttx: {=bool:?}, pta_abortrx: {=bool:?}, pta_masktx: {=bool:?}, pta_maskrx: {=bool:?}, pta_wlantx: {=bool:?}, pta_wlanrx: {=bool:?}, pta_force_en: {=u8:?} }}" , self . tx_ant_delay () , self . rx_ant_delay () , self . pta_force_val () , self . pta_actsel () , self . pta_actmode () , self . pta_actpol () , self . pta_priomode () , self . pta_abortmode () , self . pta_wlanpol () , self . pta_aborttx () , self . pta_abortrx () , self . pta_masktx () , self . pta_maskrx () , self . pta_wlantx () , self . pta_wlanrx () , self . pta_force_en ())
    }
}
#[doc = "BTCURRENTRXDESCPTR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btcurrentrxdescptr(pub u32);
impl Btcurrentrxdescptr {
    #[must_use]
    #[inline(always)]
    pub const fn currentrxdescptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_currentrxdescptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Btcurrentrxdescptr {
    #[inline(always)]
    fn default() -> Btcurrentrxdescptr {
        Btcurrentrxdescptr(0)
    }
}
impl core::fmt::Debug for Btcurrentrxdescptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btcurrentrxdescptr")
            .field("currentrxdescptr", &self.currentrxdescptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btcurrentrxdescptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btcurrentrxdescptr {{ currentrxdescptr: {=u16:?} }}",
            self.currentrxdescptr()
        )
    }
}
#[doc = "BTDEBUGADDMAX"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btdebugaddmax(pub u32);
impl Btdebugaddmax {
    #[must_use]
    #[inline(always)]
    pub const fn em_addmax(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_em_addmax(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reg_addmax(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_reg_addmax(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Btdebugaddmax {
    #[inline(always)]
    fn default() -> Btdebugaddmax {
        Btdebugaddmax(0)
    }
}
impl core::fmt::Debug for Btdebugaddmax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btdebugaddmax")
            .field("em_addmax", &self.em_addmax())
            .field("reg_addmax", &self.reg_addmax())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btdebugaddmax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btdebugaddmax {{ em_addmax: {=u16:?}, reg_addmax: {=u16:?} }}",
            self.em_addmax(),
            self.reg_addmax()
        )
    }
}
#[doc = "BTDEBUGADDMIN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btdebugaddmin(pub u32);
impl Btdebugaddmin {
    #[must_use]
    #[inline(always)]
    pub const fn em_addmin(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_em_addmin(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reg_addmin(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_reg_addmin(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Btdebugaddmin {
    #[inline(always)]
    fn default() -> Btdebugaddmin {
        Btdebugaddmin(0)
    }
}
impl core::fmt::Debug for Btdebugaddmin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btdebugaddmin")
            .field("em_addmin", &self.em_addmin())
            .field("reg_addmin", &self.reg_addmin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btdebugaddmin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btdebugaddmin {{ em_addmin: {=u16:?}, reg_addmin: {=u16:?} }}",
            self.em_addmin(),
            self.reg_addmin()
        )
    }
}
#[doc = "BTDIAGCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btdiagcntl(pub u32);
impl Btdiagcntl {
    #[must_use]
    #[inline(always)]
    pub const fn diag0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag0_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Btdiagcntl {
    #[inline(always)]
    fn default() -> Btdiagcntl {
        Btdiagcntl(0)
    }
}
impl core::fmt::Debug for Btdiagcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btdiagcntl")
            .field("diag0", &self.diag0())
            .field("diag0_en", &self.diag0_en())
            .field("diag1", &self.diag1())
            .field("diag1_en", &self.diag1_en())
            .field("diag2", &self.diag2())
            .field("diag2_en", &self.diag2_en())
            .field("diag3", &self.diag3())
            .field("diag3_en", &self.diag3_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btdiagcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btdiagcntl {{ diag0: {=u8:?}, diag0_en: {=bool:?}, diag1: {=u8:?}, diag1_en: {=bool:?}, diag2: {=u8:?}, diag2_en: {=bool:?}, diag3: {=u8:?}, diag3_en: {=bool:?} }}" , self . diag0 () , self . diag0_en () , self . diag1 () , self . diag1_en () , self . diag2 () , self . diag2_en () , self . diag3 () , self . diag3_en ())
    }
}
#[doc = "BTDIAGSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btdiagstat(pub u32);
impl Btdiagstat {
    #[must_use]
    #[inline(always)]
    pub const fn diag0stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag0stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1stat(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag1stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2stat(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag2stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3stat(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag3stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Btdiagstat {
    #[inline(always)]
    fn default() -> Btdiagstat {
        Btdiagstat(0)
    }
}
impl core::fmt::Debug for Btdiagstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btdiagstat")
            .field("diag0stat", &self.diag0stat())
            .field("diag1stat", &self.diag1stat())
            .field("diag2stat", &self.diag2stat())
            .field("diag3stat", &self.diag3stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btdiagstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btdiagstat {{ diag0stat: {=u8:?}, diag1stat: {=u8:?}, diag2stat: {=u8:?}, diag3stat: {=u8:?} }}" , self . diag0stat () , self . diag1stat () , self . diag2stat () , self . diag3stat ())
    }
}
#[doc = "BTERRORTYPESTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bterrortypestat(pub u32);
impl Bterrortypestat {
    #[must_use]
    #[inline(always)]
    pub const fn txcrypt_error(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txcrypt_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxcrypt_error(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxcrypt_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cryptmode_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cryptmode_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pktcntl_emacc_error(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pktcntl_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn radio_emacc_error(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_radio_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio_emacc_error(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pcm_emacc_error(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pcm_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwscoex_emacc_error(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwscoex_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn act_schdl_entry_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_act_schdl_entry_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn act_schdl_apfm_error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_act_schdl_apfm_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn frm_cntl_apfm_error(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frm_cntl_apfm_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn frm_cntl_emacc_error(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frm_cntl_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn frm_cntl_timer_error(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frm_cntl_timer_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hopunderrun_error(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hopunderrun_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn chmap_error(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_chmap_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn csformat_error(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_csformat_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn csattnb_error(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_csattnb_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txdesc_empty_error(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txdesc_empty_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxdesc_empty_error(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxdesc_empty_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txbuf_ptr_error(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txbuf_ptr_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxbuf_ptr_error(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxbuf_ptr_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn peer_sam_error(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_peer_sam_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn local_sam_error(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_local_sam_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn fifointovf(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_fifointovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Bterrortypestat {
    #[inline(always)]
    fn default() -> Bterrortypestat {
        Bterrortypestat(0)
    }
}
impl core::fmt::Debug for Bterrortypestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bterrortypestat")
            .field("txcrypt_error", &self.txcrypt_error())
            .field("rxcrypt_error", &self.rxcrypt_error())
            .field("cryptmode_error", &self.cryptmode_error())
            .field("pktcntl_emacc_error", &self.pktcntl_emacc_error())
            .field("radio_emacc_error", &self.radio_emacc_error())
            .field("audio_emacc_error", &self.audio_emacc_error())
            .field("pcm_emacc_error", &self.pcm_emacc_error())
            .field("mwscoex_emacc_error", &self.mwscoex_emacc_error())
            .field("act_schdl_entry_error", &self.act_schdl_entry_error())
            .field("act_schdl_apfm_error", &self.act_schdl_apfm_error())
            .field("frm_cntl_apfm_error", &self.frm_cntl_apfm_error())
            .field("frm_cntl_emacc_error", &self.frm_cntl_emacc_error())
            .field("frm_cntl_timer_error", &self.frm_cntl_timer_error())
            .field("hopunderrun_error", &self.hopunderrun_error())
            .field("chmap_error", &self.chmap_error())
            .field("csformat_error", &self.csformat_error())
            .field("csattnb_error", &self.csattnb_error())
            .field("txdesc_empty_error", &self.txdesc_empty_error())
            .field("rxdesc_empty_error", &self.rxdesc_empty_error())
            .field("txbuf_ptr_error", &self.txbuf_ptr_error())
            .field("rxbuf_ptr_error", &self.rxbuf_ptr_error())
            .field("peer_sam_error", &self.peer_sam_error())
            .field("local_sam_error", &self.local_sam_error())
            .field("fifointovf", &self.fifointovf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bterrortypestat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bterrortypestat {{ txcrypt_error: {=bool:?}, rxcrypt_error: {=bool:?}, cryptmode_error: {=bool:?}, pktcntl_emacc_error: {=bool:?}, radio_emacc_error: {=bool:?}, audio_emacc_error: {=bool:?}, pcm_emacc_error: {=bool:?}, mwscoex_emacc_error: {=bool:?}, act_schdl_entry_error: {=bool:?}, act_schdl_apfm_error: {=bool:?}, frm_cntl_apfm_error: {=bool:?}, frm_cntl_emacc_error: {=bool:?}, frm_cntl_timer_error: {=bool:?}, hopunderrun_error: {=bool:?}, chmap_error: {=bool:?}, csformat_error: {=bool:?}, csattnb_error: {=bool:?}, txdesc_empty_error: {=bool:?}, rxdesc_empty_error: {=bool:?}, txbuf_ptr_error: {=bool:?}, rxbuf_ptr_error: {=bool:?}, peer_sam_error: {=bool:?}, local_sam_error: {=bool:?}, fifointovf: {=bool:?} }}" , self . txcrypt_error () , self . rxcrypt_error () , self . cryptmode_error () , self . pktcntl_emacc_error () , self . radio_emacc_error () , self . audio_emacc_error () , self . pcm_emacc_error () , self . mwscoex_emacc_error () , self . act_schdl_entry_error () , self . act_schdl_apfm_error () , self . frm_cntl_apfm_error () , self . frm_cntl_emacc_error () , self . frm_cntl_timer_error () , self . hopunderrun_error () , self . chmap_error () , self . csformat_error () , self . csattnb_error () , self . txdesc_empty_error () , self . rxdesc_empty_error () , self . txbuf_ptr_error () , self . rxbuf_ptr_error () , self . peer_sam_error () , self . local_sam_error () , self . fifointovf ())
    }
}
#[doc = "BTINTACK0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btintack0(pub u32);
impl Btintack0 {
    #[must_use]
    #[inline(always)]
    pub const fn frsyncintack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frsyncintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mtoffint0ack(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mtoffint0ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mtoffint1ack(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mtoffint1ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwswcitxintack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwswcitxintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwswcirxintack(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwswcirxintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio0intack(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio0intack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio1intack(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio1intack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio2intack(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio2intack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn errorintack(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Btintack0 {
    #[inline(always)]
    fn default() -> Btintack0 {
        Btintack0(0)
    }
}
impl core::fmt::Debug for Btintack0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btintack0")
            .field("frsyncintack", &self.frsyncintack())
            .field("mtoffint0ack", &self.mtoffint0ack())
            .field("mtoffint1ack", &self.mtoffint1ack())
            .field("mwswcitxintack", &self.mwswcitxintack())
            .field("mwswcirxintack", &self.mwswcirxintack())
            .field("audio0intack", &self.audio0intack())
            .field("audio1intack", &self.audio1intack())
            .field("audio2intack", &self.audio2intack())
            .field("errorintack", &self.errorintack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btintack0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btintack0 {{ frsyncintack: {=bool:?}, mtoffint0ack: {=bool:?}, mtoffint1ack: {=bool:?}, mwswcitxintack: {=bool:?}, mwswcirxintack: {=bool:?}, audio0intack: {=bool:?}, audio1intack: {=bool:?}, audio2intack: {=bool:?}, errorintack: {=bool:?} }}" , self . frsyncintack () , self . mtoffint0ack () , self . mtoffint1ack () , self . mwswcitxintack () , self . mwswcirxintack () , self . audio0intack () , self . audio1intack () , self . audio2intack () , self . errorintack ())
    }
}
#[doc = "BTINTCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btintcntl0(pub u32);
impl Btintcntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn startfrmintmsk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_startfrmintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn endfrmintmsk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_endfrmintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn skipfrmintmsk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_skipfrmintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxintmsk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn frsyncintmsk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frsyncintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mtoffint0msk(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mtoffint0msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mtoffint1msk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mtoffint1msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwswcitxintmsk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwswcitxintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwswcirxintmsk(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwswcirxintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio0intmsk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio0intmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio1intmsk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio1intmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio2intmsk(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio2intmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn errorintmsk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Btintcntl0 {
    #[inline(always)]
    fn default() -> Btintcntl0 {
        Btintcntl0(0)
    }
}
impl core::fmt::Debug for Btintcntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btintcntl0")
            .field("startfrmintmsk", &self.startfrmintmsk())
            .field("endfrmintmsk", &self.endfrmintmsk())
            .field("skipfrmintmsk", &self.skipfrmintmsk())
            .field("rxintmsk", &self.rxintmsk())
            .field("frsyncintmsk", &self.frsyncintmsk())
            .field("mtoffint0msk", &self.mtoffint0msk())
            .field("mtoffint1msk", &self.mtoffint1msk())
            .field("mwswcitxintmsk", &self.mwswcitxintmsk())
            .field("mwswcirxintmsk", &self.mwswcirxintmsk())
            .field("audio0intmsk", &self.audio0intmsk())
            .field("audio1intmsk", &self.audio1intmsk())
            .field("audio2intmsk", &self.audio2intmsk())
            .field("errorintmsk", &self.errorintmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btintcntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btintcntl0 {{ startfrmintmsk: {=bool:?}, endfrmintmsk: {=bool:?}, skipfrmintmsk: {=bool:?}, rxintmsk: {=bool:?}, frsyncintmsk: {=bool:?}, mtoffint0msk: {=bool:?}, mtoffint1msk: {=bool:?}, mwswcitxintmsk: {=bool:?}, mwswcirxintmsk: {=bool:?}, audio0intmsk: {=bool:?}, audio1intmsk: {=bool:?}, audio2intmsk: {=bool:?}, errorintmsk: {=bool:?} }}" , self . startfrmintmsk () , self . endfrmintmsk () , self . skipfrmintmsk () , self . rxintmsk () , self . frsyncintmsk () , self . mtoffint0msk () , self . mtoffint1msk () , self . mwswcitxintmsk () , self . mwswcirxintmsk () , self . audio0intmsk () , self . audio1intmsk () , self . audio2intmsk () , self . errorintmsk ())
    }
}
#[doc = "BTINTSTAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btintstat0(pub u32);
impl Btintstat0 {
    #[must_use]
    #[inline(always)]
    pub const fn frsyncintstat(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frsyncintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mtoffint0stat(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mtoffint0stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mtoffint1stat(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mtoffint1stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwswcitxintstat(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwswcitxintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mwswcirxintstat(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mwswcirxintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio0intstat(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio0intstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio1intstat(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio1intstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn audio2intstat(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_audio2intstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn errorintstat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Btintstat0 {
    #[inline(always)]
    fn default() -> Btintstat0 {
        Btintstat0(0)
    }
}
impl core::fmt::Debug for Btintstat0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btintstat0")
            .field("frsyncintstat", &self.frsyncintstat())
            .field("mtoffint0stat", &self.mtoffint0stat())
            .field("mtoffint1stat", &self.mtoffint1stat())
            .field("mwswcitxintstat", &self.mwswcitxintstat())
            .field("mwswcirxintstat", &self.mwswcirxintstat())
            .field("audio0intstat", &self.audio0intstat())
            .field("audio1intstat", &self.audio1intstat())
            .field("audio2intstat", &self.audio2intstat())
            .field("errorintstat", &self.errorintstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btintstat0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btintstat0 {{ frsyncintstat: {=bool:?}, mtoffint0stat: {=bool:?}, mtoffint1stat: {=bool:?}, mwswcitxintstat: {=bool:?}, mwswcirxintstat: {=bool:?}, audio0intstat: {=bool:?}, audio1intstat: {=bool:?}, audio2intstat: {=bool:?}, errorintstat: {=bool:?} }}" , self . frsyncintstat () , self . mtoffint0stat () , self . mtoffint1stat () , self . mwswcitxintstat () , self . mwswcirxintstat () , self . audio0intstat () , self . audio1intstat () , self . audio2intstat () , self . errorintstat ())
    }
}
#[doc = "BTMPRIO0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btmprio0(pub u32);
impl Btmprio0 {
    #[must_use]
    #[inline(always)]
    pub const fn btm0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Btmprio0 {
    #[inline(always)]
    fn default() -> Btmprio0 {
        Btmprio0(0)
    }
}
impl core::fmt::Debug for Btmprio0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btmprio0")
            .field("btm0", &self.btm0())
            .field("btm1", &self.btm1())
            .field("btm2", &self.btm2())
            .field("btm3", &self.btm3())
            .field("btm4", &self.btm4())
            .field("btm5", &self.btm5())
            .field("btm6", &self.btm6())
            .field("btm7", &self.btm7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btmprio0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btmprio0 {{ btm0: {=u8:?}, btm1: {=u8:?}, btm2: {=u8:?}, btm3: {=u8:?}, btm4: {=u8:?}, btm5: {=u8:?}, btm6: {=u8:?}, btm7: {=u8:?} }}" , self . btm0 () , self . btm1 () , self . btm2 () , self . btm3 () , self . btm4 () , self . btm5 () , self . btm6 () , self . btm7 ())
    }
}
#[doc = "BTMPRIO1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btmprio1(pub u32);
impl Btmprio1 {
    #[must_use]
    #[inline(always)]
    pub const fn btm8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Btmprio1 {
    #[inline(always)]
    fn default() -> Btmprio1 {
        Btmprio1(0)
    }
}
impl core::fmt::Debug for Btmprio1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btmprio1")
            .field("btm8", &self.btm8())
            .field("btm9", &self.btm9())
            .field("btm10", &self.btm10())
            .field("btm11", &self.btm11())
            .field("btm12", &self.btm12())
            .field("btm13", &self.btm13())
            .field("btm14", &self.btm14())
            .field("btm15", &self.btm15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btmprio1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btmprio1 {{ btm8: {=u8:?}, btm9: {=u8:?}, btm10: {=u8:?}, btm11: {=u8:?}, btm12: {=u8:?}, btm13: {=u8:?}, btm14: {=u8:?}, btm15: {=u8:?} }}" , self . btm8 () , self . btm9 () , self . btm10 () , self . btm11 () , self . btm12 () , self . btm13 () , self . btm14 () , self . btm15 ())
    }
}
#[doc = "BTMPRIO2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btmprio2(pub u32);
impl Btmprio2 {
    #[must_use]
    #[inline(always)]
    pub const fn btm16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm17(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm18(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm19(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btm20(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btm20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btmdefault(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btmdefault(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Btmprio2 {
    #[inline(always)]
    fn default() -> Btmprio2 {
        Btmprio2(0)
    }
}
impl core::fmt::Debug for Btmprio2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btmprio2")
            .field("btm16", &self.btm16())
            .field("btm17", &self.btm17())
            .field("btm18", &self.btm18())
            .field("btm19", &self.btm19())
            .field("btm20", &self.btm20())
            .field("btmdefault", &self.btmdefault())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btmprio2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btmprio2 {{ btm16: {=u8:?}, btm17: {=u8:?}, btm18: {=u8:?}, btm19: {=u8:?}, btm20: {=u8:?}, btmdefault: {=u8:?} }}" , self . btm16 () , self . btm17 () , self . btm18 () , self . btm19 () , self . btm20 () , self . btmdefault ())
    }
}
#[doc = "BTRADIOCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btradiocntl2(pub u32);
impl Btradiocntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn freqtable_ptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_freqtable_ptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn trailer_gating_val(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_trailer_gating_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Btradiocntl2 {
    #[inline(always)]
    fn default() -> Btradiocntl2 {
        Btradiocntl2(0)
    }
}
impl core::fmt::Debug for Btradiocntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btradiocntl2")
            .field("freqtable_ptr", &self.freqtable_ptr())
            .field("trailer_gating_val", &self.trailer_gating_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btradiocntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btradiocntl2 {{ freqtable_ptr: {=u16:?}, trailer_gating_val: {=u8:?} }}",
            self.freqtable_ptr(),
            self.trailer_gating_val()
        )
    }
}
#[doc = "BTRADIOCNTL3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btradiocntl3(pub u32);
impl Btradiocntl3 {
    #[must_use]
    #[inline(always)]
    pub const fn txrate0cfg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txrate0cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txrate1cfg(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txrate1cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txrate2cfg(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txrate2cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxrate0cfg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxrate0cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxrate1cfg(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxrate1cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxrate2cfg(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxrate2cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for Btradiocntl3 {
    #[inline(always)]
    fn default() -> Btradiocntl3 {
        Btradiocntl3(0)
    }
}
impl core::fmt::Debug for Btradiocntl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btradiocntl3")
            .field("txrate0cfg", &self.txrate0cfg())
            .field("txrate1cfg", &self.txrate1cfg())
            .field("txrate2cfg", &self.txrate2cfg())
            .field("rxrate0cfg", &self.rxrate0cfg())
            .field("rxrate1cfg", &self.rxrate1cfg())
            .field("rxrate2cfg", &self.rxrate2cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btradiocntl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btradiocntl3 {{ txrate0cfg: {=u8:?}, txrate1cfg: {=u8:?}, txrate2cfg: {=u8:?}, rxrate0cfg: {=u8:?}, rxrate1cfg: {=u8:?}, rxrate2cfg: {=u8:?} }}" , self . txrate0cfg () , self . txrate1cfg () , self . txrate2cfg () , self . rxrate0cfg () , self . rxrate1cfg () , self . rxrate2cfg ())
    }
}
#[doc = "BTRADIOPWRUPDN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btradiopwrupdn(pub u32);
impl Btradiopwrupdn {
    #[must_use]
    #[inline(always)]
    pub const fn txpwrupct(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrupct(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txpwrdnct(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpwrdnct(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpwrupct(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxpwrupct(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Btradiopwrupdn {
    #[inline(always)]
    fn default() -> Btradiopwrupdn {
        Btradiopwrupdn(0)
    }
}
impl core::fmt::Debug for Btradiopwrupdn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btradiopwrupdn")
            .field("txpwrupct", &self.txpwrupct())
            .field("txpwrdnct", &self.txpwrdnct())
            .field("rxpwrupct", &self.rxpwrupct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btradiopwrupdn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btradiopwrupdn {{ txpwrupct: {=u8:?}, txpwrdnct: {=u8:?}, rxpwrupct: {=u8:?} }}",
            self.txpwrupct(),
            self.txpwrdnct(),
            self.rxpwrupct()
        )
    }
}
#[doc = "BTRADIOTXRXTIM"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btradiotxrxtim(pub u32);
impl Btradiotxrxtim {
    #[must_use]
    #[inline(always)]
    pub const fn txpathdly(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txpathdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpathdly(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxpathdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sync_position(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sync_position(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Btradiotxrxtim {
    #[inline(always)]
    fn default() -> Btradiotxrxtim {
        Btradiotxrxtim(0)
    }
}
impl core::fmt::Debug for Btradiotxrxtim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btradiotxrxtim")
            .field("txpathdly", &self.txpathdly())
            .field("rxpathdly", &self.rxpathdly())
            .field("sync_position", &self.sync_position())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btradiotxrxtim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btradiotxrxtim {{ txpathdly: {=u8:?}, rxpathdly: {=u8:?}, sync_position: {=u8:?} }}",
            self.txpathdly(),
            self.rxpathdly(),
            self.sync_position()
        )
    }
}
#[doc = "BTRFTESTCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btrftestcntl(pub u32);
impl Btrftestcntl {
    #[must_use]
    #[inline(always)]
    pub const fn txpktcnten(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txpktcnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txpldsrc(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txpldsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn prbstype(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_prbstype(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn infinitetx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_infinitetx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn herrren(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_herrren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sserrren(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sserrren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn percount_mode(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_percount_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxpktcnten(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxpktcnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn infiniterx(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_infiniterx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Btrftestcntl {
    #[inline(always)]
    fn default() -> Btrftestcntl {
        Btrftestcntl(0)
    }
}
impl core::fmt::Debug for Btrftestcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btrftestcntl")
            .field("txpktcnten", &self.txpktcnten())
            .field("txpldsrc", &self.txpldsrc())
            .field("prbstype", &self.prbstype())
            .field("infinitetx", &self.infinitetx())
            .field("herrren", &self.herrren())
            .field("sserrren", &self.sserrren())
            .field("percount_mode", &self.percount_mode())
            .field("rxpktcnten", &self.rxpktcnten())
            .field("infiniterx", &self.infiniterx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btrftestcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btrftestcntl {{ txpktcnten: {=bool:?}, txpldsrc: {=bool:?}, prbstype: {=bool:?}, infinitetx: {=bool:?}, herrren: {=bool:?}, sserrren: {=bool:?}, percount_mode: {=u8:?}, rxpktcnten: {=bool:?}, infiniterx: {=bool:?} }}" , self . txpktcnten () , self . txpldsrc () , self . prbstype () , self . infinitetx () , self . herrren () , self . sserrren () , self . percount_mode () , self . rxpktcnten () , self . infiniterx ())
    }
}
#[doc = "BTRFTESTFREQ"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btrftestfreq(pub u32);
impl Btrftestfreq {
    #[must_use]
    #[inline(always)]
    pub const fn txfreq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxfreq(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn testmodeen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_testmodeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn directloopbacken(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_directloopbacken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn loopback_mode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_loopback_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Btrftestfreq {
    #[inline(always)]
    fn default() -> Btrftestfreq {
        Btrftestfreq(0)
    }
}
impl core::fmt::Debug for Btrftestfreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btrftestfreq")
            .field("txfreq", &self.txfreq())
            .field("rxfreq", &self.rxfreq())
            .field("testmodeen", &self.testmodeen())
            .field("directloopbacken", &self.directloopbacken())
            .field("loopback_mode", &self.loopback_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btrftestfreq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Btrftestfreq {{ txfreq: {=u8:?}, rxfreq: {=u8:?}, testmodeen: {=bool:?}, directloopbacken: {=bool:?}, loopback_mode: {=bool:?} }}" , self . txfreq () , self . rxfreq () , self . testmodeen () , self . directloopbacken () , self . loopback_mode ())
    }
}
#[doc = "BTRFTESTRXSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btrftestrxstat(pub u32);
impl Btrftestrxstat {
    #[must_use]
    #[inline(always)]
    pub const fn rxpktcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_rxpktcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Btrftestrxstat {
    #[inline(always)]
    fn default() -> Btrftestrxstat {
        Btrftestrxstat(0)
    }
}
impl core::fmt::Debug for Btrftestrxstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btrftestrxstat")
            .field("rxpktcnt", &self.rxpktcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btrftestrxstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btrftestrxstat {{ rxpktcnt: {=u32:?} }}",
            self.rxpktcnt()
        )
    }
}
#[doc = "BTRFTESTTXSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btrftesttxstat(pub u32);
impl Btrftesttxstat {
    #[must_use]
    #[inline(always)]
    pub const fn txpktcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_txpktcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Btrftesttxstat {
    #[inline(always)]
    fn default() -> Btrftesttxstat {
        Btrftesttxstat(0)
    }
}
impl core::fmt::Debug for Btrftesttxstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btrftesttxstat")
            .field("txpktcnt", &self.txpktcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btrftesttxstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btrftesttxstat {{ txpktcnt: {=u32:?} }}",
            self.txpktcnt()
        )
    }
}
#[doc = "BTSWPROFILING"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btswprofiling(pub u32);
impl Btswprofiling {
    #[must_use]
    #[inline(always)]
    pub const fn swprof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_swprof(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Btswprofiling {
    #[inline(always)]
    fn default() -> Btswprofiling {
        Btswprofiling(0)
    }
}
impl core::fmt::Debug for Btswprofiling {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btswprofiling")
            .field("swprof", &self.swprof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btswprofiling {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Btswprofiling {{ swprof: {=u32:?} }}", self.swprof())
    }
}
#[doc = "BTVERSION"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btversion(pub u32);
impl Btversion {
    #[must_use]
    #[inline(always)]
    pub const fn build_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_build_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn upg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_upg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn typ(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_typ(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Btversion {
    #[inline(always)]
    fn default() -> Btversion {
        Btversion(0)
    }
}
impl core::fmt::Debug for Btversion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btversion")
            .field("build_num", &self.build_num())
            .field("upg", &self.upg())
            .field("rel", &self.rel())
            .field("typ", &self.typ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btversion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btversion {{ build_num: {=u8:?}, upg: {=u8:?}, rel: {=u8:?}, typ: {=u8:?} }}",
            self.build_num(),
            self.upg(),
            self.rel(),
            self.typ()
        )
    }
}
#[doc = "CLKNCNTCORR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkncntcorr(pub u32);
impl Clkncntcorr {
    #[must_use]
    #[inline(always)]
    pub const fn clkncntcorr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_clkncntcorr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abs_delta(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_abs_delta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Clkncntcorr {
    #[inline(always)]
    fn default() -> Clkncntcorr {
        Clkncntcorr(0)
    }
}
impl core::fmt::Debug for Clkncntcorr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkncntcorr")
            .field("clkncntcorr", &self.clkncntcorr())
            .field("abs_delta", &self.abs_delta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkncntcorr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkncntcorr {{ clkncntcorr: {=u32:?}, abs_delta: {=bool:?} }}",
            self.clkncntcorr(),
            self.abs_delta()
        )
    }
}
#[doc = "CLKNTGT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkntgt1(pub u32);
impl Clkntgt1 {
    #[must_use]
    #[inline(always)]
    pub const fn clkntarget(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_clkntarget(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Clkntgt1 {
    #[inline(always)]
    fn default() -> Clkntgt1 {
        Clkntgt1(0)
    }
}
impl core::fmt::Debug for Clkntgt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkntgt1")
            .field("clkntarget", &self.clkntarget())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkntgt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkntgt1 {{ clkntarget: {=u32:?} }}", self.clkntarget())
    }
}
#[doc = "CLKNTGT2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkntgt2(pub u32);
impl Clkntgt2 {
    #[must_use]
    #[inline(always)]
    pub const fn clkntarget(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_clkntarget(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Clkntgt2 {
    #[inline(always)]
    fn default() -> Clkntgt2 {
        Clkntgt2(0)
    }
}
impl core::fmt::Debug for Clkntgt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkntgt2")
            .field("clkntarget", &self.clkntarget())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkntgt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkntgt2 {{ clkntarget: {=u32:?} }}", self.clkntarget())
    }
}
#[doc = "CLKNTGT3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkntgt3(pub u32);
impl Clkntgt3 {
    #[must_use]
    #[inline(always)]
    pub const fn clkntarget(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_clkntarget(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Clkntgt3 {
    #[inline(always)]
    fn default() -> Clkntgt3 {
        Clkntgt3(0)
    }
}
impl core::fmt::Debug for Clkntgt3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkntgt3")
            .field("clkntarget", &self.clkntarget())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkntgt3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkntgt3 {{ clkntarget: {=u32:?} }}", self.clkntarget())
    }
}
#[doc = "COEXCHN0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coexchn0(pub u32);
impl Coexchn0 {
    #[must_use]
    #[inline(always)]
    pub const fn coexchn31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_coexchn31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Coexchn0 {
    #[inline(always)]
    fn default() -> Coexchn0 {
        Coexchn0(0)
    }
}
impl core::fmt::Debug for Coexchn0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Coexchn0")
            .field("coexchn31_0", &self.coexchn31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Coexchn0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Coexchn0 {{ coexchn31_0: {=u32:?} }}",
            self.coexchn31_0()
        )
    }
}
#[doc = "COEXCHN1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coexchn1(pub u32);
impl Coexchn1 {
    #[must_use]
    #[inline(always)]
    pub const fn coexchn63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_coexchn63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Coexchn1 {
    #[inline(always)]
    fn default() -> Coexchn1 {
        Coexchn1(0)
    }
}
impl core::fmt::Debug for Coexchn1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Coexchn1")
            .field("coexchn63_32", &self.coexchn63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Coexchn1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Coexchn1 {{ coexchn63_32: {=u32:?} }}",
            self.coexchn63_32()
        )
    }
}
#[doc = "COEXCHN2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coexchn2(pub u32);
impl Coexchn2 {
    #[must_use]
    #[inline(always)]
    pub const fn coexchn78_64(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_coexchn78_64(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Coexchn2 {
    #[inline(always)]
    fn default() -> Coexchn2 {
        Coexchn2(0)
    }
}
impl core::fmt::Debug for Coexchn2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Coexchn2")
            .field("coexchn78_64", &self.coexchn78_64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Coexchn2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Coexchn2 {{ coexchn78_64: {=u16:?} }}",
            self.coexchn78_64()
        )
    }
}
#[doc = "DEEPSLCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deepslcntl(pub u32);
impl Deepslcntl {
    #[must_use]
    #[inline(always)]
    pub const fn deep_sleep_corr_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_deep_sleep_corr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn corr_mask_clknint(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_corr_mask_clknint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Deepslcntl {
    #[inline(always)]
    fn default() -> Deepslcntl {
        Deepslcntl(0)
    }
}
impl core::fmt::Debug for Deepslcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Deepslcntl")
            .field("deep_sleep_corr_en", &self.deep_sleep_corr_en())
            .field("corr_mask_clknint", &self.corr_mask_clknint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Deepslcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Deepslcntl {{ deep_sleep_corr_en: {=bool:?}, corr_mask_clknint: {=bool:?} }}",
            self.deep_sleep_corr_en(),
            self.corr_mask_clknint()
        )
    }
}
#[doc = "DFANCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfancntl(pub u32);
impl Dfancntl {
    #[must_use]
    #[inline(always)]
    pub const fn letxprimantid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_letxprimantid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn letxprimidcntlen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_letxprimidcntlen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lerxprimantid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_lerxprimantid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lerxprimidcntlen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lerxprimidcntlen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bttxprimantid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bttxprimantid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bttxprimidcntlen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bttxprimidcntlen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btrxprimantid(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_btrxprimantid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn btrxprimidcntlen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_btrxprimidcntlen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dfancntl {
    #[inline(always)]
    fn default() -> Dfancntl {
        Dfancntl(0)
    }
}
impl core::fmt::Debug for Dfancntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfancntl")
            .field("letxprimantid", &self.letxprimantid())
            .field("letxprimidcntlen", &self.letxprimidcntlen())
            .field("lerxprimantid", &self.lerxprimantid())
            .field("lerxprimidcntlen", &self.lerxprimidcntlen())
            .field("bttxprimantid", &self.bttxprimantid())
            .field("bttxprimidcntlen", &self.bttxprimidcntlen())
            .field("btrxprimantid", &self.btrxprimantid())
            .field("btrxprimidcntlen", &self.btrxprimidcntlen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfancntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dfancntl {{ letxprimantid: {=u8:?}, letxprimidcntlen: {=bool:?}, lerxprimantid: {=u8:?}, lerxprimidcntlen: {=bool:?}, bttxprimantid: {=u8:?}, bttxprimidcntlen: {=bool:?}, btrxprimantid: {=u8:?}, btrxprimidcntlen: {=bool:?} }}" , self . letxprimantid () , self . letxprimidcntlen () , self . lerxprimantid () , self . lerxprimidcntlen () , self . bttxprimantid () , self . bttxprimidcntlen () , self . btrxprimantid () , self . btrxprimidcntlen ())
    }
}
#[doc = "DFANTCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfantcntl(pub u32);
impl Dfantcntl {
    #[must_use]
    #[inline(always)]
    pub const fn txprimantid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txprimantid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txprimidcnten(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txprimidcnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxprimantid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxprimantid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxprimidcnten(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxprimidcnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Dfantcntl {
    #[inline(always)]
    fn default() -> Dfantcntl {
        Dfantcntl(0)
    }
}
impl core::fmt::Debug for Dfantcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfantcntl")
            .field("txprimantid", &self.txprimantid())
            .field("txprimidcnten", &self.txprimidcnten())
            .field("rxprimantid", &self.rxprimantid())
            .field("rxprimidcnten", &self.rxprimidcnten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfantcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dfantcntl {{ txprimantid: {=u8:?}, txprimidcnten: {=bool:?}, rxprimantid: {=u8:?}, rxprimidcnten: {=bool:?} }}" , self . txprimantid () , self . txprimidcnten () , self . rxprimantid () , self . rxprimidcnten ())
    }
}
#[doc = "DFCNTL0_1US"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfcntl01us(pub u32);
impl Dfcntl01us {
    #[must_use]
    #[inline(always)]
    pub const fn txswstinst0_1us(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txswstinst0_1us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxswstinst0_1us(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxswstinst0_1us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxsampstinst0_1us(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxsampstinst0_1us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Dfcntl01us {
    #[inline(always)]
    fn default() -> Dfcntl01us {
        Dfcntl01us(0)
    }
}
impl core::fmt::Debug for Dfcntl01us {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfcntl01us")
            .field("txswstinst0_1us", &self.txswstinst0_1us())
            .field("rxswstinst0_1us", &self.rxswstinst0_1us())
            .field("rxsampstinst0_1us", &self.rxsampstinst0_1us())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfcntl01us {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dfcntl01us {{ txswstinst0_1us: {=u8:?}, rxswstinst0_1us: {=u8:?}, rxsampstinst0_1us: {=u8:?} }}" , self . txswstinst0_1us () , self . rxswstinst0_1us () , self . rxsampstinst0_1us ())
    }
}
#[doc = "DFCNTL0_2US"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfcntl02us(pub u32);
impl Dfcntl02us {
    #[must_use]
    #[inline(always)]
    pub const fn txswstinst0_2us(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txswstinst0_2us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxswstinst0_2us(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxswstinst0_2us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxsampstinst0_2us(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxsampstinst0_2us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Dfcntl02us {
    #[inline(always)]
    fn default() -> Dfcntl02us {
        Dfcntl02us(0)
    }
}
impl core::fmt::Debug for Dfcntl02us {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfcntl02us")
            .field("txswstinst0_2us", &self.txswstinst0_2us())
            .field("rxswstinst0_2us", &self.rxswstinst0_2us())
            .field("rxsampstinst0_2us", &self.rxsampstinst0_2us())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfcntl02us {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dfcntl02us {{ txswstinst0_2us: {=u8:?}, rxswstinst0_2us: {=u8:?}, rxsampstinst0_2us: {=u8:?} }}" , self . txswstinst0_2us () , self . rxswstinst0_2us () , self . rxsampstinst0_2us ())
    }
}
#[doc = "DFCNTL1_1US"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfcntl11us(pub u32);
impl Dfcntl11us {
    #[must_use]
    #[inline(always)]
    pub const fn txswstinst1_1us(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txswstinst1_1us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxswstinst1_1us(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxswstinst1_1us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxsampstinst1_1us(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxsampstinst1_1us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Dfcntl11us {
    #[inline(always)]
    fn default() -> Dfcntl11us {
        Dfcntl11us(0)
    }
}
impl core::fmt::Debug for Dfcntl11us {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfcntl11us")
            .field("txswstinst1_1us", &self.txswstinst1_1us())
            .field("rxswstinst1_1us", &self.rxswstinst1_1us())
            .field("rxsampstinst1_1us", &self.rxsampstinst1_1us())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfcntl11us {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dfcntl11us {{ txswstinst1_1us: {=u8:?}, rxswstinst1_1us: {=u8:?}, rxsampstinst1_1us: {=u8:?} }}" , self . txswstinst1_1us () , self . rxswstinst1_1us () , self . rxsampstinst1_1us ())
    }
}
#[doc = "DFCNTL1_2US"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfcntl12us(pub u32);
impl Dfcntl12us {
    #[must_use]
    #[inline(always)]
    pub const fn txswstinst1_2us(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txswstinst1_2us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxswstinst1_2us(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxswstinst1_2us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxsampstinst1_2us(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxsampstinst1_2us(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Dfcntl12us {
    #[inline(always)]
    fn default() -> Dfcntl12us {
        Dfcntl12us(0)
    }
}
impl core::fmt::Debug for Dfcntl12us {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfcntl12us")
            .field("txswstinst1_2us", &self.txswstinst1_2us())
            .field("rxswstinst1_2us", &self.rxswstinst1_2us())
            .field("rxsampstinst1_2us", &self.rxsampstinst1_2us())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfcntl12us {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dfcntl12us {{ txswstinst1_2us: {=u8:?}, rxswstinst1_2us: {=u8:?}, rxsampstinst1_2us: {=u8:?} }}" , self . txswstinst1_2us () , self . rxswstinst1_2us () , self . rxsampstinst1_2us ())
    }
}
#[doc = "DFCURRENTPTR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfcurrentptr(pub u32);
impl Dfcurrentptr {
    #[must_use]
    #[inline(always)]
    pub const fn dfcurrentptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_dfcurrentptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Dfcurrentptr {
    #[inline(always)]
    fn default() -> Dfcurrentptr {
        Dfcurrentptr(0)
    }
}
impl core::fmt::Debug for Dfcurrentptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfcurrentptr")
            .field("dfcurrentptr", &self.dfcurrentptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfcurrentptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dfcurrentptr {{ dfcurrentptr: {=u16:?} }}",
            self.dfcurrentptr()
        )
    }
}
#[doc = "DFIFCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfifcntl(pub u32);
impl Dfifcntl {
    #[must_use]
    #[inline(always)]
    pub const fn symbol_order(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_symbol_order(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn msb_lsb_order(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_msb_lsb_order(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn if_width(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_if_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sampvalid_beh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sampvalid_beh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sampreq_beh(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sampreq_beh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn antswtcnt_beh(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_antswtcnt_beh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Dfifcntl {
    #[inline(always)]
    fn default() -> Dfifcntl {
        Dfifcntl(0)
    }
}
impl core::fmt::Debug for Dfifcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dfifcntl")
            .field("symbol_order", &self.symbol_order())
            .field("msb_lsb_order", &self.msb_lsb_order())
            .field("if_width", &self.if_width())
            .field("sampvalid_beh", &self.sampvalid_beh())
            .field("sampreq_beh", &self.sampreq_beh())
            .field("antswtcnt_beh", &self.antswtcnt_beh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dfifcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dfifcntl {{ symbol_order: {=bool:?}, msb_lsb_order: {=bool:?}, if_width: {=u8:?}, sampvalid_beh: {=u8:?}, sampreq_beh: {=bool:?}, antswtcnt_beh: {=bool:?} }}" , self . symbol_order () , self . msb_lsb_order () , self . if_width () , self . sampvalid_beh () , self . sampreq_beh () , self . antswtcnt_beh ())
    }
}
#[doc = "DMAESPTR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmaesptr(pub u32);
impl Dmaesptr {
    #[must_use]
    #[inline(always)]
    pub const fn aesptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_aesptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dmaesptr {
    #[inline(always)]
    fn default() -> Dmaesptr {
        Dmaesptr(0)
    }
}
impl core::fmt::Debug for Dmaesptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmaesptr")
            .field("aesptr", &self.aesptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmaesptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmaesptr {{ aesptr: {=u16:?} }}", self.aesptr())
    }
}
#[doc = "DMDEBUGADDMAX"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdebugaddmax(pub u32);
impl Dmdebugaddmax {
    #[must_use]
    #[inline(always)]
    pub const fn em_addmax(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_em_addmax(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reg_addmax(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_reg_addmax(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Dmdebugaddmax {
    #[inline(always)]
    fn default() -> Dmdebugaddmax {
        Dmdebugaddmax(0)
    }
}
impl core::fmt::Debug for Dmdebugaddmax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmdebugaddmax")
            .field("em_addmax", &self.em_addmax())
            .field("reg_addmax", &self.reg_addmax())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmdebugaddmax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmdebugaddmax {{ em_addmax: {=u16:?}, reg_addmax: {=u16:?} }}",
            self.em_addmax(),
            self.reg_addmax()
        )
    }
}
#[doc = "DMDEBUGADDMIN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdebugaddmin(pub u32);
impl Dmdebugaddmin {
    #[must_use]
    #[inline(always)]
    pub const fn em_addmin(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_em_addmin(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reg_addmin(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_reg_addmin(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Dmdebugaddmin {
    #[inline(always)]
    fn default() -> Dmdebugaddmin {
        Dmdebugaddmin(0)
    }
}
impl core::fmt::Debug for Dmdebugaddmin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmdebugaddmin")
            .field("em_addmin", &self.em_addmin())
            .field("reg_addmin", &self.reg_addmin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmdebugaddmin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmdebugaddmin {{ em_addmin: {=u16:?}, reg_addmin: {=u16:?} }}",
            self.em_addmin(),
            self.reg_addmin()
        )
    }
}
#[doc = "DMDIAGCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdiagcntl(pub u32);
impl Dmdiagcntl {
    #[must_use]
    #[inline(always)]
    pub const fn diag0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag0_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_diag3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmdiagcntl {
    #[inline(always)]
    fn default() -> Dmdiagcntl {
        Dmdiagcntl(0)
    }
}
impl core::fmt::Debug for Dmdiagcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmdiagcntl")
            .field("diag0", &self.diag0())
            .field("diag0_en", &self.diag0_en())
            .field("diag1", &self.diag1())
            .field("diag1_en", &self.diag1_en())
            .field("diag2", &self.diag2())
            .field("diag2_en", &self.diag2_en())
            .field("diag3", &self.diag3())
            .field("diag3_en", &self.diag3_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmdiagcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmdiagcntl {{ diag0: {=u8:?}, diag0_en: {=bool:?}, diag1: {=u8:?}, diag1_en: {=bool:?}, diag2: {=u8:?}, diag2_en: {=bool:?}, diag3: {=u8:?}, diag3_en: {=bool:?} }}" , self . diag0 () , self . diag0_en () , self . diag1 () , self . diag1_en () , self . diag2 () , self . diag2_en () , self . diag3 () , self . diag3_en ())
    }
}
#[doc = "DMDIAGSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdiagstat(pub u32);
impl Dmdiagstat {
    #[must_use]
    #[inline(always)]
    pub const fn diag0stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag0stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag1stat(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag1stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag2stat(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag2stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn diag3stat(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_diag3stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Dmdiagstat {
    #[inline(always)]
    fn default() -> Dmdiagstat {
        Dmdiagstat(0)
    }
}
impl core::fmt::Debug for Dmdiagstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmdiagstat")
            .field("diag0stat", &self.diag0stat())
            .field("diag1stat", &self.diag1stat())
            .field("diag2stat", &self.diag2stat())
            .field("diag3stat", &self.diag3stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmdiagstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmdiagstat {{ diag0stat: {=u8:?}, diag1stat: {=u8:?}, diag2stat: {=u8:?}, diag3stat: {=u8:?} }}" , self . diag0stat () , self . diag1stat () , self . diag2stat () , self . diag3stat ())
    }
}
#[doc = "DMERRORTYPESTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmerrortypestat(pub u32);
impl Dmerrortypestat {
    #[must_use]
    #[inline(always)]
    pub const fn radio_emacc_error(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_radio_emacc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn fifowriteerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_fifowriteerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn act_schdl_entry_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_act_schdl_entry_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn act_schdl_apfm_error(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_act_schdl_apfm_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Dmerrortypestat {
    #[inline(always)]
    fn default() -> Dmerrortypestat {
        Dmerrortypestat(0)
    }
}
impl core::fmt::Debug for Dmerrortypestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmerrortypestat")
            .field("radio_emacc_error", &self.radio_emacc_error())
            .field("fifowriteerr", &self.fifowriteerr())
            .field("act_schdl_entry_error", &self.act_schdl_entry_error())
            .field("act_schdl_apfm_error", &self.act_schdl_apfm_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmerrortypestat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmerrortypestat {{ radio_emacc_error: {=bool:?}, fifowriteerr: {=bool:?}, act_schdl_entry_error: {=bool:?}, act_schdl_apfm_error: {=bool:?} }}" , self . radio_emacc_error () , self . fifowriteerr () , self . act_schdl_entry_error () , self . act_schdl_apfm_error ())
    }
}
#[doc = "DMINTACK0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmintack0(pub u32);
impl Dmintack0 {
    #[must_use]
    #[inline(always)]
    pub const fn errorintack(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Dmintack0 {
    #[inline(always)]
    fn default() -> Dmintack0 {
        Dmintack0(0)
    }
}
impl core::fmt::Debug for Dmintack0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmintack0")
            .field("errorintack", &self.errorintack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmintack0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmintack0 {{ errorintack: {=bool:?} }}",
            self.errorintack()
        )
    }
}
#[doc = "DMINTACK1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmintack1(pub u32);
impl Dmintack1 {
    #[must_use]
    #[inline(always)]
    pub const fn clkntintack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clkntintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn slpintack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slpintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cryptintack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cryptintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn swintack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_swintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn finetgtintack(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_finetgtintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt1intack(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt1intack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt2intack(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt2intack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt3intack(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt3intack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccalintack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccalintack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn fifointack(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_fifointack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Dmintack1 {
    #[inline(always)]
    fn default() -> Dmintack1 {
        Dmintack1(0)
    }
}
impl core::fmt::Debug for Dmintack1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmintack1")
            .field("clkntintack", &self.clkntintack())
            .field("slpintack", &self.slpintack())
            .field("cryptintack", &self.cryptintack())
            .field("swintack", &self.swintack())
            .field("finetgtintack", &self.finetgtintack())
            .field("timestamptgt1intack", &self.timestamptgt1intack())
            .field("timestamptgt2intack", &self.timestamptgt2intack())
            .field("timestamptgt3intack", &self.timestamptgt3intack())
            .field("rccalintack", &self.rccalintack())
            .field("fifointack", &self.fifointack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmintack1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmintack1 {{ clkntintack: {=bool:?}, slpintack: {=bool:?}, cryptintack: {=bool:?}, swintack: {=bool:?}, finetgtintack: {=bool:?}, timestamptgt1intack: {=bool:?}, timestamptgt2intack: {=bool:?}, timestamptgt3intack: {=bool:?}, rccalintack: {=bool:?}, fifointack: {=bool:?} }}" , self . clkntintack () , self . slpintack () , self . cryptintack () , self . swintack () , self . finetgtintack () , self . timestamptgt1intack () , self . timestamptgt2intack () , self . timestamptgt3intack () , self . rccalintack () , self . fifointack ())
    }
}
#[doc = "DMINTCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmintcntl0(pub u32);
impl Dmintcntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn errorintmsk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Dmintcntl0 {
    #[inline(always)]
    fn default() -> Dmintcntl0 {
        Dmintcntl0(0)
    }
}
impl core::fmt::Debug for Dmintcntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmintcntl0")
            .field("errorintmsk", &self.errorintmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmintcntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmintcntl0 {{ errorintmsk: {=bool:?} }}",
            self.errorintmsk()
        )
    }
}
#[doc = "DMINTCNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmintcntl1(pub u32);
impl Dmintcntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn clknintmsk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clknintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn slpintmsk(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slpintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cryptintmsk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cryptintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn swintmsk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_swintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn finetgtntmsk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_finetgtntmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt1intmsk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt1intmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt2intmsk(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt2intmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt3intmsk(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt3intmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccalintmsk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccalintmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn fifointmsk(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_fifointmsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clknintsrval(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_clknintsrval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clknintsrmsk(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_clknintsrmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Dmintcntl1 {
    #[inline(always)]
    fn default() -> Dmintcntl1 {
        Dmintcntl1(0)
    }
}
impl core::fmt::Debug for Dmintcntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmintcntl1")
            .field("clknintmsk", &self.clknintmsk())
            .field("slpintmsk", &self.slpintmsk())
            .field("cryptintmsk", &self.cryptintmsk())
            .field("swintmsk", &self.swintmsk())
            .field("finetgtntmsk", &self.finetgtntmsk())
            .field("timestamptgt1intmsk", &self.timestamptgt1intmsk())
            .field("timestamptgt2intmsk", &self.timestamptgt2intmsk())
            .field("timestamptgt3intmsk", &self.timestamptgt3intmsk())
            .field("rccalintmsk", &self.rccalintmsk())
            .field("fifointmsk", &self.fifointmsk())
            .field("clknintsrval", &self.clknintsrval())
            .field("clknintsrmsk", &self.clknintsrmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmintcntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmintcntl1 {{ clknintmsk: {=bool:?}, slpintmsk: {=bool:?}, cryptintmsk: {=bool:?}, swintmsk: {=bool:?}, finetgtntmsk: {=bool:?}, timestamptgt1intmsk: {=bool:?}, timestamptgt2intmsk: {=bool:?}, timestamptgt3intmsk: {=bool:?}, rccalintmsk: {=bool:?}, fifointmsk: {=bool:?}, clknintsrval: {=u8:?}, clknintsrmsk: {=u8:?} }}" , self . clknintmsk () , self . slpintmsk () , self . cryptintmsk () , self . swintmsk () , self . finetgtntmsk () , self . timestamptgt1intmsk () , self . timestamptgt2intmsk () , self . timestamptgt3intmsk () , self . rccalintmsk () , self . fifointmsk () , self . clknintsrval () , self . clknintsrmsk ())
    }
}
#[doc = "DMINTSTAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmintstat0(pub u32);
impl Dmintstat0 {
    #[must_use]
    #[inline(always)]
    pub const fn errorintstat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_errorintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Dmintstat0 {
    #[inline(always)]
    fn default() -> Dmintstat0 {
        Dmintstat0(0)
    }
}
impl core::fmt::Debug for Dmintstat0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmintstat0")
            .field("errorintstat", &self.errorintstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmintstat0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmintstat0 {{ errorintstat: {=bool:?} }}",
            self.errorintstat()
        )
    }
}
#[doc = "DMINTSTAT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmintstat1(pub u32);
impl Dmintstat1 {
    #[must_use]
    #[inline(always)]
    pub const fn clknintstat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clknintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn slpintstat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slpintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cryptintstat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cryptintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn swintstat(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_swintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn finetgtintstat(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_finetgtintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt1intstat(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt1intstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt2intstat(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt2intstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn timestamptgt3intstat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_timestamptgt3intstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccalintstat(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccalintstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn fifointstat(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_fifointstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Dmintstat1 {
    #[inline(always)]
    fn default() -> Dmintstat1 {
        Dmintstat1(0)
    }
}
impl core::fmt::Debug for Dmintstat1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmintstat1")
            .field("clknintstat", &self.clknintstat())
            .field("slpintstat", &self.slpintstat())
            .field("cryptintstat", &self.cryptintstat())
            .field("swintstat", &self.swintstat())
            .field("finetgtintstat", &self.finetgtintstat())
            .field("timestamptgt1intstat", &self.timestamptgt1intstat())
            .field("timestamptgt2intstat", &self.timestamptgt2intstat())
            .field("timestamptgt3intstat", &self.timestamptgt3intstat())
            .field("rccalintstat", &self.rccalintstat())
            .field("fifointstat", &self.fifointstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmintstat1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmintstat1 {{ clknintstat: {=bool:?}, slpintstat: {=bool:?}, cryptintstat: {=bool:?}, swintstat: {=bool:?}, finetgtintstat: {=bool:?}, timestamptgt1intstat: {=bool:?}, timestamptgt2intstat: {=bool:?}, timestamptgt3intstat: {=bool:?}, rccalintstat: {=bool:?}, fifointstat: {=bool:?} }}" , self . clknintstat () , self . slpintstat () , self . cryptintstat () , self . swintstat () , self . finetgtintstat () , self . timestamptgt1intstat () , self . timestamptgt2intstat () , self . timestamptgt3intstat () , self . rccalintstat () , self . fifointstat ())
    }
}
#[doc = "DMRADIOCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmradiocntl0(pub u32);
impl Dmradiocntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn phy_rf_rxoff_delay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_phy_rf_rxoff_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn phy_rf_txoff_delay(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_phy_rf_txoff_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn phy_rxon_delay(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_phy_rxon_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn phy_txon_delay(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_phy_txon_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Dmradiocntl0 {
    #[inline(always)]
    fn default() -> Dmradiocntl0 {
        Dmradiocntl0(0)
    }
}
impl core::fmt::Debug for Dmradiocntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmradiocntl0")
            .field("phy_rf_rxoff_delay", &self.phy_rf_rxoff_delay())
            .field("phy_rf_txoff_delay", &self.phy_rf_txoff_delay())
            .field("phy_rxon_delay", &self.phy_rxon_delay())
            .field("phy_txon_delay", &self.phy_txon_delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmradiocntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmradiocntl0 {{ phy_rf_rxoff_delay: {=u8:?}, phy_rf_txoff_delay: {=u8:?}, phy_rxon_delay: {=u8:?}, phy_txon_delay: {=u8:?} }}" , self . phy_rf_rxoff_delay () , self . phy_rf_txoff_delay () , self . phy_rxon_delay () , self . phy_txon_delay ())
    }
}
#[doc = "DMRADIOCNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmradiocntl1(pub u32);
impl Dmradiocntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn rx_samp_dly(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_samp_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ext_master(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ext_master(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ext_slave(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ext_slave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dbgtrigsel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dbgtrigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_nbt_ble_val(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_nbt_ble_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_nbt_ble(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_nbt_ble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_rate_val(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_force_rate_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_rate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_rate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_rx_val(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_rx_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_rx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_tx_val(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_tx_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_tx(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn channel(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_channel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 23usize)) | (((val as u32) & 0x7f) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_channel(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_channel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_syncword(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_syncword(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmradiocntl1 {
    #[inline(always)]
    fn default() -> Dmradiocntl1 {
        Dmradiocntl1(0)
    }
}
impl core::fmt::Debug for Dmradiocntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmradiocntl1")
            .field("rx_samp_dly", &self.rx_samp_dly())
            .field("ext_master", &self.ext_master())
            .field("ext_slave", &self.ext_slave())
            .field("dbgtrigsel", &self.dbgtrigsel())
            .field("force_nbt_ble_val", &self.force_nbt_ble_val())
            .field("force_nbt_ble", &self.force_nbt_ble())
            .field("force_rate_val", &self.force_rate_val())
            .field("force_rate", &self.force_rate())
            .field("force_rx_val", &self.force_rx_val())
            .field("force_rx", &self.force_rx())
            .field("force_tx_val", &self.force_tx_val())
            .field("force_tx", &self.force_tx())
            .field("channel", &self.channel())
            .field("force_channel", &self.force_channel())
            .field("force_syncword", &self.force_syncword())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmradiocntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmradiocntl1 {{ rx_samp_dly: {=u8:?}, ext_master: {=bool:?}, ext_slave: {=bool:?}, dbgtrigsel: {=u8:?}, force_nbt_ble_val: {=bool:?}, force_nbt_ble: {=bool:?}, force_rate_val: {=u8:?}, force_rate: {=bool:?}, force_rx_val: {=bool:?}, force_rx: {=bool:?}, force_tx_val: {=bool:?}, force_tx: {=bool:?}, channel: {=u8:?}, force_channel: {=bool:?}, force_syncword: {=bool:?} }}" , self . rx_samp_dly () , self . ext_master () , self . ext_slave () , self . dbgtrigsel () , self . force_nbt_ble_val () , self . force_nbt_ble () , self . force_rate_val () , self . force_rate () , self . force_rx_val () , self . force_rx () , self . force_tx_val () , self . force_tx () , self . channel () , self . force_channel () , self . force_syncword ())
    }
}
#[doc = "DMRADIOCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmradiocntl2(pub u32);
impl Dmradiocntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn syncword1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_syncword1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmradiocntl2 {
    #[inline(always)]
    fn default() -> Dmradiocntl2 {
        Dmradiocntl2(0)
    }
}
impl core::fmt::Debug for Dmradiocntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmradiocntl2")
            .field("syncword1", &self.syncword1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmradiocntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmradiocntl2 {{ syncword1: {=u32:?} }}",
            self.syncword1()
        )
    }
}
#[doc = "DMRADIOCNTL3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmradiocntl3(pub u32);
impl Dmradiocntl3 {
    #[must_use]
    #[inline(always)]
    pub const fn syncword2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_syncword2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmradiocntl3 {
    #[inline(always)]
    fn default() -> Dmradiocntl3 {
        Dmradiocntl3(0)
    }
}
impl core::fmt::Debug for Dmradiocntl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmradiocntl3")
            .field("syncword2", &self.syncword2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmradiocntl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmradiocntl3 {{ syncword2: {=u32:?} }}",
            self.syncword2()
        )
    }
}
#[doc = "DMRADIOCNTL4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmradiocntl4(pub u32);
impl Dmradiocntl4 {
    #[must_use]
    #[inline(always)]
    pub const fn chptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_chptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmradiocntl4 {
    #[inline(always)]
    fn default() -> Dmradiocntl4 {
        Dmradiocntl4(0)
    }
}
impl core::fmt::Debug for Dmradiocntl4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmradiocntl4")
            .field("chptr", &self.chptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmradiocntl4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmradiocntl4 {{ chptr: {=u32:?} }}", self.chptr())
    }
}
#[doc = "DMSWPROFILING"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmswprofiling(pub u32);
impl Dmswprofiling {
    #[must_use]
    #[inline(always)]
    pub const fn swprof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_swprof(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmswprofiling {
    #[inline(always)]
    fn default() -> Dmswprofiling {
        Dmswprofiling(0)
    }
}
impl core::fmt::Debug for Dmswprofiling {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmswprofiling")
            .field("swprof", &self.swprof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmswprofiling {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmswprofiling {{ swprof: {=u32:?} }}", self.swprof())
    }
}
#[doc = "DMTIMGENCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmtimgencntl(pub u32);
impl Dmtimgencntl {
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_time(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_prefetch_time(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn prefetchabort_time(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_prefetchabort_time(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Dmtimgencntl {
    #[inline(always)]
    fn default() -> Dmtimgencntl {
        Dmtimgencntl(0)
    }
}
impl core::fmt::Debug for Dmtimgencntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmtimgencntl")
            .field("prefetch_time", &self.prefetch_time())
            .field("prefetchabort_time", &self.prefetchabort_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmtimgencntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmtimgencntl {{ prefetch_time: {=u16:?}, prefetchabort_time: {=u16:?} }}",
            self.prefetch_time(),
            self.prefetchabort_time()
        )
    }
}
#[doc = "DMVERSION"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmversion(pub u32);
impl Dmversion {
    #[must_use]
    #[inline(always)]
    pub const fn build_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_build_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn upg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_upg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn typ(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_typ(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Dmversion {
    #[inline(always)]
    fn default() -> Dmversion {
        Dmversion(0)
    }
}
impl core::fmt::Debug for Dmversion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmversion")
            .field("build_num", &self.build_num())
            .field("upg", &self.upg())
            .field("rel", &self.rel())
            .field("typ", &self.typ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmversion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmversion {{ build_num: {=u8:?}, upg: {=u8:?}, rel: {=u8:?}, typ: {=u8:?} }}",
            self.build_num(),
            self.upg(),
            self.rel(),
            self.typ()
        )
    }
}
#[doc = "EDRCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edrcntl(pub u32);
impl Edrcntl {
    #[must_use]
    #[inline(always)]
    pub const fn rxgrd_timeout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxgrd_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn gb_txqual_gen_dsb(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_gb_txqual_gen_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxguarddsb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxguarddsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn guard_band_time(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_guard_band_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_swap(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tx_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_swap(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rx_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edrbcast(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edrbcast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txrate_swinstant(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txrate_swinstant(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Edrcntl {
    #[inline(always)]
    fn default() -> Edrcntl {
        Edrcntl(0)
    }
}
impl core::fmt::Debug for Edrcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Edrcntl")
            .field("rxgrd_timeout", &self.rxgrd_timeout())
            .field("gb_txqual_gen_dsb", &self.gb_txqual_gen_dsb())
            .field("rxguarddsb", &self.rxguarddsb())
            .field("guard_band_time", &self.guard_band_time())
            .field("tx_swap", &self.tx_swap())
            .field("rx_swap", &self.rx_swap())
            .field("edrbcast", &self.edrbcast())
            .field("txrate_swinstant", &self.txrate_swinstant())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Edrcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Edrcntl {{ rxgrd_timeout: {=u8:?}, gb_txqual_gen_dsb: {=bool:?}, rxguarddsb: {=bool:?}, guard_band_time: {=u8:?}, tx_swap: {=bool:?}, rx_swap: {=bool:?}, edrbcast: {=bool:?}, txrate_swinstant: {=bool:?} }}" , self . rxgrd_timeout () , self . gb_txqual_gen_dsb () , self . rxguarddsb () , self . guard_band_time () , self . tx_swap () , self . rx_swap () , self . edrbcast () , self . txrate_swinstant ())
    }
}
#[doc = "ENDEVTCLKN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endevtclkn(pub u32);
impl Endevtclkn {
    #[must_use]
    #[inline(always)]
    pub const fn endevtclknts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_endevtclknts(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Endevtclkn {
    #[inline(always)]
    fn default() -> Endevtclkn {
        Endevtclkn(0)
    }
}
impl core::fmt::Debug for Endevtclkn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endevtclkn")
            .field("endevtclknts", &self.endevtclknts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endevtclkn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endevtclkn {{ endevtclknts: {=u32:?} }}",
            self.endevtclknts()
        )
    }
}
#[doc = "ENDEVTFINECNT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endevtfinecnt(pub u32);
impl Endevtfinecnt {
    #[must_use]
    #[inline(always)]
    pub const fn endevtfinecntts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_endevtfinecntts(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Endevtfinecnt {
    #[inline(always)]
    fn default() -> Endevtfinecnt {
        Endevtfinecnt(0)
    }
}
impl core::fmt::Debug for Endevtfinecnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endevtfinecnt")
            .field("endevtfinecntts", &self.endevtfinecntts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endevtfinecnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endevtfinecnt {{ endevtfinecntts: {=u16:?} }}",
            self.endevtfinecntts()
        )
    }
}
#[doc = "ENDFRMCLKNTS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endfrmclknts(pub u32);
impl Endfrmclknts {
    #[must_use]
    #[inline(always)]
    pub const fn endfrmclknts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_endfrmclknts(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Endfrmclknts {
    #[inline(always)]
    fn default() -> Endfrmclknts {
        Endfrmclknts(0)
    }
}
impl core::fmt::Debug for Endfrmclknts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endfrmclknts")
            .field("endfrmclknts", &self.endfrmclknts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endfrmclknts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endfrmclknts {{ endfrmclknts: {=u32:?} }}",
            self.endfrmclknts()
        )
    }
}
#[doc = "ENDFRMFINECNTTS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endfrmfinecntts(pub u32);
impl Endfrmfinecntts {
    #[must_use]
    #[inline(always)]
    pub const fn endfrmfinecntts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_endfrmfinecntts(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Endfrmfinecntts {
    #[inline(always)]
    fn default() -> Endfrmfinecntts {
        Endfrmfinecntts(0)
    }
}
impl core::fmt::Debug for Endfrmfinecntts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endfrmfinecntts")
            .field("endfrmfinecntts", &self.endfrmfinecntts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endfrmfinecntts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endfrmfinecntts {{ endfrmfinecntts: {=u16:?} }}",
            self.endfrmfinecntts()
        )
    }
}
#[doc = "ESCOCHANCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escochancntl0(pub u32);
impl Escochancntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn tesco0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tesco0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn intdelay0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_intdelay0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn itmode0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_itmode0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escochanen0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escochanen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escochanswen0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escochanswen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tog0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Escochancntl0 {
    #[inline(always)]
    fn default() -> Escochancntl0 {
        Escochancntl0(0)
    }
}
impl core::fmt::Debug for Escochancntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escochancntl0")
            .field("tesco0", &self.tesco0())
            .field("intdelay0", &self.intdelay0())
            .field("itmode0", &self.itmode0())
            .field("escochanen0", &self.escochanen0())
            .field("escochanswen0", &self.escochanswen0())
            .field("tog0", &self.tog0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escochancntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escochancntl0 {{ tesco0: {=u8:?}, intdelay0: {=u8:?}, itmode0: {=bool:?}, escochanen0: {=bool:?}, escochanswen0: {=bool:?}, tog0: {=bool:?} }}" , self . tesco0 () , self . intdelay0 () , self . itmode0 () , self . escochanen0 () , self . escochanswen0 () , self . tog0 ())
    }
}
#[doc = "ESCOCHANCNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escochancntl1(pub u32);
impl Escochancntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn tesco1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tesco1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn intdelay1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_intdelay1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn itmode1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_itmode1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escochanen1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escochanen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escochanswen1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escochanswen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tog1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Escochancntl1 {
    #[inline(always)]
    fn default() -> Escochancntl1 {
        Escochancntl1(0)
    }
}
impl core::fmt::Debug for Escochancntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escochancntl1")
            .field("tesco1", &self.tesco1())
            .field("intdelay1", &self.intdelay1())
            .field("itmode1", &self.itmode1())
            .field("escochanen1", &self.escochanen1())
            .field("escochanswen1", &self.escochanswen1())
            .field("tog1", &self.tog1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escochancntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escochancntl1 {{ tesco1: {=u8:?}, intdelay1: {=u8:?}, itmode1: {=bool:?}, escochanen1: {=bool:?}, escochanswen1: {=bool:?}, tog1: {=bool:?} }}" , self . tesco1 () , self . intdelay1 () , self . itmode1 () , self . escochanen1 () , self . escochanswen1 () , self . tog1 ())
    }
}
#[doc = "ESCOCHANCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escochancntl2(pub u32);
impl Escochancntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn tesco2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tesco2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn intdelay2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_intdelay2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn itmode2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_itmode2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escochanen2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escochanen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escochanswen2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escochanswen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tog2(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tog2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Escochancntl2 {
    #[inline(always)]
    fn default() -> Escochancntl2 {
        Escochancntl2(0)
    }
}
impl core::fmt::Debug for Escochancntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escochancntl2")
            .field("tesco2", &self.tesco2())
            .field("intdelay2", &self.intdelay2())
            .field("itmode2", &self.itmode2())
            .field("escochanen2", &self.escochanen2())
            .field("escochanswen2", &self.escochanswen2())
            .field("tog2", &self.tog2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escochancntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escochancntl2 {{ tesco2: {=u8:?}, intdelay2: {=u8:?}, itmode2: {=bool:?}, escochanen2: {=bool:?}, escochanswen2: {=bool:?}, tog2: {=bool:?} }}" , self . tesco2 () , self . intdelay2 () , self . itmode2 () , self . escochanen2 () , self . escochanswen2 () , self . tog2 ())
    }
}
#[doc = "ESCOCURRENTRXPTR0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escocurrentrxptr0(pub u32);
impl Escocurrentrxptr0 {
    #[must_use]
    #[inline(always)]
    pub const fn esco0ptrrx0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco0ptrrx0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn esco0ptrrx1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco0ptrrx1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Escocurrentrxptr0 {
    #[inline(always)]
    fn default() -> Escocurrentrxptr0 {
        Escocurrentrxptr0(0)
    }
}
impl core::fmt::Debug for Escocurrentrxptr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escocurrentrxptr0")
            .field("esco0ptrrx0", &self.esco0ptrrx0())
            .field("esco0ptrrx1", &self.esco0ptrrx1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escocurrentrxptr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escocurrentrxptr0 {{ esco0ptrrx0: {=u16:?}, esco0ptrrx1: {=u16:?} }}",
            self.esco0ptrrx0(),
            self.esco0ptrrx1()
        )
    }
}
#[doc = "ESCOCURRENTRXPTR1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escocurrentrxptr1(pub u32);
impl Escocurrentrxptr1 {
    #[must_use]
    #[inline(always)]
    pub const fn esco1ptrrx0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco1ptrrx0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn esco1ptrrx1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco1ptrrx1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Escocurrentrxptr1 {
    #[inline(always)]
    fn default() -> Escocurrentrxptr1 {
        Escocurrentrxptr1(0)
    }
}
impl core::fmt::Debug for Escocurrentrxptr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escocurrentrxptr1")
            .field("esco1ptrrx0", &self.esco1ptrrx0())
            .field("esco1ptrrx1", &self.esco1ptrrx1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escocurrentrxptr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escocurrentrxptr1 {{ esco1ptrrx0: {=u16:?}, esco1ptrrx1: {=u16:?} }}",
            self.esco1ptrrx0(),
            self.esco1ptrrx1()
        )
    }
}
#[doc = "ESCOCURRENTRXPTR2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escocurrentrxptr2(pub u32);
impl Escocurrentrxptr2 {
    #[must_use]
    #[inline(always)]
    pub const fn esco2ptrrx0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco2ptrrx0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn esco2ptrrx1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco2ptrrx1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Escocurrentrxptr2 {
    #[inline(always)]
    fn default() -> Escocurrentrxptr2 {
        Escocurrentrxptr2(0)
    }
}
impl core::fmt::Debug for Escocurrentrxptr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escocurrentrxptr2")
            .field("esco2ptrrx0", &self.esco2ptrrx0())
            .field("esco2ptrrx1", &self.esco2ptrrx1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escocurrentrxptr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escocurrentrxptr2 {{ esco2ptrrx0: {=u16:?}, esco2ptrrx1: {=u16:?} }}",
            self.esco2ptrrx0(),
            self.esco2ptrrx1()
        )
    }
}
#[doc = "ESCOCURRENTTXPTR0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escocurrenttxptr0(pub u32);
impl Escocurrenttxptr0 {
    #[must_use]
    #[inline(always)]
    pub const fn esco0ptrtx0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco0ptrtx0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn esco0ptrtx1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco0ptrtx1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Escocurrenttxptr0 {
    #[inline(always)]
    fn default() -> Escocurrenttxptr0 {
        Escocurrenttxptr0(0)
    }
}
impl core::fmt::Debug for Escocurrenttxptr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escocurrenttxptr0")
            .field("esco0ptrtx0", &self.esco0ptrtx0())
            .field("esco0ptrtx1", &self.esco0ptrtx1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escocurrenttxptr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escocurrenttxptr0 {{ esco0ptrtx0: {=u16:?}, esco0ptrtx1: {=u16:?} }}",
            self.esco0ptrtx0(),
            self.esco0ptrtx1()
        )
    }
}
#[doc = "ESCOCURRENTTXPTR1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escocurrenttxptr1(pub u32);
impl Escocurrenttxptr1 {
    #[must_use]
    #[inline(always)]
    pub const fn esco1ptrtx0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco1ptrtx0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn esco1ptrtx1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco1ptrtx1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Escocurrenttxptr1 {
    #[inline(always)]
    fn default() -> Escocurrenttxptr1 {
        Escocurrenttxptr1(0)
    }
}
impl core::fmt::Debug for Escocurrenttxptr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escocurrenttxptr1")
            .field("esco1ptrtx0", &self.esco1ptrtx0())
            .field("esco1ptrtx1", &self.esco1ptrtx1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escocurrenttxptr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escocurrenttxptr1 {{ esco1ptrtx0: {=u16:?}, esco1ptrtx1: {=u16:?} }}",
            self.esco1ptrtx0(),
            self.esco1ptrtx1()
        )
    }
}
#[doc = "ESCOCURRENTTXPTR2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escocurrenttxptr2(pub u32);
impl Escocurrenttxptr2 {
    #[must_use]
    #[inline(always)]
    pub const fn esco2ptrtx0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco2ptrtx0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn esco2ptrtx1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_esco2ptrtx1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Escocurrenttxptr2 {
    #[inline(always)]
    fn default() -> Escocurrenttxptr2 {
        Escocurrenttxptr2(0)
    }
}
impl core::fmt::Debug for Escocurrenttxptr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escocurrenttxptr2")
            .field("esco2ptrtx0", &self.esco2ptrtx0())
            .field("esco2ptrtx1", &self.esco2ptrtx1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escocurrenttxptr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escocurrenttxptr2 {{ esco2ptrtx0: {=u16:?}, esco2ptrtx1: {=u16:?} }}",
            self.esco2ptrtx0(),
            self.esco2ptrtx1()
        )
    }
}
#[doc = "ESCODAYCNT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escodaycnt0(pub u32);
impl Escodaycnt0 {
    #[must_use]
    #[inline(always)]
    pub const fn dayconter0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_dayconter0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for Escodaycnt0 {
    #[inline(always)]
    fn default() -> Escodaycnt0 {
        Escodaycnt0(0)
    }
}
impl core::fmt::Debug for Escodaycnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escodaycnt0")
            .field("dayconter0", &self.dayconter0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escodaycnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escodaycnt0 {{ dayconter0: {=u16:?} }}",
            self.dayconter0()
        )
    }
}
#[doc = "ESCODAYCNT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escodaycnt1(pub u32);
impl Escodaycnt1 {
    #[must_use]
    #[inline(always)]
    pub const fn dayconter1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_dayconter1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for Escodaycnt1 {
    #[inline(always)]
    fn default() -> Escodaycnt1 {
        Escodaycnt1(0)
    }
}
impl core::fmt::Debug for Escodaycnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escodaycnt1")
            .field("dayconter1", &self.dayconter1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escodaycnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escodaycnt1 {{ dayconter1: {=u16:?} }}",
            self.dayconter1()
        )
    }
}
#[doc = "ESCODAYCNT2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escodaycnt2(pub u32);
impl Escodaycnt2 {
    #[must_use]
    #[inline(always)]
    pub const fn dayconter2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_dayconter2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for Escodaycnt2 {
    #[inline(always)]
    fn default() -> Escodaycnt2 {
        Escodaycnt2(0)
    }
}
impl core::fmt::Debug for Escodaycnt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escodaycnt2")
            .field("dayconter2", &self.dayconter2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escodaycnt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Escodaycnt2 {{ dayconter2: {=u16:?} }}",
            self.dayconter2()
        )
    }
}
#[doc = "ESCOLTCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escoltcntl0(pub u32);
impl Escoltcntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn synltaddr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_synltaddr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn syntype0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_syntype0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escoedrtx0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escoedrtx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escoedrrx0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escoedrrx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn retxnb0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_retxnb0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Escoltcntl0 {
    #[inline(always)]
    fn default() -> Escoltcntl0 {
        Escoltcntl0(0)
    }
}
impl core::fmt::Debug for Escoltcntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escoltcntl0")
            .field("synltaddr0", &self.synltaddr0())
            .field("syntype0", &self.syntype0())
            .field("escoedrtx0", &self.escoedrtx0())
            .field("escoedrrx0", &self.escoedrrx0())
            .field("retxnb0", &self.retxnb0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escoltcntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escoltcntl0 {{ synltaddr0: {=u8:?}, syntype0: {=bool:?}, escoedrtx0: {=bool:?}, escoedrrx0: {=bool:?}, retxnb0: {=u8:?} }}" , self . synltaddr0 () , self . syntype0 () , self . escoedrtx0 () , self . escoedrrx0 () , self . retxnb0 ())
    }
}
#[doc = "ESCOLTCNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escoltcntl1(pub u32);
impl Escoltcntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn synltaddr1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_synltaddr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn syntype1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_syntype1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escoedrtx1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escoedrtx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escoedrrx1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escoedrrx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn retxnb1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_retxnb1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Escoltcntl1 {
    #[inline(always)]
    fn default() -> Escoltcntl1 {
        Escoltcntl1(0)
    }
}
impl core::fmt::Debug for Escoltcntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escoltcntl1")
            .field("synltaddr1", &self.synltaddr1())
            .field("syntype1", &self.syntype1())
            .field("escoedrtx1", &self.escoedrtx1())
            .field("escoedrrx1", &self.escoedrrx1())
            .field("retxnb1", &self.retxnb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escoltcntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escoltcntl1 {{ synltaddr1: {=u8:?}, syntype1: {=bool:?}, escoedrtx1: {=bool:?}, escoedrrx1: {=bool:?}, retxnb1: {=u8:?} }}" , self . synltaddr1 () , self . syntype1 () , self . escoedrtx1 () , self . escoedrrx1 () , self . retxnb1 ())
    }
}
#[doc = "ESCOLTCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escoltcntl2(pub u32);
impl Escoltcntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn synltaddr2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_synltaddr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn syntype2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_syntype2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escoedrtx2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escoedrtx2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn escoedrrx2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_escoedrrx2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn retxnb2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_retxnb2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Escoltcntl2 {
    #[inline(always)]
    fn default() -> Escoltcntl2 {
        Escoltcntl2(0)
    }
}
impl core::fmt::Debug for Escoltcntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escoltcntl2")
            .field("synltaddr2", &self.synltaddr2())
            .field("syntype2", &self.syntype2())
            .field("escoedrtx2", &self.escoedrtx2())
            .field("escoedrrx2", &self.escoedrrx2())
            .field("retxnb2", &self.retxnb2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escoltcntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escoltcntl2 {{ synltaddr2: {=u8:?}, syntype2: {=bool:?}, escoedrtx2: {=bool:?}, escoedrrx2: {=bool:?}, retxnb2: {=u8:?} }}" , self . synltaddr2 () , self . syntype2 () , self . escoedrtx2 () , self . escoedrrx2 () , self . retxnb2 ())
    }
}
#[doc = "ESCOMUTECNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escomutecntl0(pub u32);
impl Escomutecntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn mutepatt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_mutepatt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn invl0_0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_invl0_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn invl0_1(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_invl0_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mute_source0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mute_source0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mute_sink0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mute_sink0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Escomutecntl0 {
    #[inline(always)]
    fn default() -> Escomutecntl0 {
        Escomutecntl0(0)
    }
}
impl core::fmt::Debug for Escomutecntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escomutecntl0")
            .field("mutepatt0", &self.mutepatt0())
            .field("invl0_0", &self.invl0_0())
            .field("invl0_1", &self.invl0_1())
            .field("mute_source0", &self.mute_source0())
            .field("mute_sink0", &self.mute_sink0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escomutecntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escomutecntl0 {{ mutepatt0: {=u16:?}, invl0_0: {=u8:?}, invl0_1: {=u8:?}, mute_source0: {=bool:?}, mute_sink0: {=bool:?} }}" , self . mutepatt0 () , self . invl0_0 () , self . invl0_1 () , self . mute_source0 () , self . mute_sink0 ())
    }
}
#[doc = "ESCOMUTECNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escomutecntl1(pub u32);
impl Escomutecntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn mutepatt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_mutepatt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn invl1_0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_invl1_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn invl1_1(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_invl1_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mute_source1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mute_source1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mute_sink1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mute_sink1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Escomutecntl1 {
    #[inline(always)]
    fn default() -> Escomutecntl1 {
        Escomutecntl1(0)
    }
}
impl core::fmt::Debug for Escomutecntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escomutecntl1")
            .field("mutepatt1", &self.mutepatt1())
            .field("invl1_0", &self.invl1_0())
            .field("invl1_1", &self.invl1_1())
            .field("mute_source1", &self.mute_source1())
            .field("mute_sink1", &self.mute_sink1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escomutecntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escomutecntl1 {{ mutepatt1: {=u16:?}, invl1_0: {=u8:?}, invl1_1: {=u8:?}, mute_source1: {=bool:?}, mute_sink1: {=bool:?} }}" , self . mutepatt1 () , self . invl1_0 () , self . invl1_1 () , self . mute_source1 () , self . mute_sink1 ())
    }
}
#[doc = "ESCOMUTECNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escomutecntl2(pub u32);
impl Escomutecntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn mutepatt2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_mutepatt2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn invl2_0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_invl2_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn invl2_1(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_invl2_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mute_source2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mute_source2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mute_sink2(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mute_sink2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Escomutecntl2 {
    #[inline(always)]
    fn default() -> Escomutecntl2 {
        Escomutecntl2(0)
    }
}
impl core::fmt::Debug for Escomutecntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escomutecntl2")
            .field("mutepatt2", &self.mutepatt2())
            .field("invl2_0", &self.invl2_0())
            .field("invl2_1", &self.invl2_1())
            .field("mute_source2", &self.mute_source2())
            .field("mute_sink2", &self.mute_sink2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escomutecntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escomutecntl2 {{ mutepatt2: {=u16:?}, invl2_0: {=u8:?}, invl2_1: {=u8:?}, mute_source2: {=bool:?}, mute_sink2: {=bool:?} }}" , self . mutepatt2 () , self . invl2_0 () , self . invl2_1 () , self . mute_source2 () , self . mute_sink2 ())
    }
}
#[doc = "ESCOTRCNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escotrcntl0(pub u32);
impl Escotrcntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn rxtype0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxtype0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxlen0(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rxlen0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 4usize)) | (((val as u32) & 0x03ff) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txtype0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txtype0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txlen0(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_txlen0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txseqn0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txseqn0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Escotrcntl0 {
    #[inline(always)]
    fn default() -> Escotrcntl0 {
        Escotrcntl0(0)
    }
}
impl core::fmt::Debug for Escotrcntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escotrcntl0")
            .field("rxtype0", &self.rxtype0())
            .field("rxlen0", &self.rxlen0())
            .field("txtype0", &self.txtype0())
            .field("txlen0", &self.txlen0())
            .field("txseqn0", &self.txseqn0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escotrcntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escotrcntl0 {{ rxtype0: {=u8:?}, rxlen0: {=u16:?}, txtype0: {=u8:?}, txlen0: {=u16:?}, txseqn0: {=bool:?} }}" , self . rxtype0 () , self . rxlen0 () , self . txtype0 () , self . txlen0 () , self . txseqn0 ())
    }
}
#[doc = "ESCOTRCNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escotrcntl1(pub u32);
impl Escotrcntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn rxtype1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxtype1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxlen1(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rxlen1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 4usize)) | (((val as u32) & 0x03ff) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txtype1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txtype1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txlen1(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_txlen1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txseqn1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txseqn1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Escotrcntl1 {
    #[inline(always)]
    fn default() -> Escotrcntl1 {
        Escotrcntl1(0)
    }
}
impl core::fmt::Debug for Escotrcntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escotrcntl1")
            .field("rxtype1", &self.rxtype1())
            .field("rxlen1", &self.rxlen1())
            .field("txtype1", &self.txtype1())
            .field("txlen1", &self.txlen1())
            .field("txseqn1", &self.txseqn1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escotrcntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escotrcntl1 {{ rxtype1: {=u8:?}, rxlen1: {=u16:?}, txtype1: {=u8:?}, txlen1: {=u16:?}, txseqn1: {=bool:?} }}" , self . rxtype1 () , self . rxlen1 () , self . txtype1 () , self . txlen1 () , self . txseqn1 ())
    }
}
#[doc = "ESCOTRCNTL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escotrcntl2(pub u32);
impl Escotrcntl2 {
    #[must_use]
    #[inline(always)]
    pub const fn rxtype2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxtype2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxlen2(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rxlen2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 4usize)) | (((val as u32) & 0x03ff) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txtype2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_txtype2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txlen2(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_txlen2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txseqn2(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_txseqn2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Escotrcntl2 {
    #[inline(always)]
    fn default() -> Escotrcntl2 {
        Escotrcntl2(0)
    }
}
impl core::fmt::Debug for Escotrcntl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Escotrcntl2")
            .field("rxtype2", &self.rxtype2())
            .field("rxlen2", &self.rxlen2())
            .field("txtype2", &self.txtype2())
            .field("txlen2", &self.txlen2())
            .field("txseqn2", &self.txseqn2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Escotrcntl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Escotrcntl2 {{ rxtype2: {=u8:?}, rxlen2: {=u16:?}, txtype2: {=u8:?}, txlen2: {=u16:?}, txseqn2: {=bool:?} }}" , self . rxtype2 () , self . rxlen2 () , self . txtype2 () , self . txlen2 () , self . txseqn2 ())
    }
}
#[doc = "ETPTR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etptr(pub u32);
impl Etptr {
    #[must_use]
    #[inline(always)]
    pub const fn etptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_etptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Etptr {
    #[inline(always)]
    fn default() -> Etptr {
        Etptr(0)
    }
}
impl core::fmt::Debug for Etptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etptr")
            .field("etptr", &self.etptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Etptr {{ etptr: {=u16:?} }}", self.etptr())
    }
}
#[doc = "FINECNTCORR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Finecntcorr(pub u32);
impl Finecntcorr {
    #[must_use]
    #[inline(always)]
    pub const fn finecntcorr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_finecntcorr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Finecntcorr {
    #[inline(always)]
    fn default() -> Finecntcorr {
        Finecntcorr(0)
    }
}
impl core::fmt::Debug for Finecntcorr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Finecntcorr")
            .field("finecntcorr", &self.finecntcorr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Finecntcorr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Finecntcorr {{ finecntcorr: {=u16:?} }}",
            self.finecntcorr()
        )
    }
}
#[doc = "FINETIMECNT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Finetimecnt(pub u32);
impl Finetimecnt {
    #[must_use]
    #[inline(always)]
    pub const fn finecnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_finecnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Finetimecnt {
    #[inline(always)]
    fn default() -> Finetimecnt {
        Finetimecnt(0)
    }
}
impl core::fmt::Debug for Finetimecnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Finetimecnt")
            .field("finecnt", &self.finecnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Finetimecnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Finetimecnt {{ finecnt: {=u16:?} }}", self.finecnt())
    }
}
#[doc = "FINETIMTGT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Finetimtgt(pub u32);
impl Finetimtgt {
    #[must_use]
    #[inline(always)]
    pub const fn finetarget(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_finetarget(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Finetimtgt {
    #[inline(always)]
    fn default() -> Finetimtgt {
        Finetimtgt(0)
    }
}
impl core::fmt::Debug for Finetimtgt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Finetimtgt")
            .field("finetarget", &self.finetarget())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Finetimtgt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Finetimtgt {{ finetarget: {=u32:?} }}",
            self.finetarget()
        )
    }
}
#[doc = "FREQ_CS2_SEED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqCs2Seed(pub u32);
impl FreqCs2Seed {
    #[must_use]
    #[inline(always)]
    pub const fn freqsel_evtcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_freqsel_evtcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn channel_identifier(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_channel_identifier(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FreqCs2Seed {
    #[inline(always)]
    fn default() -> FreqCs2Seed {
        FreqCs2Seed(0)
    }
}
impl core::fmt::Debug for FreqCs2Seed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqCs2Seed")
            .field("freqsel_evtcnt", &self.freqsel_evtcnt())
            .field("channel_identifier", &self.channel_identifier())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqCs2Seed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FreqCs2Seed {{ freqsel_evtcnt: {=u16:?}, channel_identifier: {=u16:?} }}",
            self.freqsel_evtcnt(),
            self.channel_identifier()
        )
    }
}
#[doc = "FREQSEL_CS1_SEED"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqselCs1Seed(pub u32);
impl FreqselCs1Seed {
    #[must_use]
    #[inline(always)]
    pub const fn freqsel_hopint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_freqsel_hopint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn freqsel_last_chidx(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_freqsel_last_chidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for FreqselCs1Seed {
    #[inline(always)]
    fn default() -> FreqselCs1Seed {
        FreqselCs1Seed(0)
    }
}
impl core::fmt::Debug for FreqselCs1Seed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqselCs1Seed")
            .field("freqsel_hopint", &self.freqsel_hopint())
            .field("freqsel_last_chidx", &self.freqsel_last_chidx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqselCs1Seed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FreqselCs1Seed {{ freqsel_hopint: {=u8:?}, freqsel_last_chidx: {=u8:?} }}",
            self.freqsel_hopint(),
            self.freqsel_last_chidx()
        )
    }
}
#[doc = "FREQSEL_LLCHMAP0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqselLlchmap0(pub u32);
impl FreqselLlchmap0 {
    #[must_use]
    #[inline(always)]
    pub const fn freqsel_llchamp0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_freqsel_llchamp0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FreqselLlchmap0 {
    #[inline(always)]
    fn default() -> FreqselLlchmap0 {
        FreqselLlchmap0(0)
    }
}
impl core::fmt::Debug for FreqselLlchmap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqselLlchmap0")
            .field("freqsel_llchamp0", &self.freqsel_llchamp0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqselLlchmap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FreqselLlchmap0 {{ freqsel_llchamp0: {=u32:?} }}",
            self.freqsel_llchamp0()
        )
    }
}
#[doc = "FREQSEL_LLCHMAP1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqselLlchmap1(pub u32);
impl FreqselLlchmap1 {
    #[must_use]
    #[inline(always)]
    pub const fn freqsel_llchmap1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_freqsel_llchmap1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for FreqselLlchmap1 {
    #[inline(always)]
    fn default() -> FreqselLlchmap1 {
        FreqselLlchmap1(0)
    }
}
impl core::fmt::Debug for FreqselLlchmap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqselLlchmap1")
            .field("freqsel_llchmap1", &self.freqsel_llchmap1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqselLlchmap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FreqselLlchmap1 {{ freqsel_llchmap1: {=u8:?} }}",
            self.freqsel_llchmap1()
        )
    }
}
#[doc = "FREQSELCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freqselcntl(pub u32);
impl Freqselcntl {
    #[must_use]
    #[inline(always)]
    pub const fn freqsel_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_freqsel_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn freqsel_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_freqsel_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nbloops(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nbloops(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Freqselcntl {
    #[inline(always)]
    fn default() -> Freqselcntl {
        Freqselcntl(0)
    }
}
impl core::fmt::Debug for Freqselcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Freqselcntl")
            .field("freqsel_start", &self.freqsel_start())
            .field("freqsel_mode", &self.freqsel_mode())
            .field("nbloops", &self.nbloops())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freqselcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Freqselcntl {{ freqsel_start: {=bool:?}, freqsel_mode: {=bool:?}, nbloops: {=u8:?} }}",
            self.freqsel_start(),
            self.freqsel_mode(),
            self.nbloops()
        )
    }
}
#[doc = "FREQSELPTR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freqselptr(pub u32);
impl Freqselptr {
    #[must_use]
    #[inline(always)]
    pub const fn freqselptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_freqselptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Freqselptr {
    #[inline(always)]
    fn default() -> Freqselptr {
        Freqselptr(0)
    }
}
impl core::fmt::Debug for Freqselptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Freqselptr")
            .field("freqselptr", &self.freqselptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freqselptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Freqselptr {{ freqselptr: {=u16:?} }}",
            self.freqselptr()
        )
    }
}
#[doc = "HMICROSECTGT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hmicrosectgt1(pub u32);
impl Hmicrosectgt1 {
    #[must_use]
    #[inline(always)]
    pub const fn hmicrosectarget(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_hmicrosectarget(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Hmicrosectgt1 {
    #[inline(always)]
    fn default() -> Hmicrosectgt1 {
        Hmicrosectgt1(0)
    }
}
impl core::fmt::Debug for Hmicrosectgt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hmicrosectgt1")
            .field("hmicrosectarget", &self.hmicrosectarget())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hmicrosectgt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hmicrosectgt1 {{ hmicrosectarget: {=u16:?} }}",
            self.hmicrosectarget()
        )
    }
}
#[doc = "HMICROSECTGT2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hmicrosectgt2(pub u32);
impl Hmicrosectgt2 {
    #[must_use]
    #[inline(always)]
    pub const fn hmicrosectarget(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_hmicrosectarget(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Hmicrosectgt2 {
    #[inline(always)]
    fn default() -> Hmicrosectgt2 {
        Hmicrosectgt2(0)
    }
}
impl core::fmt::Debug for Hmicrosectgt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hmicrosectgt2")
            .field("hmicrosectarget", &self.hmicrosectarget())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hmicrosectgt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hmicrosectgt2 {{ hmicrosectarget: {=u16:?} }}",
            self.hmicrosectarget()
        )
    }
}
#[doc = "HMICROSECTGT3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hmicrosectgt3(pub u32);
impl Hmicrosectgt3 {
    #[must_use]
    #[inline(always)]
    pub const fn hmicrosectarget(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_hmicrosectarget(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Hmicrosectgt3 {
    #[inline(always)]
    fn default() -> Hmicrosectgt3 {
        Hmicrosectgt3(0)
    }
}
impl core::fmt::Debug for Hmicrosectgt3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hmicrosectgt3")
            .field("hmicrosectarget", &self.hmicrosectarget())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hmicrosectgt3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hmicrosectgt3 {{ hmicrosectarget: {=u16:?} }}",
            self.hmicrosectarget()
        )
    }
}
#[doc = "ISOCNTCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isocntcntl(pub u32);
impl Isocntcntl {
    #[must_use]
    #[inline(always)]
    pub const fn isocorrmode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isocorrmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn iso_phase_shift_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_iso_phase_shift_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn iso_clkshift_mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_iso_clkshift_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn iso_upd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_iso_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn isosamp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isosamp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Isocntcntl {
    #[inline(always)]
    fn default() -> Isocntcntl {
        Isocntcntl(0)
    }
}
impl core::fmt::Debug for Isocntcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isocntcntl")
            .field("isocorrmode", &self.isocorrmode())
            .field("iso_phase_shift_mode", &self.iso_phase_shift_mode())
            .field("iso_clkshift_mode", &self.iso_clkshift_mode())
            .field("iso_upd", &self.iso_upd())
            .field("isosamp", &self.isosamp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isocntcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Isocntcntl {{ isocorrmode: {=bool:?}, iso_phase_shift_mode: {=bool:?}, iso_clkshift_mode: {=bool:?}, iso_upd: {=bool:?}, isosamp: {=bool:?} }}" , self . isocorrmode () , self . iso_phase_shift_mode () , self . iso_clkshift_mode () , self . iso_upd () , self . isosamp ())
    }
}
#[doc = "ISOCNTCORR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isocntcorr(pub u32);
impl Isocntcorr {
    #[must_use]
    #[inline(always)]
    pub const fn isocntcorr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isocntcorr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isocntcorr {
    #[inline(always)]
    fn default() -> Isocntcorr {
        Isocntcorr(0)
    }
}
impl core::fmt::Debug for Isocntcorr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isocntcorr")
            .field("isocntcorr", &self.isocntcorr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isocntcorr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isocntcorr {{ isocntcorr: {=u32:?} }}",
            self.isocntcorr()
        )
    }
}
#[doc = "ISOCNTSAMP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isocntsamp(pub u32);
impl Isocntsamp {
    #[must_use]
    #[inline(always)]
    pub const fn isocntsamp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isocntsamp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isocntsamp {
    #[inline(always)]
    fn default() -> Isocntsamp {
        Isocntsamp(0)
    }
}
impl core::fmt::Debug for Isocntsamp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isocntsamp")
            .field("isocntsamp", &self.isocntsamp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isocntsamp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isocntsamp {{ isocntsamp: {=u32:?} }}",
            self.isocntsamp()
        )
    }
}
#[doc = "ISOGPIOCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isogpiocntl(pub u32);
impl Isogpiocntl {
    #[must_use]
    #[inline(always)]
    pub const fn isogpiomsk(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_isogpiomsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn isocpiobeh(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isocpiobeh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Isogpiocntl {
    #[inline(always)]
    fn default() -> Isogpiocntl {
        Isogpiocntl(0)
    }
}
impl core::fmt::Debug for Isogpiocntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isogpiocntl")
            .field("isogpiomsk", &self.isogpiomsk())
            .field("isocpiobeh", &self.isocpiobeh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isogpiocntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isogpiocntl {{ isogpiomsk: {=u8:?}, isocpiobeh: {=bool:?} }}",
            self.isogpiomsk(),
            self.isocpiobeh()
        )
    }
}
#[doc = "ISOINTACK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isointack(pub u32);
impl Isointack {
    #[must_use]
    #[inline(always)]
    pub const fn isointack(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_isointack(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Isointack {
    #[inline(always)]
    fn default() -> Isointack {
        Isointack(0)
    }
}
impl core::fmt::Debug for Isointack {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isointack")
            .field("isointack", &self.isointack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isointack {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isointack {{ isointack: {=u8:?} }}", self.isointack())
    }
}
#[doc = "ISOINTCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isointcntl(pub u32);
impl Isointcntl {
    #[must_use]
    #[inline(always)]
    pub const fn isointmsk(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_isointmsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Isointcntl {
    #[inline(always)]
    fn default() -> Isointcntl {
        Isointcntl(0)
    }
}
impl core::fmt::Debug for Isointcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isointcntl")
            .field("isointmsk", &self.isointmsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isointcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isointcntl {{ isointmsk: {=u8:?} }}", self.isointmsk())
    }
}
#[doc = "ISOINTCORR_HUS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IsointcorrHus(pub u32);
impl IsointcorrHus {
    #[must_use]
    #[inline(always)]
    pub const fn isocntcorr_hus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isocntcorr_hus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IsointcorrHus {
    #[inline(always)]
    fn default() -> IsointcorrHus {
        IsointcorrHus(0)
    }
}
impl core::fmt::Debug for IsointcorrHus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IsointcorrHus")
            .field("isocntcorr_hus", &self.isocntcorr_hus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IsointcorrHus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IsointcorrHus {{ isocntcorr_hus: {=bool:?} }}",
            self.isocntcorr_hus()
        )
    }
}
#[doc = "ISOINTSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isointstat(pub u32);
impl Isointstat {
    #[must_use]
    #[inline(always)]
    pub const fn isointstat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_isointstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Isointstat {
    #[inline(always)]
    fn default() -> Isointstat {
        Isointstat(0)
    }
}
impl core::fmt::Debug for Isointstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isointstat")
            .field("isointstat", &self.isointstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isointstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isointstat {{ isointstat: {=u8:?} }}", self.isointstat())
    }
}
#[doc = "ISOTIMERTGT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isotimertgt0(pub u32);
impl Isotimertgt0 {
    #[must_use]
    #[inline(always)]
    pub const fn isotimertgt0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isotimertgt0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isotimertgt0 {
    #[inline(always)]
    fn default() -> Isotimertgt0 {
        Isotimertgt0(0)
    }
}
impl core::fmt::Debug for Isotimertgt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isotimertgt0")
            .field("isotimertgt0", &self.isotimertgt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isotimertgt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isotimertgt0 {{ isotimertgt0: {=u32:?} }}",
            self.isotimertgt0()
        )
    }
}
#[doc = "ISOTIMERTGT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isotimertgt1(pub u32);
impl Isotimertgt1 {
    #[must_use]
    #[inline(always)]
    pub const fn isotimertgt1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isotimertgt1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isotimertgt1 {
    #[inline(always)]
    fn default() -> Isotimertgt1 {
        Isotimertgt1(0)
    }
}
impl core::fmt::Debug for Isotimertgt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isotimertgt1")
            .field("isotimertgt1", &self.isotimertgt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isotimertgt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isotimertgt1 {{ isotimertgt1: {=u32:?} }}",
            self.isotimertgt1()
        )
    }
}
#[doc = "ISOTIMERTGT2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isotimertgt2(pub u32);
impl Isotimertgt2 {
    #[must_use]
    #[inline(always)]
    pub const fn isotimertgt2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isotimertgt2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isotimertgt2 {
    #[inline(always)]
    fn default() -> Isotimertgt2 {
        Isotimertgt2(0)
    }
}
impl core::fmt::Debug for Isotimertgt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isotimertgt2")
            .field("isotimertgt2", &self.isotimertgt2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isotimertgt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isotimertgt2 {{ isotimertgt2: {=u32:?} }}",
            self.isotimertgt2()
        )
    }
}
#[doc = "ISOTIMERTGT3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isotimertgt3(pub u32);
impl Isotimertgt3 {
    #[must_use]
    #[inline(always)]
    pub const fn isotimertgt3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isotimertgt3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isotimertgt3 {
    #[inline(always)]
    fn default() -> Isotimertgt3 {
        Isotimertgt3(0)
    }
}
impl core::fmt::Debug for Isotimertgt3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isotimertgt3")
            .field("isotimertgt3", &self.isotimertgt3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isotimertgt3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isotimertgt3 {{ isotimertgt3: {=u32:?} }}",
            self.isotimertgt3()
        )
    }
}
#[doc = "ISOTIMERTGT4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isotimertgt4(pub u32);
impl Isotimertgt4 {
    #[must_use]
    #[inline(always)]
    pub const fn isotimertgt4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isotimertgt4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isotimertgt4 {
    #[inline(always)]
    fn default() -> Isotimertgt4 {
        Isotimertgt4(0)
    }
}
impl core::fmt::Debug for Isotimertgt4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isotimertgt4")
            .field("isotimertgt4", &self.isotimertgt4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isotimertgt4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isotimertgt4 {{ isotimertgt4: {=u32:?} }}",
            self.isotimertgt4()
        )
    }
}
#[doc = "ISOTIMERTGT5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isotimertgt5(pub u32);
impl Isotimertgt5 {
    #[must_use]
    #[inline(always)]
    pub const fn isotimertgt5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isotimertgt5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isotimertgt5 {
    #[inline(always)]
    fn default() -> Isotimertgt5 {
        Isotimertgt5(0)
    }
}
impl core::fmt::Debug for Isotimertgt5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isotimertgt5")
            .field("isotimertgt5", &self.isotimertgt5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isotimertgt5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isotimertgt5 {{ isotimertgt5: {=u32:?} }}",
            self.isotimertgt5()
        )
    }
}
#[doc = "ISOTIMERTGT6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isotimertgt6(pub u32);
impl Isotimertgt6 {
    #[must_use]
    #[inline(always)]
    pub const fn isotimertgt6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isotimertgt6(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isotimertgt6 {
    #[inline(always)]
    fn default() -> Isotimertgt6 {
        Isotimertgt6(0)
    }
}
impl core::fmt::Debug for Isotimertgt6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isotimertgt6")
            .field("isotimertgt6", &self.isotimertgt6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isotimertgt6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isotimertgt6 {{ isotimertgt6: {=u32:?} }}",
            self.isotimertgt6()
        )
    }
}
#[doc = "ISOTIMERTGT7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isotimertgt7(pub u32);
impl Isotimertgt7 {
    #[must_use]
    #[inline(always)]
    pub const fn isotimertgt7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_isotimertgt7(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isotimertgt7 {
    #[inline(always)]
    fn default() -> Isotimertgt7 {
        Isotimertgt7(0)
    }
}
impl core::fmt::Debug for Isotimertgt7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isotimertgt7")
            .field("isotimertgt7", &self.isotimertgt7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isotimertgt7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isotimertgt7 {{ isotimertgt7: {=u32:?} }}",
            self.isotimertgt7()
        )
    }
}
#[doc = "PCACNTL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcacntl0(pub u32);
impl Pcacntl0 {
    #[must_use]
    #[inline(always)]
    pub const fn phase_shift_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_phase_shift_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sync_source(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sync_source(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn frsync_pol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frsync_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blindcorr_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_blindcorr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn corr_step(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_corr_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn slvlbl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_slvlbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn target_offset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_target_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Pcacntl0 {
    #[inline(always)]
    fn default() -> Pcacntl0 {
        Pcacntl0(0)
    }
}
impl core::fmt::Debug for Pcacntl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcacntl0")
            .field("phase_shift_en", &self.phase_shift_en())
            .field("sync_source", &self.sync_source())
            .field("frsync_pol", &self.frsync_pol())
            .field("blindcorr_en", &self.blindcorr_en())
            .field("corr_step", &self.corr_step())
            .field("slvlbl", &self.slvlbl())
            .field("target_offset", &self.target_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcacntl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pcacntl0 {{ phase_shift_en: {=bool:?}, sync_source: {=bool:?}, frsync_pol: {=bool:?}, blindcorr_en: {=bool:?}, corr_step: {=u8:?}, slvlbl: {=u8:?}, target_offset: {=u16:?} }}" , self . phase_shift_en () , self . sync_source () , self . frsync_pol () , self . blindcorr_en () , self . corr_step () , self . slvlbl () , self . target_offset ())
    }
}
#[doc = "PCACNTL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcacntl1(pub u32);
impl Pcacntl1 {
    #[must_use]
    #[inline(always)]
    pub const fn clock_shift(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_clock_shift(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clock_shift_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clock_shift_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn corr_interval(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_corr_interval(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Pcacntl1 {
    #[inline(always)]
    fn default() -> Pcacntl1 {
        Pcacntl1(0)
    }
}
impl core::fmt::Debug for Pcacntl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcacntl1")
            .field("clock_shift", &self.clock_shift())
            .field("clock_shift_en", &self.clock_shift_en())
            .field("corr_interval", &self.corr_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcacntl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pcacntl1 {{ clock_shift: {=u16:?}, clock_shift_en: {=bool:?}, corr_interval: {=u8:?} }}" , self . clock_shift () , self . clock_shift_en () , self . corr_interval ())
    }
}
#[doc = "PCASTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcastat(pub u32);
impl Pcastat {
    #[must_use]
    #[inline(always)]
    pub const fn moment_offset(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_moment_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn shift_phase(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_shift_phase(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Pcastat {
    #[inline(always)]
    fn default() -> Pcastat {
        Pcastat(0)
    }
}
impl core::fmt::Debug for Pcastat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcastat")
            .field("moment_offset", &self.moment_offset())
            .field("shift_phase", &self.shift_phase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcastat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcastat {{ moment_offset: {=u16:?}, shift_phase: {=u16:?} }}",
            self.moment_offset(),
            self.shift_phase()
        )
    }
}
#[doc = "PRIOSCHARB"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prioscharb(pub u32);
impl Prioscharb {
    #[must_use]
    #[inline(always)]
    pub const fn brpriomode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brpriomode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn blepriomode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_blepriomode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Prioscharb {
    #[inline(always)]
    fn default() -> Prioscharb {
        Prioscharb(0)
    }
}
impl core::fmt::Debug for Prioscharb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prioscharb")
            .field("brpriomode", &self.brpriomode())
            .field("blepriomode", &self.blepriomode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prioscharb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prioscharb {{ brpriomode: {=bool:?}, blepriomode: {=bool:?} }}",
            self.brpriomode(),
            self.blepriomode()
        )
    }
}
#[doc = "RAL_LOCAL_RND"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RalLocalRnd(pub u32);
impl RalLocalRnd {
    #[must_use]
    #[inline(always)]
    pub const fn lrnd_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_lrnd_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lrnd_init(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lrnd_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RalLocalRnd {
    #[inline(always)]
    fn default() -> RalLocalRnd {
        RalLocalRnd(0)
    }
}
impl core::fmt::Debug for RalLocalRnd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RalLocalRnd")
            .field("lrnd_val", &self.lrnd_val())
            .field("lrnd_init", &self.lrnd_init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RalLocalRnd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RalLocalRnd {{ lrnd_val: {=u32:?}, lrnd_init: {=bool:?} }}",
            self.lrnd_val(),
            self.lrnd_init()
        )
    }
}
#[doc = "RAL_PEER_RND"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RalPeerRnd(pub u32);
impl RalPeerRnd {
    #[must_use]
    #[inline(always)]
    pub const fn prnd_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_prnd_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn prnd_init(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_prnd_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RalPeerRnd {
    #[inline(always)]
    fn default() -> RalPeerRnd {
        RalPeerRnd(0)
    }
}
impl core::fmt::Debug for RalPeerRnd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RalPeerRnd")
            .field("prnd_val", &self.prnd_val())
            .field("prnd_init", &self.prnd_init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RalPeerRnd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RalPeerRnd {{ prnd_val: {=u32:?}, prnd_init: {=bool:?} }}",
            self.prnd_val(),
            self.prnd_init()
        )
    }
}
#[doc = "RALCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ralcntl(pub u32);
impl Ralcntl {
    #[must_use]
    #[inline(always)]
    pub const fn ralbaseptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ralbaseptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ralnbdev(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ralnbdev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Ralcntl {
    #[inline(always)]
    fn default() -> Ralcntl {
        Ralcntl(0)
    }
}
impl core::fmt::Debug for Ralcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ralcntl")
            .field("ralbaseptr", &self.ralbaseptr())
            .field("ralnbdev", &self.ralnbdev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ralcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ralcntl {{ ralbaseptr: {=u16:?}, ralnbdev: {=u8:?} }}",
            self.ralbaseptr(),
            self.ralnbdev()
        )
    }
}
#[doc = "RALCURRENT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ralcurrent(pub u32);
impl Ralcurrent {
    #[must_use]
    #[inline(always)]
    pub const fn ralcurrentptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ralcurrentptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ralcurrent {
    #[inline(always)]
    fn default() -> Ralcurrent {
        Ralcurrent(0)
    }
}
impl core::fmt::Debug for Ralcurrent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ralcurrent")
            .field("ralcurrentptr", &self.ralcurrentptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ralcurrent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ralcurrent {{ ralcurrentptr: {=u16:?} }}",
            self.ralcurrentptr()
        )
    }
}
#[doc = "RCCAL_CTRL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccalCtrl(pub u32);
impl RccalCtrl {
    #[must_use]
    #[inline(always)]
    pub const fn rccal_length(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rccal_length(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccal_auto(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccal_auto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccal_start(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccal_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccal_stop(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccal_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn con_num(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_con_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn con_mode(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_con_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RccalCtrl {
    #[inline(always)]
    fn default() -> RccalCtrl {
        RccalCtrl(0)
    }
}
impl core::fmt::Debug for RccalCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RccalCtrl")
            .field("rccal_length", &self.rccal_length())
            .field("rccal_auto", &self.rccal_auto())
            .field("rccal_start", &self.rccal_start())
            .field("rccal_stop", &self.rccal_stop())
            .field("con_num", &self.con_num())
            .field("con_mode", &self.con_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RccalCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RccalCtrl {{ rccal_length: {=u16:?}, rccal_auto: {=bool:?}, rccal_start: {=bool:?}, rccal_stop: {=bool:?}, con_num: {=u16:?}, con_mode: {=bool:?} }}" , self . rccal_length () , self . rccal_auto () , self . rccal_start () , self . rccal_stop () , self . con_num () , self . con_mode ())
    }
}
#[doc = "RCCAL_RESULT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RccalResult(pub u32);
impl RccalResult {
    #[must_use]
    #[inline(always)]
    pub const fn rccal_result(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_rccal_result(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccal_done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccal_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RccalResult {
    #[inline(always)]
    fn default() -> RccalResult {
        RccalResult(0)
    }
}
impl core::fmt::Debug for RccalResult {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RccalResult")
            .field("rccal_result", &self.rccal_result())
            .field("rccal_done", &self.rccal_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RccalResult {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RccalResult {{ rccal_result: {=u32:?}, rccal_done: {=bool:?} }}",
            self.rccal_result(),
            self.rccal_done()
        )
    }
}
#[doc = "RWBLECNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwblecntl(pub u32);
impl Rwblecntl {
    #[must_use]
    #[inline(always)]
    pub const fn rxwinszdef(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rxwinszdef(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rwble_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rwble_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn advertfilt_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_advertfilt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn anonymous_adv_filt_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_anonymous_adv_filt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxcteerr_rety_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rxcteerr_rety_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hop_remap_dsb(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hop_remap_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn crc_dsb(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_crc_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn whit_dsb(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_whit_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lrfec_dsb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lrfec_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lrpmap_dsb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lrpmap_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn crypt_dsb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_crypt_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nesn_dsb(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_nesn_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sn_dsb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sn_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn md_dsb(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_md_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn npi_dsb(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_npi_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cie_dsb(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cie_dsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn scan_abort(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_scan_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn advert_abort(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_advert_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rftest_abort(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rftest_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn master_soft_rst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_master_soft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rwblecntl {
    #[inline(always)]
    fn default() -> Rwblecntl {
        Rwblecntl(0)
    }
}
impl core::fmt::Debug for Rwblecntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rwblecntl")
            .field("rxwinszdef", &self.rxwinszdef())
            .field("rwble_en", &self.rwble_en())
            .field("advertfilt_en", &self.advertfilt_en())
            .field("anonymous_adv_filt_en", &self.anonymous_adv_filt_en())
            .field("rxcteerr_rety_en", &self.rxcteerr_rety_en())
            .field("hop_remap_dsb", &self.hop_remap_dsb())
            .field("crc_dsb", &self.crc_dsb())
            .field("whit_dsb", &self.whit_dsb())
            .field("lrfec_dsb", &self.lrfec_dsb())
            .field("lrpmap_dsb", &self.lrpmap_dsb())
            .field("crypt_dsb", &self.crypt_dsb())
            .field("nesn_dsb", &self.nesn_dsb())
            .field("sn_dsb", &self.sn_dsb())
            .field("md_dsb", &self.md_dsb())
            .field("npi_dsb", &self.npi_dsb())
            .field("cie_dsb", &self.cie_dsb())
            .field("scan_abort", &self.scan_abort())
            .field("advert_abort", &self.advert_abort())
            .field("rftest_abort", &self.rftest_abort())
            .field("master_soft_rst", &self.master_soft_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwblecntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rwblecntl {{ rxwinszdef: {=u8:?}, rwble_en: {=bool:?}, advertfilt_en: {=bool:?}, anonymous_adv_filt_en: {=bool:?}, rxcteerr_rety_en: {=bool:?}, hop_remap_dsb: {=bool:?}, crc_dsb: {=bool:?}, whit_dsb: {=bool:?}, lrfec_dsb: {=bool:?}, lrpmap_dsb: {=bool:?}, crypt_dsb: {=bool:?}, nesn_dsb: {=bool:?}, sn_dsb: {=bool:?}, md_dsb: {=bool:?}, npi_dsb: {=bool:?}, cie_dsb: {=bool:?}, scan_abort: {=bool:?}, advert_abort: {=bool:?}, rftest_abort: {=bool:?}, master_soft_rst: {=bool:?} }}" , self . rxwinszdef () , self . rwble_en () , self . advertfilt_en () , self . anonymous_adv_filt_en () , self . rxcteerr_rety_en () , self . hop_remap_dsb () , self . crc_dsb () , self . whit_dsb () , self . lrfec_dsb () , self . lrpmap_dsb () , self . crypt_dsb () , self . nesn_dsb () , self . sn_dsb () , self . md_dsb () , self . npi_dsb () , self . cie_dsb () , self . scan_abort () , self . advert_abort () , self . rftest_abort () , self . master_soft_rst ())
    }
}
#[doc = "RWBTCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwbtcntl(pub u32);
impl Rwbtcntl {
    #[must_use]
    #[inline(always)]
    pub const fn nwinsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nwinsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hpdly_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hpdly_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rwbten(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rwbten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cx_dnabort(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cx_dnabort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cx_rxbsyena(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cx_rxbsyena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cx_txbsyena(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cx_txbsyena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn seqndsb(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_seqndsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn arqndsb(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_arqndsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn flowdsb(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_flowdsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hopdsb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hopdsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn whitdsb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_whitdsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn crcdsb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_crcdsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cryptdsb(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cryptdsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lmpflowdsb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lmpflowdsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sniff_abort(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sniff_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pageing_abort(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pageing_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rftest_abort(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rftest_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn scan_abort(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_scan_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn master_soft_rst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_master_soft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rwbtcntl {
    #[inline(always)]
    fn default() -> Rwbtcntl {
        Rwbtcntl(0)
    }
}
impl core::fmt::Debug for Rwbtcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rwbtcntl")
            .field("nwinsize", &self.nwinsize())
            .field("hpdly_en", &self.hpdly_en())
            .field("rwbten", &self.rwbten())
            .field("cx_dnabort", &self.cx_dnabort())
            .field("cx_rxbsyena", &self.cx_rxbsyena())
            .field("cx_txbsyena", &self.cx_txbsyena())
            .field("seqndsb", &self.seqndsb())
            .field("arqndsb", &self.arqndsb())
            .field("flowdsb", &self.flowdsb())
            .field("hopdsb", &self.hopdsb())
            .field("whitdsb", &self.whitdsb())
            .field("crcdsb", &self.crcdsb())
            .field("cryptdsb", &self.cryptdsb())
            .field("lmpflowdsb", &self.lmpflowdsb())
            .field("sniff_abort", &self.sniff_abort())
            .field("pageing_abort", &self.pageing_abort())
            .field("rftest_abort", &self.rftest_abort())
            .field("scan_abort", &self.scan_abort())
            .field("master_soft_rst", &self.master_soft_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwbtcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rwbtcntl {{ nwinsize: {=u8:?}, hpdly_en: {=bool:?}, rwbten: {=bool:?}, cx_dnabort: {=bool:?}, cx_rxbsyena: {=bool:?}, cx_txbsyena: {=bool:?}, seqndsb: {=bool:?}, arqndsb: {=bool:?}, flowdsb: {=bool:?}, hopdsb: {=bool:?}, whitdsb: {=bool:?}, crcdsb: {=bool:?}, cryptdsb: {=bool:?}, lmpflowdsb: {=bool:?}, sniff_abort: {=bool:?}, pageing_abort: {=bool:?}, rftest_abort: {=bool:?}, scan_abort: {=bool:?}, master_soft_rst: {=bool:?} }}" , self . nwinsize () , self . hpdly_en () , self . rwbten () , self . cx_dnabort () , self . cx_rxbsyena () , self . cx_txbsyena () , self . seqndsb () , self . arqndsb () , self . flowdsb () , self . hopdsb () , self . whitdsb () , self . crcdsb () , self . cryptdsb () , self . lmpflowdsb () , self . sniff_abort () , self . pageing_abort () , self . rftest_abort () , self . scan_abort () , self . master_soft_rst ())
    }
}
#[doc = "RWDMCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwdmcntl(pub u32);
impl Rwdmcntl {
    #[must_use]
    #[inline(always)]
    pub const fn swint_req(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_swint_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn radiocntl_soft_rst(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_radiocntl_soft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn master_tgsoft_rst(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_master_tgsoft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn master_soft_rst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_master_soft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rwdmcntl {
    #[inline(always)]
    fn default() -> Rwdmcntl {
        Rwdmcntl(0)
    }
}
impl core::fmt::Debug for Rwdmcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rwdmcntl")
            .field("swint_req", &self.swint_req())
            .field("radiocntl_soft_rst", &self.radiocntl_soft_rst())
            .field("master_tgsoft_rst", &self.master_tgsoft_rst())
            .field("master_soft_rst", &self.master_soft_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwdmcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rwdmcntl {{ swint_req: {=bool:?}, radiocntl_soft_rst: {=bool:?}, master_tgsoft_rst: {=bool:?}, master_soft_rst: {=bool:?} }}" , self . swint_req () , self . radiocntl_soft_rst () , self . master_tgsoft_rst () , self . master_soft_rst ())
    }
}
#[doc = "RXMICVAL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxmicval(pub u32);
impl Rxmicval {
    #[must_use]
    #[inline(always)]
    pub const fn rxmicval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_rxmicval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxmicval {
    #[inline(always)]
    fn default() -> Rxmicval {
        Rxmicval(0)
    }
}
impl core::fmt::Debug for Rxmicval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxmicval")
            .field("rxmicval", &self.rxmicval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxmicval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxmicval {{ rxmicval: {=u32:?} }}", self.rxmicval())
    }
}
#[doc = "SEARCH_TIMEOUT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SearchTimeout(pub u32);
impl SearchTimeout {
    #[must_use]
    #[inline(always)]
    pub const fn search_timeout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_search_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for SearchTimeout {
    #[inline(always)]
    fn default() -> SearchTimeout {
        SearchTimeout(0)
    }
}
impl core::fmt::Debug for SearchTimeout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SearchTimeout")
            .field("search_timeout", &self.search_timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SearchTimeout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SearchTimeout {{ search_timeout: {=u8:?} }}",
            self.search_timeout()
        )
    }
}
#[doc = "SKIPEVTCLKN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Skipevtclkn(pub u32);
impl Skipevtclkn {
    #[must_use]
    #[inline(always)]
    pub const fn skipevtclknts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_skipevtclknts(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Skipevtclkn {
    #[inline(always)]
    fn default() -> Skipevtclkn {
        Skipevtclkn(0)
    }
}
impl core::fmt::Debug for Skipevtclkn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Skipevtclkn")
            .field("skipevtclknts", &self.skipevtclknts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Skipevtclkn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Skipevtclkn {{ skipevtclknts: {=u32:?} }}",
            self.skipevtclknts()
        )
    }
}
#[doc = "SKIPEVTFINECNT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Skipevtfinecnt(pub u32);
impl Skipevtfinecnt {
    #[must_use]
    #[inline(always)]
    pub const fn skipevtfinecntts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_skipevtfinecntts(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Skipevtfinecnt {
    #[inline(always)]
    fn default() -> Skipevtfinecnt {
        Skipevtfinecnt(0)
    }
}
impl core::fmt::Debug for Skipevtfinecnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Skipevtfinecnt")
            .field("skipevtfinecntts", &self.skipevtfinecntts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Skipevtfinecnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Skipevtfinecnt {{ skipevtfinecntts: {=u16:?} }}",
            self.skipevtfinecntts()
        )
    }
}
#[doc = "SKIPFRMCLKNTS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Skipfrmclknts(pub u32);
impl Skipfrmclknts {
    #[must_use]
    #[inline(always)]
    pub const fn skipfrmclknts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_skipfrmclknts(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Skipfrmclknts {
    #[inline(always)]
    fn default() -> Skipfrmclknts {
        Skipfrmclknts(0)
    }
}
impl core::fmt::Debug for Skipfrmclknts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Skipfrmclknts")
            .field("skipfrmclknts", &self.skipfrmclknts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Skipfrmclknts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Skipfrmclknts {{ skipfrmclknts: {=u32:?} }}",
            self.skipfrmclknts()
        )
    }
}
#[doc = "SKIPFRMFINECNTTS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Skipfrmfinecntts(pub u32);
impl Skipfrmfinecntts {
    #[must_use]
    #[inline(always)]
    pub const fn skipfrmfinecntts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_skipfrmfinecntts(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Skipfrmfinecntts {
    #[inline(always)]
    fn default() -> Skipfrmfinecntts {
        Skipfrmfinecntts(0)
    }
}
impl core::fmt::Debug for Skipfrmfinecntts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Skipfrmfinecntts")
            .field("skipfrmfinecntts", &self.skipfrmfinecntts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Skipfrmfinecntts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Skipfrmfinecntts {{ skipfrmfinecntts: {=u16:?} }}",
            self.skipfrmfinecntts()
        )
    }
}
#[doc = "SLOTCLK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slotclk(pub u32);
impl Slotclk {
    #[must_use]
    #[inline(always)]
    pub const fn sclk(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_sclk(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clkn_upd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clkn_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn samp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_samp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Slotclk {
    #[inline(always)]
    fn default() -> Slotclk {
        Slotclk(0)
    }
}
impl core::fmt::Debug for Slotclk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slotclk")
            .field("sclk", &self.sclk())
            .field("clkn_upd", &self.clkn_upd())
            .field("samp", &self.samp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slotclk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slotclk {{ sclk: {=u32:?}, clkn_upd: {=bool:?}, samp: {=bool:?} }}",
            self.sclk(),
            self.clkn_upd(),
            self.samp()
        )
    }
}
#[doc = "STARTEVTCLKN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Startevtclkn(pub u32);
impl Startevtclkn {
    #[must_use]
    #[inline(always)]
    pub const fn startevtclknts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_startevtclknts(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Startevtclkn {
    #[inline(always)]
    fn default() -> Startevtclkn {
        Startevtclkn(0)
    }
}
impl core::fmt::Debug for Startevtclkn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Startevtclkn")
            .field("startevtclknts", &self.startevtclknts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Startevtclkn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Startevtclkn {{ startevtclknts: {=u32:?} }}",
            self.startevtclknts()
        )
    }
}
#[doc = "STARTEVTFINECNT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Startevtfinecnt(pub u32);
impl Startevtfinecnt {
    #[must_use]
    #[inline(always)]
    pub const fn startevtfinecntts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_startevtfinecntts(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Startevtfinecnt {
    #[inline(always)]
    fn default() -> Startevtfinecnt {
        Startevtfinecnt(0)
    }
}
impl core::fmt::Debug for Startevtfinecnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Startevtfinecnt")
            .field("startevtfinecntts", &self.startevtfinecntts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Startevtfinecnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Startevtfinecnt {{ startevtfinecntts: {=u16:?} }}",
            self.startevtfinecntts()
        )
    }
}
#[doc = "STARTFRMCLKNTS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Startfrmclknts(pub u32);
impl Startfrmclknts {
    #[must_use]
    #[inline(always)]
    pub const fn startfrmclknts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_startfrmclknts(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Startfrmclknts {
    #[inline(always)]
    fn default() -> Startfrmclknts {
        Startfrmclknts(0)
    }
}
impl core::fmt::Debug for Startfrmclknts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Startfrmclknts")
            .field("startfrmclknts", &self.startfrmclknts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Startfrmclknts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Startfrmclknts {{ startfrmclknts: {=u32:?} }}",
            self.startfrmclknts()
        )
    }
}
#[doc = "STARTFRMFINECNTTS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Startfrmfinecntts(pub u32);
impl Startfrmfinecntts {
    #[must_use]
    #[inline(always)]
    pub const fn startfrmfinecntts(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_startfrmfinecntts(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Startfrmfinecntts {
    #[inline(always)]
    fn default() -> Startfrmfinecntts {
        Startfrmfinecntts(0)
    }
}
impl core::fmt::Debug for Startfrmfinecntts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Startfrmfinecntts")
            .field("startfrmfinecntts", &self.startfrmfinecntts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Startfrmfinecntts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Startfrmfinecntts {{ startfrmfinecntts: {=u16:?} }}",
            self.startfrmfinecntts()
        )
    }
}
#[doc = "TXMICVAL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txmicval(pub u32);
impl Txmicval {
    #[must_use]
    #[inline(always)]
    pub const fn txmicval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_txmicval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txmicval {
    #[inline(always)]
    fn default() -> Txmicval {
        Txmicval(0)
    }
}
impl core::fmt::Debug for Txmicval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txmicval")
            .field("txmicval", &self.txmicval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txmicval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txmicval {{ txmicval: {=u32:?} }}", self.txmicval())
    }
}
#[doc = "WPALCNTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wpalcntl(pub u32);
impl Wpalcntl {
    #[must_use]
    #[inline(always)]
    pub const fn wpalbaseptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_wpalbaseptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn wpalnbdev(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_wpalnbdev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Wpalcntl {
    #[inline(always)]
    fn default() -> Wpalcntl {
        Wpalcntl(0)
    }
}
impl core::fmt::Debug for Wpalcntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wpalcntl")
            .field("wpalbaseptr", &self.wpalbaseptr())
            .field("wpalnbdev", &self.wpalnbdev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wpalcntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wpalcntl {{ wpalbaseptr: {=u16:?}, wpalnbdev: {=u8:?} }}",
            self.wpalbaseptr(),
            self.wpalnbdev()
        )
    }
}
#[doc = "WPALCURRENT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wpalcurrent(pub u32);
impl Wpalcurrent {
    #[must_use]
    #[inline(always)]
    pub const fn wpalcurrentptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_wpalcurrentptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Wpalcurrent {
    #[inline(always)]
    fn default() -> Wpalcurrent {
        Wpalcurrent(0)
    }
}
impl core::fmt::Debug for Wpalcurrent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wpalcurrent")
            .field("wpalcurrentptr", &self.wpalcurrentptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wpalcurrent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wpalcurrent {{ wpalcurrentptr: {=u16:?} }}",
            self.wpalcurrentptr()
        )
    }
}
