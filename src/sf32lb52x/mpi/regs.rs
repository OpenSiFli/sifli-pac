#[doc = "Address Aliasing Ending Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aaear(pub u32);
impl Aaear {
    #[doc = "Ending address of the address aliasing area"]
    #[must_use]
    #[inline(always)]
    pub const fn ea(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Ending address of the address aliasing area"]
    #[inline(always)]
    pub const fn set_ea(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Aaear {
    #[inline(always)]
    fn default() -> Aaear {
        Aaear(0)
    }
}
impl core::fmt::Debug for Aaear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aaear").field("ea", &self.ea()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aaear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Aaear {{ ea: {=u32:?} }}", self.ea())
    }
}
#[doc = "Address Aliasing Offset Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aaoar(pub u32);
impl Aaoar {
    #[doc = "The offset to be added to the original address"]
    #[must_use]
    #[inline(always)]
    pub const fn oa(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "The offset to be added to the original address"]
    #[inline(always)]
    pub const fn set_oa(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Aaoar {
    #[inline(always)]
    fn default() -> Aaoar {
        Aaoar(0)
    }
}
impl core::fmt::Debug for Aaoar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aaoar").field("oa", &self.oa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aaoar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Aaoar {{ oa: {=u32:?} }}", self.oa())
    }
}
#[doc = "Address Aliasing Start Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aasar(pub u32);
impl Aasar {
    #[doc = "Starting address of the address aliasing area. Always 1KB aligned.Together with AAEAR, the aliasing area is \\[AASAR, AAEAR). If the address falls into this area, an offset AAOAR is added and the aliased address will be used to access external memory"]
    #[must_use]
    #[inline(always)]
    pub const fn sa(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Starting address of the address aliasing area. Always 1KB aligned.Together with AAEAR, the aliasing area is \\[AASAR, AAEAR). If the address falls into this area, an offset AAOAR is added and the aliased address will be used to access external memory"]
    #[inline(always)]
    pub const fn set_sa(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Aasar {
    #[inline(always)]
    fn default() -> Aasar {
        Aasar(0)
    }
}
impl core::fmt::Debug for Aasar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aasar").field("sa", &self.sa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aasar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Aasar {{ sa: {=u32:?} }}", self.sa())
    }
}
#[doc = "Alternate Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abr1(pub u32);
impl Abr1 {
    #[doc = "Alternate byte"]
    #[must_use]
    #[inline(always)]
    pub const fn abyte(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Alternate byte"]
    #[inline(always)]
    pub const fn set_abyte(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Abr1 {
    #[inline(always)]
    fn default() -> Abr1 {
        Abr1(0)
    }
}
impl core::fmt::Debug for Abr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Abr1")
            .field("abyte", &self.abyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Abr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Abr1 {{ abyte: {=u32:?} }}", self.abyte())
    }
}
#[doc = "Alternate Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abr2(pub u32);
impl Abr2 {
    #[doc = "Alternate byte in CMD2 sequence"]
    #[must_use]
    #[inline(always)]
    pub const fn abyte(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Alternate byte in CMD2 sequence"]
    #[inline(always)]
    pub const fn set_abyte(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Abr2 {
    #[inline(always)]
    fn default() -> Abr2 {
        Abr2(0)
    }
}
impl core::fmt::Debug for Abr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Abr2")
            .field("abyte", &self.abyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Abr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Abr2 {{ abyte: {=u32:?} }}", self.abyte())
    }
}
#[doc = "APM32 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apm32cr(pub u32);
impl Apm32cr {
    #[doc = "For special use by AP 32Mb PSRAM.Reserved-Do not modify"]
    #[must_use]
    #[inline(always)]
    pub const fn tcphr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "For special use by AP 32Mb PSRAM.Reserved-Do not modify"]
    #[inline(always)]
    pub const fn set_tcphr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "For special use by AP 32Mb PSRAM.Reserved-Do not modify"]
    #[must_use]
    #[inline(always)]
    pub const fn tcphw(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "For special use by AP 32Mb PSRAM.Reserved-Do not modify"]
    #[inline(always)]
    pub const fn set_tcphw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Apm32cr {
    #[inline(always)]
    fn default() -> Apm32cr {
        Apm32cr(0)
    }
}
impl core::fmt::Debug for Apm32cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apm32cr")
            .field("tcphr", &self.tcphr())
            .field("tcphw", &self.tcphw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apm32cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apm32cr {{ tcphr: {=u8:?}, tcphw: {=u8:?} }}",
            self.tcphr(),
            self.tcphw()
        )
    }
}
#[doc = "Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ar1(pub u32);
impl Ar1 {
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ar1 {
    #[inline(always)]
    fn default() -> Ar1 {
        Ar1(0)
    }
}
impl core::fmt::Debug for Ar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ar1").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ar1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ar1 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ar2(pub u32);
impl Ar2 {
    #[doc = "Address byte in CMD2 sequence"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address byte in CMD2 sequence"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ar2 {
    #[inline(always)]
    fn default() -> Ar2 {
        Ar2(0)
    }
}
impl core::fmt::Debug for Ar2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ar2").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ar2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ar2 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Calibration Clock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calcr(pub u32);
impl Calcr {
    #[doc = "calibration delay result"]
    #[must_use]
    #[inline(always)]
    pub const fn delay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "calibration delay result"]
    #[inline(always)]
    pub const fn set_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "calibration done flag"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "calibration done flag"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "calibration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "calibration enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Calcr {
    #[inline(always)]
    fn default() -> Calcr {
        Calcr(0)
    }
}
impl core::fmt::Debug for Calcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calcr")
            .field("delay", &self.delay())
            .field("done", &self.done())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calcr {{ delay: {=u8:?}, done: {=bool:?}, en: {=bool:?} }}",
            self.delay(),
            self.done(),
            self.en()
        )
    }
}
#[doc = "Communication Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "Instruction mode 0: no instruction phase 1: single line 2: dual lines 3: quad lines 4/5/6 - reserved 7 - quad lines DDR"]
    #[must_use]
    #[inline(always)]
    pub const fn imode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Instruction mode 0: no instruction phase 1: single line 2: dual lines 3: quad lines 4/5/6 - reserved 7 - quad lines DDR"]
    #[inline(always)]
    pub const fn set_imode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address mode 0: no address phase 1: single line 2: dual line 3: quad line 4/5/6: reserved 7: quad line DDR"]
    #[must_use]
    #[inline(always)]
    pub const fn admode(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Address mode 0: no address phase 1: single line 2: dual line 3: quad line 4/5/6: reserved 7: quad line DDR"]
    #[inline(always)]
    pub const fn set_admode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "Address size 0: one byte 1: two bytes 2: three bytes 3: four bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn adsize(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Address size 0: one byte 1: two bytes 2: three bytes 3: four bytes"]
    #[inline(always)]
    pub const fn set_adsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Alternate byte mode 0: no alternate byte 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR"]
    #[must_use]
    #[inline(always)]
    pub const fn abmode(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Alternate byte mode 0: no alternate byte 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR"]
    #[inline(always)]
    pub const fn set_abmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Alternate byte size 0: one byte 1: two bytes 2: three bytes 3: four bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn absize(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "Alternate byte size 0: one byte 1: two bytes 2: three bytes 3: four bytes"]
    #[inline(always)]
    pub const fn set_absize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Number of dummy cycles 0: no dummy cycle 1: one dummy cycle 2: two dummy cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn dcyc(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of dummy cycles 0: no dummy cycle 1: one dummy cycle 2: two dummy cycles"]
    #[inline(always)]
    pub const fn set_dcyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 13usize)) | (((val as u32) & 0x1f) << 13usize);
    }
    #[doc = "Data Mode 0: no data phase 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR"]
    #[must_use]
    #[inline(always)]
    pub const fn dmode(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Data Mode 0: no data phase 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR"]
    #[inline(always)]
    pub const fn set_dmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Function Mode 0: read mode 1: write mode"]
    #[must_use]
    #[inline(always)]
    pub const fn fmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Function Mode 0: read mode 1: write mode"]
    #[inline(always)]
    pub const fn set_fmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        Ccr1(0)
    }
}
impl core::fmt::Debug for Ccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr1")
            .field("imode", &self.imode())
            .field("admode", &self.admode())
            .field("adsize", &self.adsize())
            .field("abmode", &self.abmode())
            .field("absize", &self.absize())
            .field("dcyc", &self.dcyc())
            .field("dmode", &self.dmode())
            .field("fmode", &self.fmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccr1 {{ imode: {=u8:?}, admode: {=u8:?}, adsize: {=u8:?}, abmode: {=u8:?}, absize: {=u8:?}, dcyc: {=u8:?}, dmode: {=u8:?}, fmode: {=bool:?} }}" , self . imode () , self . admode () , self . adsize () , self . abmode () , self . absize () , self . dcyc () , self . dmode () , self . fmode ())
    }
}
#[doc = "Communication Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc = "This register specifies the format of CMD2 sequence. Refer to CCR1 description"]
    #[must_use]
    #[inline(always)]
    pub const fn imode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "This register specifies the format of CMD2 sequence. Refer to CCR1 description"]
    #[inline(always)]
    pub const fn set_imode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn admode(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_admode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adsize(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abmode(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_abmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn absize(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_absize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dcyc(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dcyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 13usize)) | (((val as u32) & 0x1f) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dmode(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn fmode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_fmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        Ccr2(0)
    }
}
impl core::fmt::Debug for Ccr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr2")
            .field("imode", &self.imode())
            .field("admode", &self.admode())
            .field("adsize", &self.adsize())
            .field("abmode", &self.abmode())
            .field("absize", &self.absize())
            .field("dcyc", &self.dcyc())
            .field("dmode", &self.dmode())
            .field("fmode", &self.fmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccr2 {{ imode: {=u8:?}, admode: {=u8:?}, adsize: {=u8:?}, abmode: {=u8:?}, absize: {=u8:?}, dcyc: {=u8:?}, dmode: {=u8:?}, fmode: {=bool:?} }}" , self . imode () , self . admode () , self . adsize () , self . abmode () , self . absize () , self . dcyc () , self . dmode () , self . fmode ())
    }
}
#[doc = "Command Interval Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cir(pub u32);
impl Cir {
    #[doc = "The interval between CMD1 itself. The unit is in MCLK cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn interval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The interval between CMD1 itself. The unit is in MCLK cycles"]
    #[inline(always)]
    pub const fn set_interval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The interval between CMD1 and CMD2 (or between CMD2 itself) if CMD2E is enabled. The unit is in MCLK cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn interval2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "The interval between CMD1 and CMD2 (or between CMD2 itself) if CMD2E is enabled. The unit is in MCLK cycles"]
    #[inline(always)]
    pub const fn set_interval2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cir {
    #[inline(always)]
    fn default() -> Cir {
        Cir(0)
    }
}
impl core::fmt::Debug for Cir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cir")
            .field("interval1", &self.interval1())
            .field("interval2", &self.interval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cir {{ interval1: {=u16:?}, interval2: {=u16:?} }}",
            self.interval1(),
            self.interval2()
        )
    }
}
#[doc = "Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdr1(pub u32);
impl Cmdr1 {
    #[doc = "Command. Write to this register will trigger the sequence specified in CCR1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Command. Write to this register will trigger the sequence specified in CCR1"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cmdr1 {
    #[inline(always)]
    fn default() -> Cmdr1 {
        Cmdr1(0)
    }
}
impl core::fmt::Debug for Cmdr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdr1").field("cmd", &self.cmd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmdr1 {{ cmd: {=u8:?} }}", self.cmd())
    }
}
#[doc = "Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdr2(pub u32);
impl Cmdr2 {
    #[doc = "Command 2. If CMD2E is enabled, the CMD2 sequence will be issued after CMD1 as specified in CCR2 Note: CMD2 sequence cannot be issue individually"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Command 2. If CMD2E is enabled, the CMD2 sequence will be issued after CMD1 as specified in CCR2 Note: CMD2 sequence cannot be issue individually"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cmdr2 {
    #[inline(always)]
    fn default() -> Cmdr2 {
        Cmdr2(0)
    }
}
impl core::fmt::Debug for Cmdr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdr2").field("cmd", &self.cmd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmdr2 {{ cmd: {=u8:?} }}", self.cmd())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Enable MPI"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable MPI"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable WP function on IO2. Use this only in SPI or Dual SPI mode"]
    #[must_use]
    #[inline(always)]
    pub const fn wpe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable WP function on IO2. Use this only in SPI or Dual SPI mode"]
    #[inline(always)]
    pub const fn set_wpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "The value of WP when WPE is set"]
    #[must_use]
    #[inline(always)]
    pub const fn wp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "The value of WP when WPE is set"]
    #[inline(always)]
    pub const fn set_wp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable HOLD function on IO3. Use this only in SPI or Dual SPI mode"]
    #[must_use]
    #[inline(always)]
    pub const fn holde(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable HOLD function on IO3. Use this only in SPI or Dual SPI mode"]
    #[inline(always)]
    pub const fn set_holde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The value of HOLD when HOLDE is set"]
    #[must_use]
    #[inline(always)]
    pub const fn hold(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "The value of HOLD when HOLDE is set"]
    #[inline(always)]
    pub const fn set_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA enable 0: disabled 1: enable DMA to read or write DR register"]
    #[must_use]
    #[inline(always)]
    pub const fn dmae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable 0: disabled 1: enable DMA to read or write DR register"]
    #[inline(always)]
    pub const fn set_dmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "AES-CTR on-the-fly decryption enable 0: disabled 1: enabled, data read from memory will be decrypted on the fly by MPI controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ctre(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "AES-CTR on-the-fly decryption enable 0: disabled 1: enabled, data read from memory will be decrypted on the fly by MPI controller"]
    #[inline(always)]
    pub const fn set_ctre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "AES-CTR mode 0: AES-128 1: AES-256"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AES-CTR mode 0: AES-128 1: AES-256"]
    #[inline(always)]
    pub const fn set_ctrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Status match interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Status match interrupt enable"]
    #[inline(always)]
    pub const fn set_smie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CS max violation interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn csvie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CS max violation interrupt enable"]
    #[inline(always)]
    pub const fn set_csvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Row boundary crossing interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rbxie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Row boundary crossing interrupt enable"]
    #[inline(always)]
    pub const fn set_rbxie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable CMD2 0: disabled 1: CMD2 is enabled and will be issued after CMD1 with an interval of TI2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd2e(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable CMD2 0: disabled 1: CMD2 is enabled and will be issued after CMD1 with an interval of TI2"]
    #[inline(always)]
    pub const fn set_cmd2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status match enable. If enabled, CMD1 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled (either SME1 or SME2 can be enabled, and SME1 has high priority)"]
    #[must_use]
    #[inline(always)]
    pub const fn sme1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status match enable. If enabled, CMD1 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled (either SME1 or SME2 can be enabled, and SME1 has high priority)"]
    #[inline(always)]
    pub const fn set_sme1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Status match enable. If enabled, CMD2 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn sme2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Status match enable. If enabled, CMD2 will be issued repeatedly until the data match the value in SMR and SMKR 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn set_sme2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Status match mode 0: AND mode 1: OR mode"]
    #[must_use]
    #[inline(always)]
    pub const fn smm(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Status match mode 0: AND mode 1: OR mode"]
    #[inline(always)]
    pub const fn set_smm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Hardware interface enableReserved-Do not modify"]
    #[must_use]
    #[inline(always)]
    pub const fn hwife(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware interface enableReserved-Do not modify"]
    #[inline(always)]
    pub const fn set_hwife(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OPI interface enable 0: x8 mode disabled 1: x8 mode enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn opie(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "OPI interface enable 0: x8 mode disabled 1: x8 mode enabled"]
    #[inline(always)]
    pub const fn set_opie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Prefetch enable. If enabled, MPI will prefetch at consequtive address following a read transaction. Recommend to use when reading large data in a burst manner. 0: prefetch disabled 1: prefetch enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn prefe(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable. If enabled, MPI will prefetch at consequtive address following a read transaction. Recommend to use when reading large data in a burst manner. 0: prefetch disabled 1: prefetch enabled"]
    #[inline(always)]
    pub const fn set_prefe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Mode X16Reserved-Do not modify"]
    #[must_use]
    #[inline(always)]
    pub const fn mx16(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Mode X16Reserved-Do not modify"]
    #[inline(always)]
    pub const fn set_mx16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Dual Flash ModeReserved-Do not modify"]
    #[must_use]
    #[inline(always)]
    pub const fn dfm(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Dual Flash ModeReserved-Do not modify"]
    #[inline(always)]
    pub const fn set_dfm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Hold hreadyout low if AHB access"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbdis(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Hold hreadyout low if AHB access"]
    #[inline(always)]
    pub const fn set_ahbdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write 1 to abort internal state machine. For debug purpose only"]
    #[must_use]
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to abort internal state machine. For debug purpose only"]
    #[inline(always)]
    pub const fn set_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("en", &self.en())
            .field("wpe", &self.wpe())
            .field("wp", &self.wp())
            .field("holde", &self.holde())
            .field("hold", &self.hold())
            .field("dmae", &self.dmae())
            .field("ctre", &self.ctre())
            .field("ctrm", &self.ctrm())
            .field("tcie", &self.tcie())
            .field("smie", &self.smie())
            .field("csvie", &self.csvie())
            .field("rbxie", &self.rbxie())
            .field("cmd2e", &self.cmd2e())
            .field("sme1", &self.sme1())
            .field("sme2", &self.sme2())
            .field("smm", &self.smm())
            .field("hwife", &self.hwife())
            .field("opie", &self.opie())
            .field("prefe", &self.prefe())
            .field("mx16", &self.mx16())
            .field("dfm", &self.dfm())
            .field("ahbdis", &self.ahbdis())
            .field("abort", &self.abort())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cr {{ en: {=bool:?}, wpe: {=bool:?}, wp: {=bool:?}, holde: {=bool:?}, hold: {=bool:?}, dmae: {=bool:?}, ctre: {=bool:?}, ctrm: {=bool:?}, tcie: {=bool:?}, smie: {=bool:?}, csvie: {=bool:?}, rbxie: {=bool:?}, cmd2e: {=bool:?}, sme1: {=bool:?}, sme2: {=bool:?}, smm: {=bool:?}, hwife: {=bool:?}, opie: {=bool:?}, prefe: {=bool:?}, mx16: {=bool:?}, dfm: {=bool:?}, ahbdis: {=bool:?}, abort: {=bool:?} }}" , self . en () , self . wpe () , self . wp () , self . holde () , self . hold () , self . dmae () , self . ctre () , self . ctrm () , self . tcie () , self . smie () , self . csvie () , self . rbxie () , self . cmd2e () , self . sme1 () , self . sme2 () , self . smm () , self . hwife () , self . opie () , self . prefe () , self . mx16 () , self . dfm () , self . ahbdis () , self . abort ())
    }
}
#[doc = "Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Repeat CMD1->CMD2 sequence for n times. This filed is only valid when CMD2E=1 and SME2=0. For example if LOOP=0, then the sequence is CMD1 -> CMD2. If LOOP=2, then the sequence is (CMD1->CMD2) -> (CMD1->CMD2) -> (CMD1->CMD2)"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Repeat CMD1->CMD2 sequence for n times. This filed is only valid when CMD2E=1 and SME2=0. For example if LOOP=0, then the sequence is CMD1 -> CMD2. If LOOP=2, then the sequence is (CMD1->CMD2) -> (CMD1->CMD2) -> (CMD1->CMD2)"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        f.debug_struct("Cr2").field("loop_", &self.loop_()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cr2 {{ loop_: {=u8:?} }}", self.loop_())
    }
}
#[doc = "CTR Ending Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrear(pub u32);
impl Ctrear {
    #[doc = "Ending address of the AES decryption area"]
    #[must_use]
    #[inline(always)]
    pub const fn ea(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Ending address of the AES decryption area"]
    #[inline(always)]
    pub const fn set_ea(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Ctrear {
    #[inline(always)]
    fn default() -> Ctrear {
        Ctrear(0)
    }
}
impl core::fmt::Debug for Ctrear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrear").field("ea", &self.ea()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctrear {{ ea: {=u32:?} }}", self.ea())
    }
}
#[doc = "CTR Starting Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrsar(pub u32);
impl Ctrsar {
    #[doc = "Starting address of the AES decryption area. Since the lowest 10 bits are zero, the address is always 1KB aligned. Together with CTREAR, the total area is \\[CTRSAR, CTREAR) For example, CTRSAR = 32'h0, CTREAR = 32'h200000, then the on-the-fly decryption area is 0x0 - 0x1FFFFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sa(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Starting address of the AES decryption area. Since the lowest 10 bits are zero, the address is always 1KB aligned. Together with CTREAR, the total area is \\[CTRSAR, CTREAR) For example, CTRSAR = 32'h0, CTREAR = 32'h200000, then the on-the-fly decryption area is 0x0 - 0x1FFFFF"]
    #[inline(always)]
    pub const fn set_sa(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Ctrsar {
    #[inline(always)]
    fn default() -> Ctrsar {
        Ctrsar(0)
    }
}
impl core::fmt::Debug for Ctrsar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrsar").field("sa", &self.sa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrsar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctrsar {{ sa: {=u32:?} }}", self.sa())
    }
}
#[doc = "Device Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc = "Row boundary size. 0: no row boundary 1: 2^(1+3) = 16 bytes 2: 2^(2+3) = 32 bytes ... n: 2^(n+3) bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn rbsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Row boundary size. 0: no row boundary 1: 2^(1+3) = 16 bytes 2: 2^(2+3) = 32 bytes ... n: 2^(n+3) bytes"]
    #[inline(always)]
    pub const fn set_rbsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "DQS enable. Setting to 1 indicates device provides DQS signal for Rx data latching"]
    #[must_use]
    #[inline(always)]
    pub const fn dqse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DQS enable. Setting to 1 indicates device provides DQS signal for Rx data latching"]
    #[inline(always)]
    pub const fn set_dqse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HyperBus protocol. Set to 1 for HyperRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn hyper(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HyperBus protocol. Set to 1 for HyperRAM."]
    #[inline(always)]
    pub const fn set_hyper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Xccela legacy protocol. Set to 1 for AP 32Mb PSRAM only, othersize always set to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn xlegacy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Xccela legacy protocol. Set to 1 for AP 32Mb PSRAM only, othersize always set to 0."]
    #[inline(always)]
    pub const fn set_xlegacy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Maximum CS low active time in MCLK cycles For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMAX = n, then CS Low time = (n+1) * 1000/240 ns which must meet the maximum tCEM requirement for PSRAM"]
    #[must_use]
    #[inline(always)]
    pub const fn cslmax(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum CS low active time in MCLK cycles For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMAX = n, then CS Low time = (n+1) * 1000/240 ns which must meet the maximum tCEM requirement for PSRAM"]
    #[inline(always)]
    pub const fn set_cslmax(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 6usize)) | (((val as u32) & 0x0fff) << 6usize);
    }
    #[doc = "Minimum CS low active time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMIN = n, then CS Low time = (n+1) * 1000/240 ns which must meet the minimum tCEM requirement for PSRAM"]
    #[must_use]
    #[inline(always)]
    pub const fn cslmin(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Minimum CS low active time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSLMIN = n, then CS Low time = (n+1) * 1000/240 ns which must meet the minimum tCEM requirement for PSRAM"]
    #[inline(always)]
    pub const fn set_cslmin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Minimum CS high deselect time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSHMIN = n, then CS High time = (n+1) * 1000/240 ns which must meet minimum tCPH requirement for PSRAM"]
    #[must_use]
    #[inline(always)]
    pub const fn cshmin(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "Minimum CS high deselect time in MCLK cycles. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and CSHMIN = n, then CS High time = (n+1) * 1000/240 ns which must meet minimum tCPH requirement for PSRAM"]
    #[inline(always)]
    pub const fn set_cshmin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Write/Read cycle minimum time in internal MCLK cycles. Please see MCLK frequency in PSCLR description. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and TRCMIN = n, then tRC time = (n+1) * 1000/240 ns which must meet minimum tRC requirement for PSRAM"]
    #[must_use]
    #[inline(always)]
    pub const fn trcmin(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Write/Read cycle minimum time in internal MCLK cycles. Please see MCLK frequency in PSCLR description. For example, if PSRAM clock is 120MHz (i.e. internal MCLK is 240MHz) and TRCMIN = n, then tRC time = (n+1) * 1000/240 ns which must meet minimum tRC requirement for PSRAM"]
    #[inline(always)]
    pub const fn set_trcmin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Indicate PSRAM is fixed latency or variable latency. It must be compatible to the configuration in PSRAM registers. Recommend always set to 1. 0: variable latency 1: fixed latency"]
    #[must_use]
    #[inline(always)]
    pub const fn fixlat(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicate PSRAM is fixed latency or variable latency. It must be compatible to the configuration in PSRAM registers. Recommend always set to 1. 0: variable latency 1: fixed latency"]
    #[inline(always)]
    pub const fn set_fixlat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("rbsize", &self.rbsize())
            .field("dqse", &self.dqse())
            .field("hyper", &self.hyper())
            .field("xlegacy", &self.xlegacy())
            .field("cslmax", &self.cslmax())
            .field("cslmin", &self.cslmin())
            .field("cshmin", &self.cshmin())
            .field("trcmin", &self.trcmin())
            .field("fixlat", &self.fixlat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcr {{ rbsize: {=u8:?}, dqse: {=bool:?}, hyper: {=bool:?}, xlegacy: {=bool:?}, cslmax: {=u16:?}, cslmin: {=u8:?}, cshmin: {=u8:?}, trcmin: {=u8:?}, fixlat: {=bool:?} }}" , self . rbsize () , self . dqse () , self . hyper () , self . xlegacy () , self . cslmax () , self . cslmin () , self . cshmin () , self . trcmin () , self . fixlat ())
    }
}
#[doc = "Data Length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlr1(pub u32);
impl Dlr1 {
    #[doc = "Data length 0: one byte 1: two bytes ... n: (n+1) bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn dlen(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Data length 0: one byte 1: two bytes ... n: (n+1) bytes"]
    #[inline(always)]
    pub const fn set_dlen(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Dlr1 {
    #[inline(always)]
    fn default() -> Dlr1 {
        Dlr1(0)
    }
}
impl core::fmt::Debug for Dlr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dlr1").field("dlen", &self.dlen()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dlr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dlr1 {{ dlen: {=u32:?} }}", self.dlen())
    }
}
#[doc = "Data Length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlr2(pub u32);
impl Dlr2 {
    #[doc = "Data length in CMD2 sequence"]
    #[must_use]
    #[inline(always)]
    pub const fn dlen(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Data length in CMD2 sequence"]
    #[inline(always)]
    pub const fn set_dlen(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Dlr2 {
    #[inline(always)]
    fn default() -> Dlr2 {
        Dlr2(0)
    }
}
impl core::fmt::Debug for Dlr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dlr2").field("dlen", &self.dlen()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dlr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dlr2 {{ dlen: {=u32:?} }}", self.dlen())
    }
}
#[doc = "Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "The entry of internal data FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The entry of internal data FIFO"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifocr(pub u32);
impl Fifocr {
    #[doc = "write 1 to clear Rx FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn rxclr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear Rx FIFO"]
    #[inline(always)]
    pub const fn set_rxclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO empty"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to clear Tx FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn txclr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear Tx FIFO"]
    #[inline(always)]
    pub const fn set_txclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Tx FIFO full flag"]
    #[must_use]
    #[inline(always)]
    pub const fn txf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO full flag"]
    #[inline(always)]
    pub const fn set_txf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "When DMA enabled, asserts DMA reqeust if TXFIFO vacant slots is greater than or equal to TXSLOTS. Note: this field should be set in accordance to the burst length in DMA. For example, if DMA employs BURST8 transction, then this filed is set to 8"]
    #[must_use]
    #[inline(always)]
    pub const fn txslots(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "When DMA enabled, asserts DMA reqeust if TXFIFO vacant slots is greater than or equal to TXSLOTS. Note: this field should be set in accordance to the burst length in DMA. For example, if DMA employs BURST8 transction, then this filed is set to 8"]
    #[inline(always)]
    pub const fn set_txslots(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
}
impl Default for Fifocr {
    #[inline(always)]
    fn default() -> Fifocr {
        Fifocr(0)
    }
}
impl core::fmt::Debug for Fifocr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifocr")
            .field("rxclr", &self.rxclr())
            .field("rxe", &self.rxe())
            .field("txclr", &self.txclr())
            .field("txf", &self.txf())
            .field("txslots", &self.txslots())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifocr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Fifocr {{ rxclr: {=bool:?}, rxe: {=bool:?}, txclr: {=bool:?}, txf: {=bool:?}, txslots: {=u8:?} }}" , self . rxclr () , self . rxe () , self . txclr () , self . txf () , self . txslots ())
    }
}
#[doc = "AHB Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcmdr(pub u32);
impl Hcmdr {
    #[doc = "AHB read command. During XIP, the AHB read transaction will be translated into this Read Command on memory interface"]
    #[must_use]
    #[inline(always)]
    pub const fn rcmd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB read command. During XIP, the AHB read transaction will be translated into this Read Command on memory interface"]
    #[inline(always)]
    pub const fn set_rcmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB write command. During XIP, the AHB write transaction will be translated into this Write Command on memory interface"]
    #[must_use]
    #[inline(always)]
    pub const fn wcmd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "AHB write command. During XIP, the AHB write transaction will be translated into this Write Command on memory interface"]
    #[inline(always)]
    pub const fn set_wcmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Hcmdr {
    #[inline(always)]
    fn default() -> Hcmdr {
        Hcmdr(0)
    }
}
impl core::fmt::Debug for Hcmdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcmdr")
            .field("rcmd", &self.rcmd())
            .field("wcmd", &self.wcmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcmdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcmdr {{ rcmd: {=u8:?}, wcmd: {=u8:?} }}",
            self.rcmd(),
            self.wcmd()
        )
    }
}
#[doc = "AHB Read Alternate Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrabr(pub u32);
impl Hrabr {
    #[must_use]
    #[inline(always)]
    pub const fn abyte(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_abyte(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Hrabr {
    #[inline(always)]
    fn default() -> Hrabr {
        Hrabr(0)
    }
}
impl core::fmt::Debug for Hrabr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hrabr")
            .field("abyte", &self.abyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hrabr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hrabr {{ abyte: {=u32:?} }}", self.abyte())
    }
}
#[doc = "AHB Read Communication Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrccr(pub u32);
impl Hrccr {
    #[doc = "This register specifies the format of AHB read command sequence. Refer to CCR1 description"]
    #[must_use]
    #[inline(always)]
    pub const fn imode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "This register specifies the format of AHB read command sequence. Refer to CCR1 description"]
    #[inline(always)]
    pub const fn set_imode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn admode(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_admode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adsize(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abmode(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_abmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn absize(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_absize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dcyc(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dcyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 13usize)) | (((val as u32) & 0x1f) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dmode(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
}
impl Default for Hrccr {
    #[inline(always)]
    fn default() -> Hrccr {
        Hrccr(0)
    }
}
impl core::fmt::Debug for Hrccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hrccr")
            .field("imode", &self.imode())
            .field("admode", &self.admode())
            .field("adsize", &self.adsize())
            .field("abmode", &self.abmode())
            .field("absize", &self.absize())
            .field("dcyc", &self.dcyc())
            .field("dmode", &self.dmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hrccr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hrccr {{ imode: {=u8:?}, admode: {=u8:?}, adsize: {=u8:?}, abmode: {=u8:?}, absize: {=u8:?}, dcyc: {=u8:?}, dmode: {=u8:?} }}" , self . imode () , self . admode () , self . adsize () , self . abmode () , self . absize () , self . dcyc () , self . dmode ())
    }
}
#[doc = "AHB Write Alternate Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwabr(pub u32);
impl Hwabr {
    #[must_use]
    #[inline(always)]
    pub const fn abyte(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_abyte(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Hwabr {
    #[inline(always)]
    fn default() -> Hwabr {
        Hwabr(0)
    }
}
impl core::fmt::Debug for Hwabr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hwabr")
            .field("abyte", &self.abyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hwabr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hwabr {{ abyte: {=u32:?} }}", self.abyte())
    }
}
#[doc = "AHB Write Communication Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwccr(pub u32);
impl Hwccr {
    #[doc = "This register specifies the format of AHB write command sequence. Refer to CCR1 description"]
    #[must_use]
    #[inline(always)]
    pub const fn imode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "This register specifies the format of AHB write command sequence. Refer to CCR1 description"]
    #[inline(always)]
    pub const fn set_imode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn admode(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_admode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adsize(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn abmode(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_abmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn absize(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_absize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dcyc(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dcyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 13usize)) | (((val as u32) & 0x1f) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dmode(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
}
impl Default for Hwccr {
    #[inline(always)]
    fn default() -> Hwccr {
        Hwccr(0)
    }
}
impl core::fmt::Debug for Hwccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hwccr")
            .field("imode", &self.imode())
            .field("admode", &self.admode())
            .field("adsize", &self.adsize())
            .field("abmode", &self.abmode())
            .field("absize", &self.absize())
            .field("dcyc", &self.dcyc())
            .field("dmode", &self.dmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hwccr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Hwccr {{ imode: {=u8:?}, admode: {=u8:?}, adsize: {=u8:?}, abmode: {=u8:?}, absize: {=u8:?}, dcyc: {=u8:?}, dmode: {=u8:?} }}" , self . imode () , self . admode () , self . adsize () , self . abmode () , self . absize () , self . dcyc () , self . dmode ())
    }
}
#[doc = "Miscelaneous Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miscr(pub u32);
impl Miscr {
    #[doc = "Add delay on internal Rx clock to fine tune the sampling position. Note: effective 5-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxclkdly(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Add delay on internal Rx clock to fine tune the sampling position. Note: effective 5-bit"]
    #[inline(always)]
    pub const fn set_rxclkdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Add delay on output clock to fine tune the clock position. Note: effective 7-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sckdly(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Add delay on output clock to fine tune the clock position. Note: effective 7-bit"]
    #[inline(always)]
    pub const fn set_sckdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Delay the input DQS signal to the appropriate sampling position. For device w/ DQS signal only. Note: effective 7-bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dqsdly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Delay the input DQS signal to the appropriate sampling position. For device w/ DQS signal only. Note: effective 7-bit"]
    #[inline(always)]
    pub const fn set_dqsdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Invert internal Rx clock to add half-cycle delay (coarse tune) when sampling data. It is usually used for FLASH device w/ higher frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn rxclkinv(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Invert internal Rx clock to add half-cycle delay (coarse tune) when sampling data. It is usually used for FLASH device w/ higher frequency."]
    #[inline(always)]
    pub const fn set_rxclkinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Invert output clock. This bit is used to align (coarse tune) the output clock to the center of output data."]
    #[must_use]
    #[inline(always)]
    pub const fn sckinv(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output clock. This bit is used to align (coarse tune) the output clock to the center of output data."]
    #[inline(always)]
    pub const fn set_sckinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable pre-sampling for DTRReserved-Do not modify"]
    #[must_use]
    #[inline(always)]
    pub const fn dtrpre(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable pre-sampling for DTRReserved-Do not modify"]
    #[inline(always)]
    pub const fn set_dtrpre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dbgsel(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dbgsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Miscr {
    #[inline(always)]
    fn default() -> Miscr {
        Miscr(0)
    }
}
impl core::fmt::Debug for Miscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Miscr")
            .field("rxclkdly", &self.rxclkdly())
            .field("sckdly", &self.sckdly())
            .field("dqsdly", &self.dqsdly())
            .field("rxclkinv", &self.rxclkinv())
            .field("sckinv", &self.sckinv())
            .field("dtrpre", &self.dtrpre())
            .field("dbgsel", &self.dbgsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Miscr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Miscr {{ rxclkdly: {=u8:?}, sckdly: {=u8:?}, dqsdly: {=u8:?}, rxclkinv: {=bool:?}, sckinv: {=bool:?}, dtrpre: {=bool:?}, dbgsel: {=u8:?} }}" , self . rxclkdly () , self . sckdly () , self . dqsdly () , self . rxclkinv () , self . sckinv () , self . dtrpre () , self . dbgsel ())
    }
}
#[doc = "Nonce A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Noncea(pub u32);
impl Noncea {
    #[doc = "Used for on-the-fly decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn noncea(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Used for on-the-fly decryption"]
    #[inline(always)]
    pub const fn set_noncea(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Noncea {
    #[inline(always)]
    fn default() -> Noncea {
        Noncea(0)
    }
}
impl core::fmt::Debug for Noncea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Noncea")
            .field("noncea", &self.noncea())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Noncea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Noncea {{ noncea: {=u32:?} }}", self.noncea())
    }
}
#[doc = "Nonce B Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nonceb(pub u32);
impl Nonceb {
    #[doc = "Used for on-the-fly decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn nonceb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Used for on-the-fly decryption"]
    #[inline(always)]
    pub const fn set_nonceb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Nonceb {
    #[inline(always)]
    fn default() -> Nonceb {
        Nonceb(0)
    }
}
impl core::fmt::Debug for Nonceb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nonceb")
            .field("nonceb", &self.nonceb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nonceb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nonceb {{ nonceb: {=u32:?} }}", self.nonceb())
    }
}
#[doc = "Prefetch Ending Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prear(pub u32);
impl Prear {
    #[doc = "Ending address of the prefetch area"]
    #[must_use]
    #[inline(always)]
    pub const fn ea(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Ending address of the prefetch area"]
    #[inline(always)]
    pub const fn set_ea(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Prear {
    #[inline(always)]
    fn default() -> Prear {
        Prear(0)
    }
}
impl core::fmt::Debug for Prear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prear").field("ea", &self.ea()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Prear {{ ea: {=u32:?} }}", self.ea())
    }
}
#[doc = "Prefetch Starting Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prsar(pub u32);
impl Prsar {
    #[doc = "Starting address of the prefetch area If prefetch is enabled and the read address falls into \\[PRSAR, PREAR), controller will prefetch the following data"]
    #[must_use]
    #[inline(always)]
    pub const fn sa(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Starting address of the prefetch area If prefetch is enabled and the read address falls into \\[PRSAR, PREAR), controller will prefetch the following data"]
    #[inline(always)]
    pub const fn set_sa(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Prsar {
    #[inline(always)]
    fn default() -> Prsar {
        Prsar(0)
    }
}
impl core::fmt::Debug for Prsar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prsar").field("sa", &self.sa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prsar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Prsar {{ sa: {=u32:?} }}", self.sa())
    }
}
#[doc = "Prescaler Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psclr(pub u32);
impl Psclr {
    #[doc = "Prescaler divider. 0: MCLK = FCLK/1 1: MCLK = FCLK/1 2: MCLK = FCLK/2 n: MCLK = FCLK/n Note: FLASH clock = MCLK. E.g. FCLK=192M and DIV=2, then FLASH clock = MCLK = 192/2 = 96MHz PSRAM clock = MCLK/2. E.g. FCLK=240M and DIV=1, then PSRAM clock = MCLK/2 = 240/2 = 120MHz"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler divider. 0: MCLK = FCLK/1 1: MCLK = FCLK/1 2: MCLK = FCLK/2 n: MCLK = FCLK/n Note: FLASH clock = MCLK. E.g. FCLK=192M and DIV=2, then FLASH clock = MCLK = 192/2 = 96MHz PSRAM clock = MCLK/2. E.g. FCLK=240M and DIV=1, then PSRAM clock = MCLK/2 = 240/2 = 120MHz"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Psclr {
    #[inline(always)]
    fn default() -> Psclr {
        Psclr(0)
    }
}
impl core::fmt::Debug for Psclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psclr").field("div", &self.div()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Psclr {{ div: {=u8:?} }}", self.div())
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
#[doc = "Status Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Write 1 to clear TCF"]
    #[must_use]
    #[inline(always)]
    pub const fn tcfc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear TCF"]
    #[inline(always)]
    pub const fn set_tcfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write 1 to clear SMF"]
    #[must_use]
    #[inline(always)]
    pub const fn smfc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear SMF"]
    #[inline(always)]
    pub const fn set_smfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write 1 to clear CSVF"]
    #[must_use]
    #[inline(always)]
    pub const fn csvfc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear CSVF"]
    #[inline(always)]
    pub const fn set_csvfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write 1 to clear RBXF"]
    #[must_use]
    #[inline(always)]
    pub const fn rbxfc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear RBXF"]
    #[inline(always)]
    pub const fn set_rbxfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("tcfc", &self.tcfc())
            .field("smfc", &self.smfc())
            .field("csvfc", &self.csvfc())
            .field("rbxfc", &self.rbxfc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ tcfc: {=bool:?}, smfc: {=bool:?}, csvfc: {=bool:?}, rbxfc: {=bool:?} }}",
            self.tcfc(),
            self.smfc(),
            self.csvfc(),
            self.rbxfc()
        )
    }
}
#[doc = "Status Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smkr(pub u32);
impl Smkr {
    #[doc = "Status mask 0: the corresponding bit is not considered to compare 1: the corresponding bit is considered to compare"]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Status mask 0: the corresponding bit is not considered to compare 1: the corresponding bit is considered to compare"]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Smkr {
    #[inline(always)]
    fn default() -> Smkr {
        Smkr(0)
    }
}
impl core::fmt::Debug for Smkr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smkr").field("mask", &self.mask()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smkr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smkr {{ mask: {=u32:?} }}", self.mask())
    }
}
#[doc = "Status Match Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr(pub u32);
impl Smr {
    #[doc = "If status match is enabled, this register is compared with the data read from external memory. Together with SMKR, only the bits with mask=1 will be considered to compare in AND or OR mode as configured in SMM field."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "If status match is enabled, this register is compared with the data read from external memory. Together with SMKR, only the bits with mask=1 will be considered to compare in AND or OR mode as configured in SMM field."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Smr {
    #[inline(always)]
    fn default() -> Smr {
        Smr(0)
    }
}
impl core::fmt::Debug for Smr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smr")
            .field("status", &self.status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smr {{ status: {=u32:?} }}", self.status())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Transfer complete flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Status match flag in Polling Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn smf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Status match flag in Polling Mode"]
    #[inline(always)]
    pub const fn set_smf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CS max violation flag"]
    #[must_use]
    #[inline(always)]
    pub const fn csvf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CS max violation flag"]
    #[inline(always)]
    pub const fn set_csvf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Row boundary crossing flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rbxf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Row boundary crossing flag"]
    #[inline(always)]
    pub const fn set_rbxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "For debug purpose only"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purpose only"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("tcf", &self.tcf())
            .field("smf", &self.smf())
            .field("csvf", &self.csvf())
            .field("rbxf", &self.rbxf())
            .field("busy", &self.busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sr {{ tcf: {=bool:?}, smf: {=bool:?}, csvf: {=bool:?}, rbxf: {=bool:?}, busy: {=bool:?} }}" , self . tcf () , self . smf () , self . csvf () , self . rbxf () , self . busy ())
    }
}
#[doc = "Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timr(pub u32);
impl Timr {
    #[doc = "After the transaction is complete, CS remains low for multiple cycles of MCLK as specified by this register. For example if TIMEOUT=n, CS remains active for n cycles, during which if a new transaction occurs and the address is consecutive, the memory access can be resumed w/o sending the command and address again."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "After the transaction is complete, CS remains low for multiple cycles of MCLK as specified by this register. For example if TIMEOUT=n, CS remains active for n cycles, during which if a new transaction occurs and the address is consecutive, the memory access can be resumed w/o sending the command and address again."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Timr {
    #[inline(always)]
    fn default() -> Timr {
        Timr(0)
    }
}
impl core::fmt::Debug for Timr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timr")
            .field("timeout", &self.timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timr {{ timeout: {=u16:?} }}", self.timeout())
    }
}
#[doc = "WDT Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtr(pub u32);
impl Wdtr {
    #[doc = "Set timeout value in number of clk_wdt cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Set timeout value in number of clk_wdt cycles"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "WDT enable. This watchdog is on AHB side such that bus access will not hang in exceptional cases"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "WDT enable. This watchdog is on AHB side such that bus access will not hang in exceptional cases"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Timeout flag. Self cleared when HREADYOUT becomes ready"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout flag. Self cleared when HREADYOUT becomes ready"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Wdtr {
    #[inline(always)]
    fn default() -> Wdtr {
        Wdtr(0)
    }
}
impl core::fmt::Debug for Wdtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdtr")
            .field("timeout", &self.timeout())
            .field("en", &self.en())
            .field("tof", &self.tof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdtr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdtr {{ timeout: {=u16:?}, en: {=bool:?}, tof: {=bool:?} }}",
            self.timeout(),
            self.en(),
            self.tof()
        )
    }
}
