#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dir {
    #[doc = "Read from peripheral"]
    PeripheralToMemory = 0x0,
    #[doc = "Read from memory"]
    MemoryToPeripheral = 0x01,
}
impl Dir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dir {
    #[inline(always)]
    fn from(val: u8) -> Dir {
        Dir::from_bits(val)
    }
}
impl From<Dir> for u8 {
    #[inline(always)]
    fn from(val: Dir) -> u8 {
        Dir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pl {
    #[doc = "Low priority"]
    Low = 0x0,
    #[doc = "Medium priority"]
    Medium = 0x01,
    #[doc = "High priority"]
    High = 0x02,
    #[doc = "Very high priority"]
    VeryHigh = 0x03,
}
impl Pl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pl {
    #[inline(always)]
    fn from(val: u8) -> Pl {
        Pl::from_bits(val)
    }
}
impl From<Pl> for u8 {
    #[inline(always)]
    fn from(val: Pl) -> u8 {
        Pl::to_bits(val)
    }
}
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
