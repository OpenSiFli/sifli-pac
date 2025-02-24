#[doc = "Bank3 Data7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anacr(pub u32);
impl Anacr {
    #[inline(always)]
    pub const fn ldo_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn ldo_vref_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_ldo_vref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[inline(always)]
    pub const fn ldo_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ldo_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn ldo_dc_tr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_ldo_dc_tr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn reserve0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_reserve0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[inline(always)]
    pub const fn reserve1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_reserve1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Anacr {
    #[inline(always)]
    fn default() -> Anacr {
        Anacr(0)
    }
}
impl core::fmt::Debug for Anacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Anacr")
            .field("ldo_en", &self.ldo_en())
            .field("ldo_vref_sel", &self.ldo_vref_sel())
            .field("ldo_mode", &self.ldo_mode())
            .field("ldo_dc_tr", &self.ldo_dc_tr())
            .field("reserve0", &self.reserve0())
            .field("reserve1", &self.reserve1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Anacr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Anacr {{ ldo_en: {=bool:?}, ldo_vref_sel: {=u8:?}, ldo_mode: {=bool:?}, ldo_dc_tr: {=u8:?}, reserve0: {=u8:?}, reserve1: {=u8:?} }}" , self . ldo_en () , self . ldo_vref_sel () , self . ldo_mode () , self . ldo_dc_tr () , self . reserve0 () , self . reserve1 ())
    }
}
#[doc = "Bank0 Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank0Data0(pub u32);
impl Bank0Data0 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank0Data0 {
    #[inline(always)]
    fn default() -> Bank0Data0 {
        Bank0Data0(0)
    }
}
impl core::fmt::Debug for Bank0Data0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank0Data0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank0Data0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank0Data0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank0 Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank0Data1(pub u32);
impl Bank0Data1 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank0Data1 {
    #[inline(always)]
    fn default() -> Bank0Data1 {
        Bank0Data1(0)
    }
}
impl core::fmt::Debug for Bank0Data1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank0Data1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank0Data1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank0Data1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank0 Data2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank0Data2(pub u32);
impl Bank0Data2 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank0Data2 {
    #[inline(always)]
    fn default() -> Bank0Data2 {
        Bank0Data2(0)
    }
}
impl core::fmt::Debug for Bank0Data2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank0Data2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank0Data2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank0Data2 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank0 Data3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank0Data3(pub u32);
impl Bank0Data3 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank0Data3 {
    #[inline(always)]
    fn default() -> Bank0Data3 {
        Bank0Data3(0)
    }
}
impl core::fmt::Debug for Bank0Data3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank0Data3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank0Data3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank0Data3 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank0 Data4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank0Data4(pub u32);
impl Bank0Data4 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank0Data4 {
    #[inline(always)]
    fn default() -> Bank0Data4 {
        Bank0Data4(0)
    }
}
impl core::fmt::Debug for Bank0Data4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank0Data4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank0Data4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank0Data4 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank0 Data5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank0Data5(pub u32);
impl Bank0Data5 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank0Data5 {
    #[inline(always)]
    fn default() -> Bank0Data5 {
        Bank0Data5(0)
    }
}
impl core::fmt::Debug for Bank0Data5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank0Data5")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank0Data5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank0Data5 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank0 Data6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank0Data6(pub u32);
impl Bank0Data6 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank0Data6 {
    #[inline(always)]
    fn default() -> Bank0Data6 {
        Bank0Data6(0)
    }
}
impl core::fmt::Debug for Bank0Data6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank0Data6")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank0Data6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank0Data6 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank0 Data7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank0Data7(pub u32);
impl Bank0Data7 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank0Data7 {
    #[inline(always)]
    fn default() -> Bank0Data7 {
        Bank0Data7(0)
    }
}
impl core::fmt::Debug for Bank0Data7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank0Data7")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank0Data7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank0Data7 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank1 Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank1Data0(pub u32);
impl Bank1Data0 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank1Data0 {
    #[inline(always)]
    fn default() -> Bank1Data0 {
        Bank1Data0(0)
    }
}
impl core::fmt::Debug for Bank1Data0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank1Data0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank1Data0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank1Data0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank1 Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank1Data1(pub u32);
impl Bank1Data1 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank1Data1 {
    #[inline(always)]
    fn default() -> Bank1Data1 {
        Bank1Data1(0)
    }
}
impl core::fmt::Debug for Bank1Data1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank1Data1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank1Data1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank1Data1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank1 Data2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank1Data2(pub u32);
impl Bank1Data2 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank1Data2 {
    #[inline(always)]
    fn default() -> Bank1Data2 {
        Bank1Data2(0)
    }
}
impl core::fmt::Debug for Bank1Data2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank1Data2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank1Data2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank1Data2 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank1 Data3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank1Data3(pub u32);
impl Bank1Data3 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank1Data3 {
    #[inline(always)]
    fn default() -> Bank1Data3 {
        Bank1Data3(0)
    }
}
impl core::fmt::Debug for Bank1Data3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank1Data3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank1Data3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank1Data3 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank1 Data4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank1Data4(pub u32);
impl Bank1Data4 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank1Data4 {
    #[inline(always)]
    fn default() -> Bank1Data4 {
        Bank1Data4(0)
    }
}
impl core::fmt::Debug for Bank1Data4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank1Data4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank1Data4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank1Data4 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank1 Data5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank1Data5(pub u32);
impl Bank1Data5 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank1Data5 {
    #[inline(always)]
    fn default() -> Bank1Data5 {
        Bank1Data5(0)
    }
}
impl core::fmt::Debug for Bank1Data5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank1Data5")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank1Data5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank1Data5 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank1 Data6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank1Data6(pub u32);
impl Bank1Data6 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank1Data6 {
    #[inline(always)]
    fn default() -> Bank1Data6 {
        Bank1Data6(0)
    }
}
impl core::fmt::Debug for Bank1Data6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank1Data6")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank1Data6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank1Data6 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank1 Data7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank1Data7(pub u32);
impl Bank1Data7 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank1Data7 {
    #[inline(always)]
    fn default() -> Bank1Data7 {
        Bank1Data7(0)
    }
}
impl core::fmt::Debug for Bank1Data7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank1Data7")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank1Data7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank1Data7 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank2 Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank2Data0(pub u32);
impl Bank2Data0 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank2Data0 {
    #[inline(always)]
    fn default() -> Bank2Data0 {
        Bank2Data0(0)
    }
}
impl core::fmt::Debug for Bank2Data0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank2Data0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank2Data0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank2Data0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank2 Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank2Data1(pub u32);
impl Bank2Data1 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank2Data1 {
    #[inline(always)]
    fn default() -> Bank2Data1 {
        Bank2Data1(0)
    }
}
impl core::fmt::Debug for Bank2Data1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank2Data1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank2Data1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank2Data1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank2 Data2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank2Data2(pub u32);
impl Bank2Data2 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank2Data2 {
    #[inline(always)]
    fn default() -> Bank2Data2 {
        Bank2Data2(0)
    }
}
impl core::fmt::Debug for Bank2Data2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank2Data2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank2Data2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank2Data2 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank2 Data3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank2Data3(pub u32);
impl Bank2Data3 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank2Data3 {
    #[inline(always)]
    fn default() -> Bank2Data3 {
        Bank2Data3(0)
    }
}
impl core::fmt::Debug for Bank2Data3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank2Data3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank2Data3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank2Data3 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank2 Data4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank2Data4(pub u32);
impl Bank2Data4 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank2Data4 {
    #[inline(always)]
    fn default() -> Bank2Data4 {
        Bank2Data4(0)
    }
}
impl core::fmt::Debug for Bank2Data4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank2Data4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank2Data4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank2Data4 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank2 Data5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank2Data5(pub u32);
impl Bank2Data5 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank2Data5 {
    #[inline(always)]
    fn default() -> Bank2Data5 {
        Bank2Data5(0)
    }
}
impl core::fmt::Debug for Bank2Data5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank2Data5")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank2Data5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank2Data5 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank2 Data6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank2Data6(pub u32);
impl Bank2Data6 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank2Data6 {
    #[inline(always)]
    fn default() -> Bank2Data6 {
        Bank2Data6(0)
    }
}
impl core::fmt::Debug for Bank2Data6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank2Data6")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank2Data6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank2Data6 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank2 Data7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank2Data7(pub u32);
impl Bank2Data7 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank2Data7 {
    #[inline(always)]
    fn default() -> Bank2Data7 {
        Bank2Data7(0)
    }
}
impl core::fmt::Debug for Bank2Data7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank2Data7")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank2Data7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank2Data7 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank3 Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank3Data0(pub u32);
impl Bank3Data0 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank3Data0 {
    #[inline(always)]
    fn default() -> Bank3Data0 {
        Bank3Data0(0)
    }
}
impl core::fmt::Debug for Bank3Data0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank3Data0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank3Data0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank3Data0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank3 Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank3Data1(pub u32);
impl Bank3Data1 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank3Data1 {
    #[inline(always)]
    fn default() -> Bank3Data1 {
        Bank3Data1(0)
    }
}
impl core::fmt::Debug for Bank3Data1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank3Data1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank3Data1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank3Data1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank3 Data2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank3Data2(pub u32);
impl Bank3Data2 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank3Data2 {
    #[inline(always)]
    fn default() -> Bank3Data2 {
        Bank3Data2(0)
    }
}
impl core::fmt::Debug for Bank3Data2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank3Data2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank3Data2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank3Data2 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank3 Data3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank3Data3(pub u32);
impl Bank3Data3 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank3Data3 {
    #[inline(always)]
    fn default() -> Bank3Data3 {
        Bank3Data3(0)
    }
}
impl core::fmt::Debug for Bank3Data3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank3Data3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank3Data3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank3Data3 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank3 Data4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank3Data4(pub u32);
impl Bank3Data4 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank3Data4 {
    #[inline(always)]
    fn default() -> Bank3Data4 {
        Bank3Data4(0)
    }
}
impl core::fmt::Debug for Bank3Data4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank3Data4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank3Data4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank3Data4 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank3 Data5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank3Data5(pub u32);
impl Bank3Data5 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank3Data5 {
    #[inline(always)]
    fn default() -> Bank3Data5 {
        Bank3Data5(0)
    }
}
impl core::fmt::Debug for Bank3Data5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank3Data5")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank3Data5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank3Data5 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank3 Data6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank3Data6(pub u32);
impl Bank3Data6 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank3Data6 {
    #[inline(always)]
    fn default() -> Bank3Data6 {
        Bank3Data6(0)
    }
}
impl core::fmt::Debug for Bank3Data6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank3Data6")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank3Data6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank3Data6 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Bank3 Data7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank3Data7(pub u32);
impl Bank3Data7 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bank3Data7 {
    #[inline(always)]
    fn default() -> Bank3Data7 {
        Bank3Data7(0)
    }
}
impl core::fmt::Debug for Bank3Data7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bank3Data7")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bank3Data7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bank3Data7 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Write 1 to enable PGM/READ. Self clear"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to enable PGM/READ. Self clear"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 - READ, 1 - PGM"]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 - READ, 1 - PGM"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bank select"]
    #[inline(always)]
    pub const fn banksel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Bank select"]
    #[inline(always)]
    pub fn set_banksel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt enable"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("mode", &self.mode())
            .field("banksel", &self.banksel())
            .field("ie", &self.ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ en: {=bool:?}, mode: {=bool:?}, banksel: {=u8:?}, ie: {=bool:?} }}",
            self.en(),
            self.mode(),
            self.banksel(),
            self.ie()
        )
    }
}
#[doc = "debug signal select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbSel(pub u32);
impl DbSel {
    #[doc = "debug signal select"]
    #[inline(always)]
    pub const fn db_sel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "debug signal select"]
    #[inline(always)]
    pub fn set_db_sel(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbSel {
    #[inline(always)]
    fn default() -> DbSel {
        DbSel(0)
    }
}
impl core::fmt::Debug for DbSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbSel")
            .field("db_sel", &self.db_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DbSel {{ db_sel: {=u32:?} }}", self.db_sel())
    }
}
#[doc = "Program Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmData0(pub u32);
impl PgmData0 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmData0 {
    #[inline(always)]
    fn default() -> PgmData0 {
        PgmData0(0)
    }
}
impl core::fmt::Debug for PgmData0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmData0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmData0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmData0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Program Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmData1(pub u32);
impl PgmData1 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmData1 {
    #[inline(always)]
    fn default() -> PgmData1 {
        PgmData1(0)
    }
}
impl core::fmt::Debug for PgmData1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmData1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmData1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmData1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Program Data2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmData2(pub u32);
impl PgmData2 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmData2 {
    #[inline(always)]
    fn default() -> PgmData2 {
        PgmData2(0)
    }
}
impl core::fmt::Debug for PgmData2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmData2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmData2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmData2 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Program Data3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmData3(pub u32);
impl PgmData3 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmData3 {
    #[inline(always)]
    fn default() -> PgmData3 {
        PgmData3(0)
    }
}
impl core::fmt::Debug for PgmData3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmData3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmData3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmData3 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Program Data4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmData4(pub u32);
impl PgmData4 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmData4 {
    #[inline(always)]
    fn default() -> PgmData4 {
        PgmData4(0)
    }
}
impl core::fmt::Debug for PgmData4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmData4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmData4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmData4 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Program Data5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmData5(pub u32);
impl PgmData5 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmData5 {
    #[inline(always)]
    fn default() -> PgmData5 {
        PgmData5(0)
    }
}
impl core::fmt::Debug for PgmData5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmData5")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmData5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmData5 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Program Data6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmData6(pub u32);
impl PgmData6 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmData6 {
    #[inline(always)]
    fn default() -> PgmData6 {
        PgmData6(0)
    }
}
impl core::fmt::Debug for PgmData6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmData6")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmData6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmData6 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Program Data7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmData7(pub u32);
impl PgmData7 {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmData7 {
    #[inline(always)]
    fn default() -> PgmData7 {
        PgmData7(0)
    }
}
impl core::fmt::Debug for PgmData7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmData7")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmData7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmData7 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Reserved Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvdr(pub u32);
impl Rsvdr {}
impl Default for Rsvdr {
    #[inline(always)]
    fn default() -> Rsvdr {
        Rsvdr(0)
    }
}
impl core::fmt::Debug for Rsvdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvdr").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvdr {{ }}",)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Indicates PGM/READ done. Write 1 to clear"]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates PGM/READ done. Write 1 to clear"]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        f.debug_struct("Sr").field("done", &self.done()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sr {{ done: {=bool:?} }}", self.done())
    }
}
#[doc = "Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timr(pub u32);
impl Timr {
    #[doc = "SCLK to CSB hold time into READ mode. Recmmended value > 500ns"]
    #[inline(always)]
    pub const fn thrck(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "SCLK to CSB hold time into READ mode. Recmmended value > 500ns"]
    #[inline(always)]
    pub fn set_thrck(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "SCLK to CSB hold time into PGM mode. Recommended value > 20ns"]
    #[inline(always)]
    pub const fn thpck(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "SCLK to CSB hold time into PGM mode. Recommended value > 20ns"]
    #[inline(always)]
    pub fn set_thpck(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "SCLK high period for PGM. Recommended value ~10us"]
    #[inline(always)]
    pub const fn tckhp(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x07ff;
        val as u16
    }
    #[doc = "SCLK high period for PGM. Recommended value ~10us"]
    #[inline(always)]
    pub fn set_tckhp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 10usize)) | (((val as u32) & 0x07ff) << 10usize);
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
            .field("thrck", &self.thrck())
            .field("thpck", &self.thpck())
            .field("tckhp", &self.tckhp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timr {{ thrck: {=u8:?}, thpck: {=u8:?}, tckhp: {=u16:?} }}",
            self.thrck(),
            self.thpck(),
            self.tckhp()
        )
    }
}
