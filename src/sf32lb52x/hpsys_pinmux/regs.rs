#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa0_38(pub u32);
impl PadPa0_38 {
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
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
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
    pub const fn is(&self) -> super::vals::Is {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Is::from_bits(val as u8)
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: super::vals::Is) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> super::vals::Sr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sr::from_bits(val as u8)
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: super::vals::Sr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
impl Default for PadPa0_38 {
    #[inline(always)]
    fn default() -> PadPa0_38 {
        PadPa0_38(0)
    }
}
impl core::fmt::Debug for PadPa0_38 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa0_38")
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
impl defmt::Format for PadPa0_38 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PadPa0_38 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {:?}, ie: {=bool:?}, is: {:?}, sr: {:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa39_42(pub u32);
impl PadPa39_42 {
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
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
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
    pub const fn is(&self) -> super::vals::Is {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Is::from_bits(val as u8)
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: super::vals::Is) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
impl Default for PadPa39_42 {
    #[inline(always)]
    fn default() -> PadPa39_42 {
        PadPa39_42(0)
    }
}
impl core::fmt::Debug for PadPa39_42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa39_42")
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
impl defmt::Format for PadPa39_42 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PadPa39_42 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {:?}, ie: {=bool:?}, is: {:?}, mode: {=bool:?}, ds: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . mode () , self . ds () , self . poe ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPa43_44(pub u32);
impl PadPa43_44 {
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
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
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
    pub const fn is(&self) -> super::vals::Is {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Is::from_bits(val as u8)
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub fn set_is(&mut self, val: super::vals::Is) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn sr(&self) -> super::vals::Sr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sr::from_bits(val as u8)
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: super::vals::Sr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
impl Default for PadPa43_44 {
    #[inline(always)]
    fn default() -> PadPa43_44 {
        PadPa43_44(0)
    }
}
impl core::fmt::Debug for PadPa43_44 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPa43_44")
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
impl defmt::Format for PadPa43_44 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PadPa43_44 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {:?}, ie: {=bool:?}, is: {:?}, sr: {:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa00 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa01 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa02 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa03 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa04 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa05 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa06 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa07 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa08 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa09 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa10 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa11 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
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
        defmt :: write ! (f , "PadSa12 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
    }
}
