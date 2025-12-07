#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelPeri {
    Hrc48 = 0x0,
    Hxt48 = 0x01,
}
impl SelPeri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelPeri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelPeri {
    #[inline(always)]
    fn from(val: u8) -> SelPeri {
        SelPeri::from_bits(val)
    }
}
impl From<SelPeri> for u8 {
    #[inline(always)]
    fn from(val: SelPeri) -> u8 {
        SelPeri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelSys {
    Hrc48 = 0x0,
    Hxt48 = 0x01,
}
impl SelSys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelSys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelSys {
    #[inline(always)]
    fn from(val: u8) -> SelSys {
        SelSys::from_bits(val)
    }
}
impl From<SelSys> for u8 {
    #[inline(always)]
    fn from(val: SelSys) -> u8 {
        SelSys::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelTick {
    ClkRtc = 0x0,
    _RESERVED_1 = 0x01,
    Hrc48 = 0x02,
    Hxt48 = 0x03,
}
impl SelTick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelTick {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelTick {
    #[inline(always)]
    fn from(val: u8) -> SelTick {
        SelTick::from_bits(val)
    }
}
impl From<SelTick> for u8 {
    #[inline(always)]
    fn from(val: SelTick) -> u8 {
        SelTick::to_bits(val)
    }
}
