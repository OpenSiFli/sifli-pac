#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPb00(pub u32);
impl PadPb00 {
    #[doc = "Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[must_use]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[must_use]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[must_use]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[must_use]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPb00 {
    #[inline(always)]
    fn default() -> PadPb00 {
        PadPb00(0)
    }
}
impl core::fmt::Debug for PadPb00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPb00")
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
impl defmt::Format for PadPb00 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PadPb00 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPb01(pub u32);
impl PadPb01 {
    #[doc = "Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[must_use]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[must_use]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[must_use]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[must_use]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPb01 {
    #[inline(always)]
    fn default() -> PadPb01 {
        PadPb01(0)
    }
}
impl core::fmt::Debug for PadPb01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPb01")
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
impl defmt::Format for PadPb01 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PadPb01 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPb02(pub u32);
impl PadPb02 {
    #[doc = "Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[must_use]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[must_use]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[must_use]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[must_use]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPb02 {
    #[inline(always)]
    fn default() -> PadPb02 {
        PadPb02(0)
    }
}
impl core::fmt::Debug for PadPb02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPb02")
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
impl defmt::Format for PadPb02 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PadPb02 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadPb03(pub u32);
impl PadPb03 {
    #[doc = "Function Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Function Select"]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Enable. Logic HIGH enables week pull device"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pull Select. Logic HIGH selects pull-up, logic LOW select pull-down"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable. Logic HIGH enables the input buffer"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[must_use]
    #[inline(always)]
    pub const fn is(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input Select. Logic LOW selects CMOS input, logic HIGH selects Schmitt input"]
    #[inline(always)]
    pub const fn set_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate. Logic HIGH selects slow slew rate, logic LOW selects fast slew rate"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[must_use]
    #[inline(always)]
    pub const fn ds0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 0. Used to select output drive strength"]
    #[inline(always)]
    pub const fn set_ds0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[must_use]
    #[inline(always)]
    pub const fn ds1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive Select 1. Used to select output drive strength"]
    #[inline(always)]
    pub const fn set_ds1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[must_use]
    #[inline(always)]
    pub const fn poe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved. Always set to logic LOW"]
    #[inline(always)]
    pub const fn set_poe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for PadPb03 {
    #[inline(always)]
    fn default() -> PadPb03 {
        PadPb03(0)
    }
}
impl core::fmt::Debug for PadPb03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PadPb03")
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
impl defmt::Format for PadPb03 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PadPb03 {{ fsel: {=u8:?}, pe: {=bool:?}, ps: {=bool:?}, ie: {=bool:?}, is: {=bool:?}, sr: {=bool:?}, ds0: {=bool:?}, ds1: {=bool:?}, poe: {=bool:?} }}" , self . fsel () , self . pe () , self . ps () , self . ie () , self . is () , self . sr () , self . ds0 () , self . ds1 () , self . poe ())
    }
}
