#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Is {
    Cmos = 0x0,
    Schmitt = 0x01,
}
impl Is {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Is {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Is {
    #[inline(always)]
    fn from(val: u8) -> Is {
        Is::from_bits(val)
    }
}
impl From<Is> for u8 {
    #[inline(always)]
    fn from(val: Is) -> u8 {
        Is::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    Down = 0x0,
    Up = 0x01,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sr {
    Fast = 0x0,
    Slow = 0x01,
}
impl Sr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sr {
    #[inline(always)]
    fn from(val: u8) -> Sr {
        Sr::from_bits(val)
    }
}
impl From<Sr> for u8 {
    #[inline(always)]
    fn from(val: Sr) -> u8 {
        Sr::to_bits(val)
    }
}
