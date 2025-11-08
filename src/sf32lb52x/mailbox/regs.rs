#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exr(pub u32);
impl Exr {
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ex(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ex(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Exr {
    #[inline(always)]
    fn default() -> Exr {
        Exr(0)
    }
}
impl core::fmt::Debug for Exr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Exr")
            .field("id", &self.id())
            .field("ex", &self.ex())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Exr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Exr {{ id: {=u8:?}, ex: {=bool:?} }}",
            self.id(),
            self.ex()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ixr(pub u32);
impl Ixr {
    #[must_use]
    #[inline(always)]
    pub const fn int(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_int(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ixr {
    #[inline(always)]
    fn default() -> Ixr {
        Ixr(0)
    }
}
impl core::fmt::Debug for Ixr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ixr")
            .field("int[0]", &self.int(0usize))
            .field("int[1]", &self.int(1usize))
            .field("int[2]", &self.int(2usize))
            .field("int[3]", &self.int(3usize))
            .field("int[4]", &self.int(4usize))
            .field("int[5]", &self.int(5usize))
            .field("int[6]", &self.int(6usize))
            .field("int[7]", &self.int(7usize))
            .field("int[8]", &self.int(8usize))
            .field("int[9]", &self.int(9usize))
            .field("int[10]", &self.int(10usize))
            .field("int[11]", &self.int(11usize))
            .field("int[12]", &self.int(12usize))
            .field("int[13]", &self.int(13usize))
            .field("int[14]", &self.int(14usize))
            .field("int[15]", &self.int(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ixr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ixr {{ int[0]: {=bool:?}, int[1]: {=bool:?}, int[2]: {=bool:?}, int[3]: {=bool:?}, int[4]: {=bool:?}, int[5]: {=bool:?}, int[6]: {=bool:?}, int[7]: {=bool:?}, int[8]: {=bool:?}, int[9]: {=bool:?}, int[10]: {=bool:?}, int[11]: {=bool:?}, int[12]: {=bool:?}, int[13]: {=bool:?}, int[14]: {=bool:?}, int[15]: {=bool:?} }}" , self . int (0usize) , self . int (1usize) , self . int (2usize) , self . int (3usize) , self . int (4usize) , self . int (5usize) , self . int (6usize) , self . int (7usize) , self . int (8usize) , self . int (9usize) , self . int (10usize) , self . int (11usize) , self . int (12usize) , self . int (13usize) , self . int (14usize) , self . int (15usize))
    }
}
