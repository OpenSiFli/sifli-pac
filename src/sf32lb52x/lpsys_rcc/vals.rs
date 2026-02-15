#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hdiv {
    #[doc = "Divide by 1"]
    Div1 = 0x0,
    #[doc = "Divide by 2"]
    Div2 = 0x01,
    #[doc = "Divide by 3"]
    Div3 = 0x02,
    #[doc = "Divide by 4"]
    Div4 = 0x03,
    #[doc = "Divide by 5"]
    Div5 = 0x04,
    #[doc = "Divide by 6"]
    Div6 = 0x05,
    #[doc = "Divide by 7"]
    Div7 = 0x06,
    #[doc = "Divide by 8"]
    Div8 = 0x07,
    #[doc = "Divide by 9"]
    Div9 = 0x08,
    #[doc = "Divide by 10"]
    Div10 = 0x09,
    #[doc = "Divide by 11"]
    Div11 = 0x0a,
    #[doc = "Divide by 12"]
    Div12 = 0x0b,
    #[doc = "Divide by 13"]
    Div13 = 0x0c,
    #[doc = "Divide by 14"]
    Div14 = 0x0d,
    #[doc = "Divide by 15"]
    Div15 = 0x0e,
    #[doc = "Divide by 16"]
    Div16 = 0x0f,
}
impl Hdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hdiv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hdiv {
    #[inline(always)]
    fn from(val: u8) -> Hdiv {
        Hdiv::from_bits(val)
    }
}
impl From<Hdiv> for u8 {
    #[inline(always)]
    fn from(val: Hdiv) -> u8 {
        Hdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Macdiv {
    _RESERVED_0 = 0x0,
    #[doc = "Divide by 1"]
    Div1 = 0x01,
    #[doc = "Divide by 2"]
    Div2 = 0x02,
    #[doc = "Divide by 3"]
    Div3 = 0x03,
    #[doc = "Divide by 4"]
    Div4 = 0x04,
    #[doc = "Divide by 5"]
    Div5 = 0x05,
    #[doc = "Divide by 6"]
    Div6 = 0x06,
    #[doc = "Divide by 7"]
    Div7 = 0x07,
    #[doc = "Divide by 8"]
    Div8 = 0x08,
    #[doc = "Divide by 9"]
    Div9 = 0x09,
    #[doc = "Divide by 10"]
    Div10 = 0x0a,
    #[doc = "Divide by 11"]
    Div11 = 0x0b,
    #[doc = "Divide by 12"]
    Div12 = 0x0c,
    #[doc = "Divide by 13"]
    Div13 = 0x0d,
    #[doc = "Divide by 14"]
    Div14 = 0x0e,
    #[doc = "Divide by 15"]
    Div15 = 0x0f,
}
impl Macdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Macdiv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Macdiv {
    #[inline(always)]
    fn from(val: u8) -> Macdiv {
        Macdiv::from_bits(val)
    }
}
impl From<Macdiv> for u8 {
    #[inline(always)]
    fn from(val: Macdiv) -> u8 {
        Macdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdiv {
    #[doc = "Divide by 1"]
    Div1 = 0x0,
    #[doc = "Divide by 2"]
    Div2 = 0x01,
    #[doc = "Divide by 4"]
    Div4 = 0x02,
    #[doc = "Divide by 8"]
    Div8 = 0x03,
    #[doc = "Divide by 16"]
    Div16 = 0x04,
    #[doc = "Divide by 32"]
    Div32 = 0x05,
    #[doc = "Divide by 64"]
    Div64 = 0x06,
    #[doc = "Divide by 128"]
    Div128 = 0x07,
}
impl Pdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdiv {
    #[inline(always)]
    fn from(val: u8) -> Pdiv {
        Pdiv::from_bits(val)
    }
}
impl From<Pdiv> for u8 {
    #[inline(always)]
    fn from(val: Pdiv) -> u8 {
        Pdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysclk {
    #[doc = "48MHz internal RC oscillator"]
    Hrc48 = 0x0,
    #[doc = "48MHz external crystal"]
    Hxt48 = 0x01,
}
impl Sysclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysclk {
    #[inline(always)]
    fn from(val: u8) -> Sysclk {
        Sysclk::from_bits(val)
    }
}
impl From<Sysclk> for u8 {
    #[inline(always)]
    fn from(val: Sysclk) -> u8 {
        Sysclk::to_bits(val)
    }
}
pub mod mux;
