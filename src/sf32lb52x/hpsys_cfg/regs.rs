#[doc = "Analog Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anatr(pub u32);
impl Anatr {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_te_atest0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_te_atest0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_ur_atest0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_ur_atest0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_te_atest1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_te_atest1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_ur_atest1(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_ur_atest1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
}
impl Default for Anatr {
    #[inline(always)]
    fn default() -> Anatr {
        Anatr(0)
    }
}
impl core::fmt::Debug for Anatr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Anatr")
            .field("dc_te_atest0", &self.dc_te_atest0())
            .field("dc_ur_atest0", &self.dc_ur_atest0())
            .field("dc_te_atest1", &self.dc_te_atest1())
            .field("dc_ur_atest1", &self.dc_ur_atest1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Anatr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Anatr {
            dc_te_atest0: bool,
            dc_ur_atest0: u8,
            dc_te_atest1: bool,
            dc_ur_atest1: u8,
        }
        let proxy = Anatr {
            dc_te_atest0: self.dc_te_atest0(),
            dc_ur_atest0: self.dc_ur_atest0(),
            dc_te_atest1: self.dc_te_atest1(),
            dc_ur_atest1: self.dc_ur_atest1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ANAU Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnauCr(pub u32);
impl AnauCr {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn en_bg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_en_bg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn en_vbat_mon(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_en_vbat_mon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn efuse_vdd_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_efuse_vdd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn efuse_vdd_pd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_efuse_vdd_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_mr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for AnauCr {
    #[inline(always)]
    fn default() -> AnauCr {
        AnauCr(0)
    }
}
impl core::fmt::Debug for AnauCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnauCr")
            .field("en_bg", &self.en_bg())
            .field("en_vbat_mon", &self.en_vbat_mon())
            .field("efuse_vdd_en", &self.efuse_vdd_en())
            .field("efuse_vdd_pd", &self.efuse_vdd_pd())
            .field("dc_mr", &self.dc_mr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnauCr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AnauCr {
            en_bg: bool,
            en_vbat_mon: bool,
            efuse_vdd_en: bool,
            efuse_vdd_pd: bool,
            dc_mr: u8,
        }
        let proxy = AnauCr {
            en_bg: self.en_bg(),
            en_vbat_mon: self.en_vbat_mon(),
            efuse_vdd_en: self.efuse_vdd_en(),
            efuse_vdd_pd: self.efuse_vdd_pd(),
            dc_mr: self.dc_mr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ANAU Reserve Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnauRsvd(pub u32);
impl AnauRsvd {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for AnauRsvd {
    #[inline(always)]
    fn default() -> AnauRsvd {
        AnauRsvd(0)
    }
}
impl core::fmt::Debug for AnauRsvd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnauRsvd")
            .field("reserve0", &self.reserve0())
            .field("reserve1", &self.reserve1())
            .field("reserve2", &self.reserve2())
            .field("reserve3", &self.reserve3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnauRsvd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AnauRsvd {
            reserve0: u8,
            reserve1: u8,
            reserve2: u8,
            reserve3: u8,
        }
        let proxy = AnauRsvd {
            reserve0: self.reserve0(),
            reserve1: self.reserve1(),
            reserve2: self.reserve2(),
            reserve3: self.reserve3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ATIM1 Pin Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atim1Pinr1(pub u32);
impl Atim1Pinr1 {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch1_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch1_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch2_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch2_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch3_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch3_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch4_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch4_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Atim1Pinr1 {
    #[inline(always)]
    fn default() -> Atim1Pinr1 {
        Atim1Pinr1(0)
    }
}
impl core::fmt::Debug for Atim1Pinr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atim1Pinr1")
            .field("ch1_pin", &self.ch1_pin())
            .field("ch2_pin", &self.ch2_pin())
            .field("ch3_pin", &self.ch3_pin())
            .field("ch4_pin", &self.ch4_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atim1Pinr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Atim1Pinr1 {
            ch1_pin: u8,
            ch2_pin: u8,
            ch3_pin: u8,
            ch4_pin: u8,
        }
        let proxy = Atim1Pinr1 {
            ch1_pin: self.ch1_pin(),
            ch2_pin: self.ch2_pin(),
            ch3_pin: self.ch3_pin(),
            ch4_pin: self.ch4_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ATIM1 Pin Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atim1Pinr2(pub u32);
impl Atim1Pinr2 {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch1n_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch1n_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch2n_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch2n_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch3n_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch3n_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Atim1Pinr2 {
    #[inline(always)]
    fn default() -> Atim1Pinr2 {
        Atim1Pinr2(0)
    }
}
impl core::fmt::Debug for Atim1Pinr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atim1Pinr2")
            .field("ch1n_pin", &self.ch1n_pin())
            .field("ch2n_pin", &self.ch2n_pin())
            .field("ch3n_pin", &self.ch3n_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atim1Pinr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Atim1Pinr2 {
            ch1n_pin: u8,
            ch2n_pin: u8,
            ch3n_pin: u8,
        }
        let proxy = Atim1Pinr2 {
            ch1n_pin: self.ch1n_pin(),
            ch2n_pin: self.ch2n_pin(),
            ch3n_pin: self.ch3n_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ATIM1 Pin Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atim1Pinr3(pub u32);
impl Atim1Pinr3 {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn bk_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_bk_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn bk2_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_bk2_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn etr_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_etr_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Atim1Pinr3 {
    #[inline(always)]
    fn default() -> Atim1Pinr3 {
        Atim1Pinr3(0)
    }
}
impl core::fmt::Debug for Atim1Pinr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Atim1Pinr3")
            .field("bk_pin", &self.bk_pin())
            .field("bk2_pin", &self.bk2_pin())
            .field("etr_pin", &self.etr_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Atim1Pinr3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Atim1Pinr3 {
            bk_pin: u8,
            bk2_pin: u8,
            etr_pin: u8,
        }
        let proxy = Atim1Pinr3 {
            bk_pin: self.bk_pin(),
            bk2_pin: self.bk2_pin(),
            etr_pin: self.etr_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Boot Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmr(pub u32);
impl Bmr {
    #[doc = "0 - normal mode, 1 - download mode"]
    #[inline(always)]
    pub const fn boot_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 - normal mode, 1 - download mode"]
    #[inline(always)]
    pub fn set_boot_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Bmr {
    #[inline(always)]
    fn default() -> Bmr {
        Bmr(0)
    }
}
impl core::fmt::Debug for Bmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bmr")
            .field("boot_mode", &self.boot_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bmr {
            boot_mode: bool,
        }
        let proxy = Bmr {
            boot_mode: self.boot_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CAU2 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cau2Cr(pub u32);
impl Cau2Cr {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn hpbg_vddpsw_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_hpbg_vddpsw_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn hpbg_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_hpbg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_tr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_tr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_br(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_mr(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
}
impl Default for Cau2Cr {
    #[inline(always)]
    fn default() -> Cau2Cr {
        Cau2Cr(0)
    }
}
impl core::fmt::Debug for Cau2Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cau2Cr")
            .field("hpbg_vddpsw_en", &self.hpbg_vddpsw_en())
            .field("hpbg_en", &self.hpbg_en())
            .field("dc_tr", &self.dc_tr())
            .field("dc_br", &self.dc_br())
            .field("dc_mr", &self.dc_mr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cau2Cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cau2Cr {
            hpbg_vddpsw_en: bool,
            hpbg_en: bool,
            dc_tr: u8,
            dc_br: u8,
            dc_mr: u8,
        }
        let proxy = Cau2Cr {
            hpbg_vddpsw_en: self.hpbg_vddpsw_en(),
            hpbg_en: self.hpbg_en(),
            dc_tr: self.dc_tr(),
            dc_br: self.dc_br(),
            dc_mr: self.dc_mr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CAU2 RSVD Register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cau2Rsvd(pub u32);
impl Cau2Rsvd {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Cau2Rsvd {
    #[inline(always)]
    fn default() -> Cau2Rsvd {
        Cau2Rsvd(0)
    }
}
impl core::fmt::Debug for Cau2Rsvd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cau2Rsvd")
            .field("reserve0", &self.reserve0())
            .field("reserve1", &self.reserve1())
            .field("reserve2", &self.reserve2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cau2Rsvd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cau2Rsvd {
            reserve0: u8,
            reserve1: u8,
            reserve2: u8,
        }
        let proxy = Cau2Rsvd {
            reserve0: self.reserve0(),
            reserve1: self.reserve1(),
            reserve2: self.reserve2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Debug Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgr(pub u32);
impl Dbgr {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn sel_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_sel_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn sel_h(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_sel_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn biten_l(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_biten_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn biten_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_biten_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn clk_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "set 1 to send NMI interrupt to LCPU"]
    #[inline(always)]
    pub const fn hp2lp_nmi(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to send NMI interrupt to LCPU"]
    #[inline(always)]
    pub fn set_hp2lp_nmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "LP2HP NMI interrupt enable"]
    #[inline(always)]
    pub const fn lp2hp_nmie(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "LP2HP NMI interrupt enable"]
    #[inline(always)]
    pub fn set_lp2hp_nmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "LP2HP NMI interrupt flag"]
    #[inline(always)]
    pub const fn lp2hp_nmif(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "LP2HP NMI interrupt flag"]
    #[inline(always)]
    pub fn set_lp2hp_nmif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn swap(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_swap(&mut self, val: bool) {
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
            .field("hp2lp_nmi", &self.hp2lp_nmi())
            .field("lp2hp_nmie", &self.lp2hp_nmie())
            .field("lp2hp_nmif", &self.lp2hp_nmif())
            .field("swap", &self.swap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbgr {
            sel_l: u8,
            sel_h: u8,
            biten_l: u8,
            biten_h: u8,
            clk_sel: u8,
            clk_en: bool,
            hp2lp_nmi: bool,
            lp2hp_nmie: bool,
            lp2hp_nmif: bool,
            swap: bool,
        }
        let proxy = Dbgr {
            sel_l: self.sel_l(),
            sel_h: self.sel_h(),
            biten_l: self.biten_l(),
            biten_h: self.biten_h(),
            clk_sel: self.clk_sel(),
            clk_en: self.clk_en(),
            hp2lp_nmi: self.hp2lp_nmi(),
            lp2hp_nmie: self.lp2hp_nmie(),
            lp2hp_nmif: self.lp2hp_nmif(),
            swap: self.swap(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPTIM ETR Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtrPinr(pub u32);
impl EtrPinr {
    #[doc = "Connect GPTIM1_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn etr1_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect GPTIM1_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_etr1_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect GPTIM2_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn etr2_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect GPTIM2_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_etr2_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for EtrPinr {
    #[inline(always)]
    fn default() -> EtrPinr {
        EtrPinr(0)
    }
}
impl core::fmt::Debug for EtrPinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtrPinr")
            .field("etr1_pin", &self.etr1_pin())
            .field("etr2_pin", &self.etr2_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtrPinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EtrPinr {
            etr1_pin: u8,
            etr2_pin: u8,
        }
        let proxy = EtrPinr {
            etr1_pin: self.etr1_pin(),
            etr2_pin: self.etr2_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPTIM1 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptim1Pinr(pub u32);
impl Gptim1Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch1_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch1_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch2_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch2_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch3_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch3_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch4_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch4_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Gptim1Pinr {
    #[inline(always)]
    fn default() -> Gptim1Pinr {
        Gptim1Pinr(0)
    }
}
impl core::fmt::Debug for Gptim1Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gptim1Pinr")
            .field("ch1_pin", &self.ch1_pin())
            .field("ch2_pin", &self.ch2_pin())
            .field("ch3_pin", &self.ch3_pin())
            .field("ch4_pin", &self.ch4_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gptim1Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gptim1Pinr {
            ch1_pin: u8,
            ch2_pin: u8,
            ch3_pin: u8,
            ch4_pin: u8,
        }
        let proxy = Gptim1Pinr {
            ch1_pin: self.ch1_pin(),
            ch2_pin: self.ch2_pin(),
            ch3_pin: self.ch3_pin(),
            ch4_pin: self.ch4_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPTIM2 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptim2Pinr(pub u32);
impl Gptim2Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch1_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch1_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch2_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch2_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch3_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch3_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn ch4_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_ch4_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Gptim2Pinr {
    #[inline(always)]
    fn default() -> Gptim2Pinr {
        Gptim2Pinr(0)
    }
}
impl core::fmt::Debug for Gptim2Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gptim2Pinr")
            .field("ch1_pin", &self.ch1_pin())
            .field("ch2_pin", &self.ch2_pin())
            .field("ch3_pin", &self.ch3_pin())
            .field("ch4_pin", &self.ch4_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gptim2Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gptim2Pinr {
            ch1_pin: u8,
            ch2_pin: u8,
            ch3_pin: u8,
            ch4_pin: u8,
        }
        let proxy = Gptim2Pinr {
            ch1_pin: self.ch1_pin(),
            ch2_pin: self.ch2_pin(),
            ch3_pin: self.ch3_pin(),
            ch4_pin: self.ch4_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I2C1 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c1Pinr(pub u32);
impl I2c1Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn scl_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_scl_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn sda_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_sda_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for I2c1Pinr {
    #[inline(always)]
    fn default() -> I2c1Pinr {
        I2c1Pinr(0)
    }
}
impl core::fmt::Debug for I2c1Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2c1Pinr")
            .field("scl_pin", &self.scl_pin())
            .field("sda_pin", &self.sda_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2c1Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I2c1Pinr {
            scl_pin: u8,
            sda_pin: u8,
        }
        let proxy = I2c1Pinr {
            scl_pin: self.scl_pin(),
            sda_pin: self.sda_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I2C2 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c2Pinr(pub u32);
impl I2c2Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn scl_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_scl_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn sda_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_sda_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for I2c2Pinr {
    #[inline(always)]
    fn default() -> I2c2Pinr {
        I2c2Pinr(0)
    }
}
impl core::fmt::Debug for I2c2Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2c2Pinr")
            .field("scl_pin", &self.scl_pin())
            .field("sda_pin", &self.sda_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2c2Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I2c2Pinr {
            scl_pin: u8,
            sda_pin: u8,
        }
        let proxy = I2c2Pinr {
            scl_pin: self.scl_pin(),
            sda_pin: self.sda_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I2C3 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c3Pinr(pub u32);
impl I2c3Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn scl_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_scl_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn sda_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_sda_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for I2c3Pinr {
    #[inline(always)]
    fn default() -> I2c3Pinr {
        I2c3Pinr(0)
    }
}
impl core::fmt::Debug for I2c3Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2c3Pinr")
            .field("scl_pin", &self.scl_pin())
            .field("sda_pin", &self.sda_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2c3Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I2c3Pinr {
            scl_pin: u8,
            sda_pin: u8,
        }
        let proxy = I2c3Pinr {
            scl_pin: self.scl_pin(),
            sda_pin: self.sda_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I2C4 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c4Pinr(pub u32);
impl I2c4Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn scl_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_scl_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn sda_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_sda_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for I2c4Pinr {
    #[inline(always)]
    fn default() -> I2c4Pinr {
        I2c4Pinr(0)
    }
}
impl core::fmt::Debug for I2c4Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2c4Pinr")
            .field("scl_pin", &self.scl_pin())
            .field("sda_pin", &self.sda_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2c4Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I2c4Pinr {
            scl_pin: u8,
            sda_pin: u8,
        }
        let proxy = I2c4Pinr {
            scl_pin: self.scl_pin(),
            sda_pin: self.sda_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc = "Revision ID"]
    #[inline(always)]
    pub const fn revid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Revision ID"]
    #[inline(always)]
    pub fn set_revid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Package ID"]
    #[inline(always)]
    pub const fn pid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Package ID"]
    #[inline(always)]
    pub fn set_pid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Chip ID"]
    #[inline(always)]
    pub const fn cid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Chip ID"]
    #[inline(always)]
    pub fn set_cid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Series ID"]
    #[inline(always)]
    pub const fn sid(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Series ID"]
    #[inline(always)]
    pub fn set_sid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Idr {
    #[inline(always)]
    fn default() -> Idr {
        Idr(0)
    }
}
impl core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idr")
            .field("revid", &self.revid())
            .field("pid", &self.pid())
            .field("cid", &self.cid())
            .field("sid", &self.sid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Idr {
            revid: u8,
            pid: u8,
            cid: u8,
            sid: u8,
        }
        let proxy = Idr {
            revid: self.revid(),
            pid: self.pid(),
            cid: self.cid(),
            sid: self.sid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Selection for LCPU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpirq(pub u32);
impl Lpirq {
    #[doc = "select hp2lp0 interrupt source"]
    #[inline(always)]
    pub const fn sel0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "select hp2lp0 interrupt source"]
    #[inline(always)]
    pub fn set_sel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "hp2lp0 interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn if0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "hp2lp0 interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub fn set_if0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "select hp2lp1 interrupt source"]
    #[inline(always)]
    pub const fn sel1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "select hp2lp1 interrupt source"]
    #[inline(always)]
    pub fn set_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "hp2lp1 interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn if1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "hp2lp1 interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub fn set_if1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Lpirq {
    #[inline(always)]
    fn default() -> Lpirq {
        Lpirq(0)
    }
}
impl core::fmt::Debug for Lpirq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpirq")
            .field("sel0", &self.sel0())
            .field("if0", &self.if0())
            .field("sel1", &self.sel1())
            .field("if1", &self.if1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpirq {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lpirq {
            sel0: u8,
            if0: bool,
            sel1: u8,
            if1: bool,
        }
        let proxy = Lpirq {
            sel0: self.sel0(),
            if0: self.if0(),
            sel1: self.sel1(),
            if1: self.if1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM1 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptim1Pinr(pub u32);
impl Lptim1Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn in_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_in_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn out_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_out_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn etr_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_etr_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Lptim1Pinr {
    #[inline(always)]
    fn default() -> Lptim1Pinr {
        Lptim1Pinr(0)
    }
}
impl core::fmt::Debug for Lptim1Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lptim1Pinr")
            .field("in_pin", &self.in_pin())
            .field("out_pin", &self.out_pin())
            .field("etr_pin", &self.etr_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lptim1Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lptim1Pinr {
            in_pin: u8,
            out_pin: u8,
            etr_pin: u8,
        }
        let proxy = Lptim1Pinr {
            in_pin: self.in_pin(),
            out_pin: self.out_pin(),
            etr_pin: self.etr_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM2 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptim2Pinr(pub u32);
impl Lptim2Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn in_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_in_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn out_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_out_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn etr_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_etr_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Lptim2Pinr {
    #[inline(always)]
    fn default() -> Lptim2Pinr {
        Lptim2Pinr(0)
    }
}
impl core::fmt::Debug for Lptim2Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lptim2Pinr")
            .field("in_pin", &self.in_pin())
            .field("out_pin", &self.out_pin())
            .field("etr_pin", &self.etr_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lptim2Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lptim2Pinr {
            in_pin: u8,
            out_pin: u8,
            etr_pin: u8,
        }
        let proxy = Lptim2Pinr {
            in_pin: self.in_pin(),
            out_pin: self.out_pin(),
            etr_pin: self.etr_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Memory Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdbgr(pub u32);
impl Mdbgr {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ls_ram0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ls_ram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ls_ram1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ls_ram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ls_ram2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ls_ram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ls_rom(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ls_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn pd_rom(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_pd_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("ls_ram2", &self.ls_ram2())
            .field("ls_rom", &self.ls_rom())
            .field("pd_rom", &self.pd_rom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdbgr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mdbgr {
            ls_ram0: bool,
            ls_ram1: bool,
            ls_ram2: bool,
            ls_rom: bool,
            pd_rom: bool,
        }
        let proxy = Mdbgr {
            ls_ram0: self.ls_ram0(),
            ls_ram1: self.ls_ram1(),
            ls_ram2: self.ls_ram2(),
            ls_rom: self.ls_rom(),
            pd_rom: self.pd_rom(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PTA Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PtaPinr(pub u32);
impl PtaPinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn bt_active(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_bt_active(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn bt_collision(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_bt_collision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn bt_priority(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_bt_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn wlan_active(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_wlan_active(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
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
        #[derive(defmt :: Format)]
        struct PtaPinr {
            bt_active: u8,
            bt_collision: u8,
            bt_priority: u8,
            wlan_active: u8,
        }
        let proxy = PtaPinr {
            bt_active: self.bt_active(),
            bt_collision: self.bt_collision(),
            bt_priority: self.bt_priority(),
            wlan_active: self.wlan_active(),
        };
        defmt::write!(f, "{}", proxy)
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
        #[derive(defmt :: Format)]
        struct Rsvd1 {}
        let proxy = Rsvd1 {};
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Mirrored RTC Date Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcDr(pub u32);
impl RtcDr {
    #[doc = "Date units in BCD format"]
    #[inline(always)]
    pub const fn du(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Date units in BCD format"]
    #[inline(always)]
    pub fn set_du(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Month units in BCD format"]
    #[inline(always)]
    pub const fn mu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month units in BCD format"]
    #[inline(always)]
    pub fn set_mu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Month tens in BCD format"]
    #[inline(always)]
    pub const fn mt(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Month tens in BCD format"]
    #[inline(always)]
    pub fn set_mt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Week day units 000: forbidden 001: Monday ... 111: Sunday"]
    #[inline(always)]
    pub const fn wd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Week day units 000: forbidden 001: Monday ... 111: Sunday"]
    #[inline(always)]
    pub fn set_wd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Year units in BCD format"]
    #[inline(always)]
    pub const fn yu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Year units in BCD format"]
    #[inline(always)]
    pub fn set_yu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Year tens in BCD format"]
    #[inline(always)]
    pub const fn yt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Year tens in BCD format"]
    #[inline(always)]
    pub fn set_yt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Century flag"]
    #[inline(always)]
    pub const fn cb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Century flag"]
    #[inline(always)]
    pub fn set_cb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_err(&mut self, val: bool) {
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
        #[derive(defmt :: Format)]
        struct RtcDr {
            du: u8,
            dt: u8,
            mu: u8,
            mt: bool,
            wd: u8,
            yu: u8,
            yt: u8,
            cb: bool,
            err: bool,
        }
        let proxy = RtcDr {
            du: self.du(),
            dt: self.dt(),
            mu: self.mu(),
            mt: self.mt(),
            wd: self.wd(),
            yu: self.yu(),
            yt: self.yt(),
            cb: self.cb(),
            err: self.err(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Mirrored RTC Time Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcTr(pub u32);
impl RtcTr {
    #[doc = "Sub-second counter"]
    #[inline(always)]
    pub const fn ss(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sub-second counter"]
    #[inline(always)]
    pub fn set_ss(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub const fn su(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub fn set_su(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub const fn mnu(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub fn set_mnu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub const fn mnt(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub fn set_mnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub const fn hu(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub fn set_hu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub const fn ht(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub fn set_ht(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
    #[doc = "AM/PM notation 0: AM 1: PM"]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AM/PM notation 0: AM 1: PM"]
    #[inline(always)]
    pub fn set_pm(&mut self, val: bool) {
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
        #[derive(defmt :: Format)]
        struct RtcTr {
            ss: u16,
            su: u8,
            st: u8,
            mnu: u8,
            mnt: u8,
            hu: u8,
            ht: u8,
            pm: bool,
        }
        let proxy = RtcTr {
            ss: self.ss(),
            su: self.su(),
            st: self.st(),
            mnu: self.mnu(),
            mnt: self.mnt(),
            hu: self.hu(),
            ht: self.ht(),
            pm: self.pm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Security Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn fkey_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_fkey_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("fkey_mode", &self.fkey_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Scr {
            fkey_mode: bool,
        }
        let proxy = Scr {
            fkey_mode: self.fkey_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SW Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swcr(pub u32);
impl Swcr {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn swsel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_swsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Swcr {
    #[inline(always)]
    fn default() -> Swcr {
        Swcr(0)
    }
}
impl core::fmt::Debug for Swcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swcr")
            .field("swsel", &self.swsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Swcr {
            swsel: bool,
        }
        let proxy = Swcr {
            swsel: self.swsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HPSYS RSVD Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysRsvd(pub u32);
impl SysRsvd {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn reserve3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_reserve3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for SysRsvd {
    #[inline(always)]
    fn default() -> SysRsvd {
        SysRsvd(0)
    }
}
impl core::fmt::Debug for SysRsvd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysRsvd")
            .field("reserve0", &self.reserve0())
            .field("reserve1", &self.reserve1())
            .field("reserve2", &self.reserve2())
            .field("reserve3", &self.reserve3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysRsvd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SysRsvd {
            reserve0: u8,
            reserve1: u8,
            reserve2: u8,
            reserve3: u8,
        }
        let proxy = SysRsvd {
            reserve0: self.reserve0(),
            reserve1: self.reserve1(),
            reserve2: self.reserve2(),
            reserve3: self.reserve3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Configure Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscr(pub u32);
impl Syscr {
    #[doc = "If set to 1, WDT1 reset will reboot the whole chip"]
    #[inline(always)]
    pub const fn wdt1_reboot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, WDT1 reset will reboot the whole chip"]
    #[inline(always)]
    pub fn set_wdt1_reboot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0: MPI2 AHB space is allocated to MPI2 1: MPI2 AHB space is allocated to SDMMC1"]
    #[inline(always)]
    pub const fn sdnand(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0: MPI2 AHB space is allocated to MPI2 1: MPI2 AHB space is allocated to SDMMC1"]
    #[inline(always)]
    pub fn set_sdnand(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "select work mode 0: enhanced mode 1: base mode"]
    #[inline(always)]
    pub const fn ldo_vsel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "select work mode 0: enhanced mode 1: base mode"]
    #[inline(always)]
    pub fn set_ldo_vsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("wdt1_reboot", &self.wdt1_reboot())
            .field("sdnand", &self.sdnand())
            .field("ldo_vsel", &self.ldo_vsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syscr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Syscr {
            wdt1_reboot: bool,
            sdnand: bool,
            ldo_vsel: bool,
        }
        let proxy = Syscr {
            wdt1_reboot: self.wdt1_reboot(),
            sdnand: self.sdnand(),
            ldo_vsel: self.ldo_vsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ULP Memory Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulpmcr(pub u32);
impl Ulpmcr {
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ram_rm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ram_rm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ram_rme(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ram_rme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ram_ra(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ram_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ram_wa(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ram_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn ram_wpulse(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_ram_wpulse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn rom_rm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_rom_rm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn rom_rme(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_rom_rme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn rom_dis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_rom_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn force_on(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_force_on(&mut self, val: bool) {
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
        #[derive(defmt :: Format)]
        struct Ulpmcr {
            ram_rm: u8,
            ram_rme: bool,
            ram_ra: u8,
            ram_wa: u8,
            ram_wpulse: u8,
            rom_rm: u8,
            rom_rme: bool,
            rom_dis: bool,
            force_on: bool,
        }
        let proxy = Ulpmcr {
            ram_rm: self.ram_rm(),
            ram_rme: self.ram_rme(),
            ram_ra: self.ram_ra(),
            ram_wa: self.ram_wa(),
            ram_wpulse: self.ram_wpulse(),
            rom_rm: self.rom_rm(),
            rom_rme: self.rom_rme(),
            rom_dis: self.rom_dis(),
            force_on: self.force_on(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USART1 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart1Pinr(pub u32);
impl Usart1Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn txd_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_txd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn rxd_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_rxd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn rts_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_rts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn cts_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_cts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Usart1Pinr {
    #[inline(always)]
    fn default() -> Usart1Pinr {
        Usart1Pinr(0)
    }
}
impl core::fmt::Debug for Usart1Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usart1Pinr")
            .field("txd_pin", &self.txd_pin())
            .field("rxd_pin", &self.rxd_pin())
            .field("rts_pin", &self.rts_pin())
            .field("cts_pin", &self.cts_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usart1Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usart1Pinr {
            txd_pin: u8,
            rxd_pin: u8,
            rts_pin: u8,
            cts_pin: u8,
        }
        let proxy = Usart1Pinr {
            txd_pin: self.txd_pin(),
            rxd_pin: self.rxd_pin(),
            rts_pin: self.rts_pin(),
            cts_pin: self.cts_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USART2 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart2Pinr(pub u32);
impl Usart2Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn txd_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_txd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn rxd_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_rxd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn rts_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_rts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn cts_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_cts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Usart2Pinr {
    #[inline(always)]
    fn default() -> Usart2Pinr {
        Usart2Pinr(0)
    }
}
impl core::fmt::Debug for Usart2Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usart2Pinr")
            .field("txd_pin", &self.txd_pin())
            .field("rxd_pin", &self.rxd_pin())
            .field("rts_pin", &self.rts_pin())
            .field("cts_pin", &self.cts_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usart2Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usart2Pinr {
            txd_pin: u8,
            rxd_pin: u8,
            rts_pin: u8,
            cts_pin: u8,
        }
        let proxy = Usart2Pinr {
            txd_pin: self.txd_pin(),
            rxd_pin: self.rxd_pin(),
            rts_pin: self.rts_pin(),
            cts_pin: self.cts_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USART3 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart3Pinr(pub u32);
impl Usart3Pinr {
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn txd_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_txd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn rxd_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_rxd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn rts_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_rts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub const fn cts_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating."]
    #[inline(always)]
    pub fn set_cts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Usart3Pinr {
    #[inline(always)]
    fn default() -> Usart3Pinr {
        Usart3Pinr(0)
    }
}
impl core::fmt::Debug for Usart3Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usart3Pinr")
            .field("txd_pin", &self.txd_pin())
            .field("rxd_pin", &self.rxd_pin())
            .field("rts_pin", &self.rts_pin())
            .field("cts_pin", &self.cts_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usart3Pinr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usart3Pinr {
            txd_pin: u8,
            rxd_pin: u8,
            rts_pin: u8,
            cts_pin: u8,
        }
        let proxy = Usart3Pinr {
            txd_pin: self.txd_pin(),
            rxd_pin: self.rxd_pin(),
            rts_pin: self.rts_pin(),
            cts_pin: self.cts_pin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USB Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbcr(pub u32);
impl Usbcr {
    #[doc = "USB PHY enable, turn on power swith, power up LDO and bias"]
    #[inline(always)]
    pub const fn usb_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB PHY enable, turn on power swith, power up LDO and bias"]
    #[inline(always)]
    pub fn set_usb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "2.5V LDO output voltage setting 0 = 2.40 V, 1 = 2.47 V, 2 = 2.53 V, 3 = 2.60 V, 4 = 2.60 V, 5 = 2.67 V, 6 = 2.73 V, 7 = 2.8 V"]
    #[inline(always)]
    pub const fn ldo_vsel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "2.5V LDO output voltage setting 0 = 2.40 V, 1 = 2.47 V, 2 = 2.53 V, 3 = 2.60 V, 4 = 2.60 V, 5 = 2.67 V, 6 = 2.73 V, 7 = 2.8 V"]
    #[inline(always)]
    pub fn set_ldo_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "2.5V LDO low power mode enable. 0 = 240 uA, 1 = 50 uA"]
    #[inline(always)]
    pub const fn ldo_lp_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "2.5V LDO low power mode enable. 0 = 240 uA, 1 = 50 uA"]
    #[inline(always)]
    pub fn set_ldo_lp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "enable DM 15k Ohm pull down resistor"]
    #[inline(always)]
    pub const fn dm_pd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "enable DM 15k Ohm pull down resistor"]
    #[inline(always)]
    pub fn set_dm_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0:disable dp pull up or pull down 1:enable dp pull or pull down"]
    #[inline(always)]
    pub const fn dp_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0:disable dp pull up or pull down 1:enable dp pull or pull down"]
    #[inline(always)]
    pub fn set_dp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TX outp impedance tuning 0 = 50 Ohm, 1 = 46 Ohm, 2 = 43 Ohm, 3 = 40 Ohm, 4 = 37.5 Ohm, 5 = 35 Ohm, 6 = 33 Ohm, 7 = 31.5 Ohm"]
    #[inline(always)]
    pub const fn tx_rtune(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "TX outp impedance tuning 0 = 50 Ohm, 1 = 46 Ohm, 2 = 43 Ohm, 3 = 40 Ohm, 4 = 37.5 Ohm, 5 = 35 Ohm, 6 = 33 Ohm, 7 = 31.5 Ohm"]
    #[inline(always)]
    pub fn set_tx_rtune(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_te(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn dc_tr(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub fn set_dc_tr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
}
impl Default for Usbcr {
    #[inline(always)]
    fn default() -> Usbcr {
        Usbcr(0)
    }
}
impl core::fmt::Debug for Usbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbcr")
            .field("usb_en", &self.usb_en())
            .field("ldo_vsel", &self.ldo_vsel())
            .field("ldo_lp_en", &self.ldo_lp_en())
            .field("dm_pd", &self.dm_pd())
            .field("dp_en", &self.dp_en())
            .field("tx_rtune", &self.tx_rtune())
            .field("dc_te", &self.dc_te())
            .field("dc_tr", &self.dc_tr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usbcr {
            usb_en: bool,
            ldo_vsel: u8,
            ldo_lp_en: bool,
            dm_pd: bool,
            dp_en: bool,
            tx_rtune: u8,
            dc_te: bool,
            dc_tr: u8,
        }
        let proxy = Usbcr {
            usb_en: self.usb_en(),
            ldo_vsel: self.ldo_vsel(),
            ldo_lp_en: self.ldo_lp_en(),
            dm_pd: self.dm_pd(),
            dp_en: self.dp_en(),
            tx_rtune: self.tx_rtune(),
            dc_te: self.dc_te(),
            dc_tr: self.dc_tr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
