#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa00(pub u32);
impl PadPa00 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa00 {
    #[inline(always)]
    fn default() -> PadPa00 {
        PadPa00(0)
    }
}
impl core::fmt::Debug for PadPa00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa00")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa00 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa00 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa00 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa01(pub u32);
impl PadPa01 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa01 {
    #[inline(always)]
    fn default() -> PadPa01 {
        PadPa01(0)
    }
}
impl core::fmt::Debug for PadPa01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa01")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa01 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa01 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa01 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa02(pub u32);
impl PadPa02 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa02 {
    #[inline(always)]
    fn default() -> PadPa02 {
        PadPa02(0)
    }
}
impl core::fmt::Debug for PadPa02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa02")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa02 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa02 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa02 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa03(pub u32);
impl PadPa03 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa03 {
    #[inline(always)]
    fn default() -> PadPa03 {
        PadPa03(0)
    }
}
impl core::fmt::Debug for PadPa03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa03")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa03 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa03 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa03 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa04(pub u32);
impl PadPa04 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa04 {
    #[inline(always)]
    fn default() -> PadPa04 {
        PadPa04(0)
    }
}
impl core::fmt::Debug for PadPa04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa04")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa04 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa04 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa04 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa05(pub u32);
impl PadPa05 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa05 {
    #[inline(always)]
    fn default() -> PadPa05 {
        PadPa05(0)
    }
}
impl core::fmt::Debug for PadPa05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa05")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa05 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa05 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa05 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa06(pub u32);
impl PadPa06 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa06 {
    #[inline(always)]
    fn default() -> PadPa06 {
        PadPa06(0)
    }
}
impl core::fmt::Debug for PadPa06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa06")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa06 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa06 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa06 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa07(pub u32);
impl PadPa07 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa07 {
    #[inline(always)]
    fn default() -> PadPa07 {
        PadPa07(0)
    }
}
impl core::fmt::Debug for PadPa07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa07")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa07 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa07 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa07 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa08(pub u32);
impl PadPa08 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa08 {
    #[inline(always)]
    fn default() -> PadPa08 {
        PadPa08(0)
    }
}
impl core::fmt::Debug for PadPa08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa08")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa08 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa08 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa08 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa09(pub u32);
impl PadPa09 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa09 {
    #[inline(always)]
    fn default() -> PadPa09 {
        PadPa09(0)
    }
}
impl core::fmt::Debug for PadPa09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa09")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa09 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa09 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa09 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa10(pub u32);
impl PadPa10 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa10 {
    #[inline(always)]
    fn default() -> PadPa10 {
        PadPa10(0)
    }
}
impl core::fmt::Debug for PadPa10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa10")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa10 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa10 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa11(pub u32);
impl PadPa11 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa11 {
    #[inline(always)]
    fn default() -> PadPa11 {
        PadPa11(0)
    }
}
impl core::fmt::Debug for PadPa11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa11")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa11 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa11 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa12(pub u32);
impl PadPa12 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa12 {
    #[inline(always)]
    fn default() -> PadPa12 {
        PadPa12(0)
    }
}
impl core::fmt::Debug for PadPa12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa12")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa12 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa12 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa13(pub u32);
impl PadPa13 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa13 {
    #[inline(always)]
    fn default() -> PadPa13 {
        PadPa13(0)
    }
}
impl core::fmt::Debug for PadPa13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa13")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa13 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa13 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa14(pub u32);
impl PadPa14 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa14 {
    #[inline(always)]
    fn default() -> PadPa14 {
        PadPa14(0)
    }
}
impl core::fmt::Debug for PadPa14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa14")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa14 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa14 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa14 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa15(pub u32);
impl PadPa15 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa15 {
    #[inline(always)]
    fn default() -> PadPa15 {
        PadPa15(0)
    }
}
impl core::fmt::Debug for PadPa15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa15")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa15 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa15 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa15 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa16(pub u32);
impl PadPa16 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa16 {
    #[inline(always)]
    fn default() -> PadPa16 {
        PadPa16(0)
    }
}
impl core::fmt::Debug for PadPa16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa16")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa16 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa16 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa16 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa17(pub u32);
impl PadPa17 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa17 {
    #[inline(always)]
    fn default() -> PadPa17 {
        PadPa17(0)
    }
}
impl core::fmt::Debug for PadPa17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa17")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa17 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa17 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa17 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa18(pub u32);
impl PadPa18 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa18 {
    #[inline(always)]
    fn default() -> PadPa18 {
        PadPa18(0)
    }
}
impl core::fmt::Debug for PadPa18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa18")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa18 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa18 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa18 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa19(pub u32);
impl PadPa19 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa19 {
    #[inline(always)]
    fn default() -> PadPa19 {
        PadPa19(0)
    }
}
impl core::fmt::Debug for PadPa19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa19")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa19 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa19 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa19 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa20(pub u32);
impl PadPa20 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa20 {
    #[inline(always)]
    fn default() -> PadPa20 {
        PadPa20(0)
    }
}
impl core::fmt::Debug for PadPa20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa20")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa20 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa20 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa21(pub u32);
impl PadPa21 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa21 {
    #[inline(always)]
    fn default() -> PadPa21 {
        PadPa21(0)
    }
}
impl core::fmt::Debug for PadPa21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa21")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa21 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa21 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa21 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa22(pub u32);
impl PadPa22 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa22 {
    #[inline(always)]
    fn default() -> PadPa22 {
        PadPa22(0)
    }
}
impl core::fmt::Debug for PadPa22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa22")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa22 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa22 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa22 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa23(pub u32);
impl PadPa23 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa23 {
    #[inline(always)]
    fn default() -> PadPa23 {
        PadPa23(0)
    }
}
impl core::fmt::Debug for PadPa23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa23")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa23 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa23 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa23 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa24(pub u32);
impl PadPa24 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa24 {
    #[inline(always)]
    fn default() -> PadPa24 {
        PadPa24(0)
    }
}
impl core::fmt::Debug for PadPa24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa24")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa24 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa24 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa24 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa25(pub u32);
impl PadPa25 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa25 {
    #[inline(always)]
    fn default() -> PadPa25 {
        PadPa25(0)
    }
}
impl core::fmt::Debug for PadPa25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa25")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa25 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa25 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa25 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa26(pub u32);
impl PadPa26 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa26 {
    #[inline(always)]
    fn default() -> PadPa26 {
        PadPa26(0)
    }
}
impl core::fmt::Debug for PadPa26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa26")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa26 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa26 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa26 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa27(pub u32);
impl PadPa27 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa27 {
    #[inline(always)]
    fn default() -> PadPa27 {
        PadPa27(0)
    }
}
impl core::fmt::Debug for PadPa27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa27")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa27 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa27 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa27 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa28(pub u32);
impl PadPa28 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa28 {
    #[inline(always)]
    fn default() -> PadPa28 {
        PadPa28(0)
    }
}
impl core::fmt::Debug for PadPa28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa28")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa28 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa28 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa28 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa29(pub u32);
impl PadPa29 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa29 {
    #[inline(always)]
    fn default() -> PadPa29 {
        PadPa29(0)
    }
}
impl core::fmt::Debug for PadPa29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa29")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa29 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa29 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa29 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa30(pub u32);
impl PadPa30 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa30 {
    #[inline(always)]
    fn default() -> PadPa30 {
        PadPa30(0)
    }
}
impl core::fmt::Debug for PadPa30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa30")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa30 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa30 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa30 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa31(pub u32);
impl PadPa31 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa31 {
    #[inline(always)]
    fn default() -> PadPa31 {
        PadPa31(0)
    }
}
impl core::fmt::Debug for PadPa31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa31")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa31 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa31 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa31 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa32(pub u32);
impl PadPa32 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa32 {
    #[inline(always)]
    fn default() -> PadPa32 {
        PadPa32(0)
    }
}
impl core::fmt::Debug for PadPa32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa32")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa32 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa32 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa32 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa33(pub u32);
impl PadPa33 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa33 {
    #[inline(always)]
    fn default() -> PadPa33 {
        PadPa33(0)
    }
}
impl core::fmt::Debug for PadPa33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa33")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa33 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa33 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa33 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa34(pub u32);
impl PadPa34 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa34 {
    #[inline(always)]
    fn default() -> PadPa34 {
        PadPa34(0)
    }
}
impl core::fmt::Debug for PadPa34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa34")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa34 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa34 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa34 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa35(pub u32);
impl PadPa35 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa35 {
    #[inline(always)]
    fn default() -> PadPa35 {
        PadPa35(0)
    }
}
impl core::fmt::Debug for PadPa35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa35")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa35 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa35 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa35 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa36(pub u32);
impl PadPa36 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa36 {
    #[inline(always)]
    fn default() -> PadPa36 {
        PadPa36(0)
    }
}
impl core::fmt::Debug for PadPa36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa36")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa36 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa36 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa36 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa37(pub u32);
impl PadPa37 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa37 {
    #[inline(always)]
    fn default() -> PadPa37 {
        PadPa37(0)
    }
}
impl core::fmt::Debug for PadPa37 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa37")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa37 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa37 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa37 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa38(pub u32);
impl PadPa38 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa38 {
    #[inline(always)]
    fn default() -> PadPa38 {
        PadPa38(0)
    }
}
impl core::fmt::Debug for PadPa38 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa38")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa38 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa38 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa38 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa39(pub u32);
impl PadPa39 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode"]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive"]
    #[inline(always)]
    pub const fn ds(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive"]
    #[inline(always)]
    pub fn set_ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa39 {
    #[inline(always)]
    fn default() -> PadPa39 {
        PadPa39(0)
    }
}
impl core::fmt::Debug for PadPa39 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa39")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("mode", &self.mode())
            .field("ds", &self.ds())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa39 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa39 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            mode: bool,
            ds: bool,
            poe: bool,
        }
        let proxy = PadPa39 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            mode: self.mode(),
            ds: self.ds(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa40(pub u32);
impl PadPa40 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode"]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive"]
    #[inline(always)]
    pub const fn ds(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive"]
    #[inline(always)]
    pub fn set_ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa40 {
    #[inline(always)]
    fn default() -> PadPa40 {
        PadPa40(0)
    }
}
impl core::fmt::Debug for PadPa40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa40")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("mode", &self.mode())
            .field("ds", &self.ds())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa40 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa40 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            mode: bool,
            ds: bool,
            poe: bool,
        }
        let proxy = PadPa40 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            mode: self.mode(),
            ds: self.ds(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa41(pub u32);
impl PadPa41 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode"]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive"]
    #[inline(always)]
    pub const fn ds(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive"]
    #[inline(always)]
    pub fn set_ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa41 {
    #[inline(always)]
    fn default() -> PadPa41 {
        PadPa41(0)
    }
}
impl core::fmt::Debug for PadPa41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa41")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("mode", &self.mode())
            .field("ds", &self.ds())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa41 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa41 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            mode: bool,
            ds: bool,
            poe: bool,
        }
        let proxy = PadPa41 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            mode: self.mode(),
            ds: self.ds(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa42(pub u32);
impl PadPa42 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode"]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Select. Logic LOW enables GPIO mode,logic HIGH enables I2C mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive"]
    #[inline(always)]
    pub const fn ds(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select. Logic LOW selects 4mA drive,logic HIGH selects 20mA drive"]
    #[inline(always)]
    pub fn set_ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa42 {
    #[inline(always)]
    fn default() -> PadPa42 {
        PadPa42(0)
    }
}
impl core::fmt::Debug for PadPa42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa42")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("mode", &self.mode())
            .field("ds", &self.ds())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa42 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa42 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            mode: bool,
            ds: bool,
            poe: bool,
        }
        let proxy = PadPa42 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            mode: self.mode(),
            ds: self.ds(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa43(pub u32);
impl PadPa43 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa43 {
    #[inline(always)]
    fn default() -> PadPa43 {
        PadPa43(0)
    }
}
impl core::fmt::Debug for PadPa43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa43")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa43 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa43 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa43 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa44(pub u32);
impl PadPa44 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPa44 {
    #[inline(always)]
    fn default() -> PadPa44 {
        PadPa44(0)
    }
}
impl core::fmt::Debug for PadPa44 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa44")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadPa44 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadPa44 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadPa44 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa00(pub u32);
impl PadSa00 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa00 {
    #[inline(always)]
    fn default() -> PadSa00 {
        PadSa00(0)
    }
}
impl core::fmt::Debug for PadSa00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa00")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa00 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa00 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa00 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa01(pub u32);
impl PadSa01 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa01 {
    #[inline(always)]
    fn default() -> PadSa01 {
        PadSa01(0)
    }
}
impl core::fmt::Debug for PadSa01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa01")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa01 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa01 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa01 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa02(pub u32);
impl PadSa02 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa02 {
    #[inline(always)]
    fn default() -> PadSa02 {
        PadSa02(0)
    }
}
impl core::fmt::Debug for PadSa02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa02")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa02 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa02 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa02 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa03(pub u32);
impl PadSa03 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa03 {
    #[inline(always)]
    fn default() -> PadSa03 {
        PadSa03(0)
    }
}
impl core::fmt::Debug for PadSa03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa03")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa03 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa03 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa03 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa04(pub u32);
impl PadSa04 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa04 {
    #[inline(always)]
    fn default() -> PadSa04 {
        PadSa04(0)
    }
}
impl core::fmt::Debug for PadSa04 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa04")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa04 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa04 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa04 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa05(pub u32);
impl PadSa05 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa05 {
    #[inline(always)]
    fn default() -> PadSa05 {
        PadSa05(0)
    }
}
impl core::fmt::Debug for PadSa05 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa05")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa05 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa05 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa05 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa06(pub u32);
impl PadSa06 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa06 {
    #[inline(always)]
    fn default() -> PadSa06 {
        PadSa06(0)
    }
}
impl core::fmt::Debug for PadSa06 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa06")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa06 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa06 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa06 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa07(pub u32);
impl PadSa07 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa07 {
    #[inline(always)]
    fn default() -> PadSa07 {
        PadSa07(0)
    }
}
impl core::fmt::Debug for PadSa07 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa07")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa07 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa07 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa07 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa08(pub u32);
impl PadSa08 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa08 {
    #[inline(always)]
    fn default() -> PadSa08 {
        PadSa08(0)
    }
}
impl core::fmt::Debug for PadSa08 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa08")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa08 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa08 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa08 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa09(pub u32);
impl PadSa09 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa09 {
    #[inline(always)]
    fn default() -> PadSa09 {
        PadSa09(0)
    }
}
impl core::fmt::Debug for PadSa09 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa09")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa09 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa09 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa09 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa10(pub u32);
impl PadSa10 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa10 {
    #[inline(always)]
    fn default() -> PadSa10 {
        PadSa10(0)
    }
}
impl core::fmt::Debug for PadSa10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa10")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa10 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa10 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa11(pub u32);
impl PadSa11 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa11 {
    #[inline(always)]
    fn default() -> PadSa11 {
        PadSa11(0)
    }
}
impl core::fmt::Debug for PadSa11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa11")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa11 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa11 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadSa12(pub u32);
impl PadSa12 {
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadSa12 {
    #[inline(always)]
    fn default() -> PadSa12 {
        PadSa12(0)
    }
}
impl core::fmt::Debug for PadSa12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadSa12")
            .field("fsel", &self.fsel())
            .field("pe", &self.pe())
            .field("ps", &self.ps())
            .field("ie", &self.ie())
            .field("is", &self.is())
            .field("sr", &self.sr())
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("poe", &self.poe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PadSa12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PadSa12 {
            fsel: u8,
            pe: bool,
            ps: bool,
            ie: bool,
            is: bool,
            sr: bool,
            ds0: bool,
            ds1: bool,
            poe: bool,
        }
        let proxy = PadSa12 {
            fsel: self.fsel(),
            pe: self.pe(),
            ps: self.ps(),
            ie: self.ie(),
            is: self.is(),
            sr: self.sr(),
            ds0: self.ds0(),
            ds1: self.ds1(),
            poe: self.poe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
