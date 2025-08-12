#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Burst {
    #[doc = "Single transfer"]
    Single = 0x0,
    #[doc = "Incremental burst of 4 beats"]
    Incr4 = 0x01,
    #[doc = "Incremental burst of 8 beats"]
    Incr8 = 0x02,
    #[doc = "Incremental burst of 16 beats"]
    Incr16 = 0x03,
}
impl Burst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Burst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Burst {
    #[inline(always)]
    fn from(val: u8) -> Burst {
        Burst::from_bits(val)
    }
}
impl From<Burst> for u8 {
    #[inline(always)]
    fn from(val: Burst) -> u8 {
        Burst::to_bits(val)
    }
}
#[doc = "Should be fixed to 10 (32 bits), word access allowed only."]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Size {
    Bits8 = 0x0,
    Bits16 = 0x01,
    Bits32 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Size {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Size {
    #[inline(always)]
    fn from(val: u8) -> Size {
        Size::from_bits(val)
    }
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(val: Size) -> u8 {
        Size::to_bits(val)
    }
}
