#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M {
    Bit6 = 0x0,
    Bit7 = 0x01,
    #[doc = "1 start bit, 8 data bits, n stop bits"]
    Bit8 = 0x02,
    #[doc = "1 start bit, 9 data bits, n stop bits"]
    Bit9 = 0x03,
}
impl M {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M {
    #[inline(always)]
    fn from(val: u8) -> M {
        M::from_bits(val)
    }
}
impl From<M> for u8 {
    #[inline(always)]
    fn from(val: M) -> u8 {
        M::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OVER8 {
    #[doc = "Oversampling by 16"]
    Oversampling16 = 0x0,
    #[doc = "Oversampling by 8"]
    Oversampling8 = 0x01,
}
impl OVER8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OVER8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OVER8 {
    #[inline(always)]
    fn from(val: u8) -> OVER8 {
        OVER8::from_bits(val)
    }
}
impl From<OVER8> for u8 {
    #[inline(always)]
    fn from(val: OVER8) -> u8 {
        OVER8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PS {
    #[doc = "Even parity"]
    Even = 0x0,
    #[doc = "Odd parity"]
    Odd = 0x01,
}
impl PS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PS {
    #[inline(always)]
    fn from(val: u8) -> PS {
        PS::from_bits(val)
    }
}
impl From<PS> for u8 {
    #[inline(always)]
    fn from(val: PS) -> u8 {
        PS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STOP {
    #[doc = "1 stop bit"]
    Stop1 = 0x0,
    #[doc = "1 stop bit"]
    Stop1_ = 0x01,
    #[doc = "2 stop bits"]
    Stop2 = 0x02,
    #[doc = "2 stop bits"]
    Stop2_ = 0x03,
}
impl STOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STOP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STOP {
    #[inline(always)]
    fn from(val: u8) -> STOP {
        STOP::from_bits(val)
    }
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(val: STOP) -> u8 {
        STOP::to_bits(val)
    }
}
