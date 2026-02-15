#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpsel {
    #[doc = "Use clock selected by SEL_SYS"]
    SelSys = 0x0,
    #[doc = "Use WDT clock (low power)"]
    Wdt = 0x01,
}
impl Lpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpsel {
    #[inline(always)]
    fn from(val: u8) -> Lpsel {
        Lpsel::from_bits(val)
    }
}
impl From<Lpsel> for u8 {
    #[inline(always)]
    fn from(val: Lpsel) -> u8 {
        Lpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Perisel {
    #[doc = "Use HRC48 for peripheral clock"]
    Hrc48 = 0x0,
    #[doc = "Use HXT48 for peripheral clock"]
    Hxt48 = 0x01,
}
impl Perisel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Perisel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Perisel {
    #[inline(always)]
    fn from(val: u8) -> Perisel {
        Perisel::from_bits(val)
    }
}
impl From<Perisel> for u8 {
    #[inline(always)]
    fn from(val: Perisel) -> u8 {
        Perisel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ticksel {
    #[doc = "Use RTC clock"]
    ClkRtc = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Use HRC48"]
    Hrc48 = 0x02,
    #[doc = "Use HXT48"]
    Hxt48 = 0x03,
}
impl Ticksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ticksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ticksel {
    #[inline(always)]
    fn from(val: u8) -> Ticksel {
        Ticksel::from_bits(val)
    }
}
impl From<Ticksel> for u8 {
    #[inline(always)]
    fn from(val: Ticksel) -> u8 {
        Ticksel::to_bits(val)
    }
}
