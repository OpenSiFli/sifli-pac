#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dllstg {
    #[doc = "Multiply by 1 (24 MHz)"]
    Mul1 = 0x0,
    #[doc = "Multiply by 2 (48 MHz)"]
    Mul2 = 0x01,
    #[doc = "Multiply by 3 (72 MHz)"]
    Mul3 = 0x02,
    #[doc = "Multiply by 4 (96 MHz)"]
    Mul4 = 0x03,
    #[doc = "Multiply by 5 (120 MHz)"]
    Mul5 = 0x04,
    #[doc = "Multiply by 6 (144 MHz)"]
    Mul6 = 0x05,
    #[doc = "Multiply by 7 (168 MHz)"]
    Mul7 = 0x06,
    #[doc = "Multiply by 8 (192 MHz)"]
    Mul8 = 0x07,
    #[doc = "Multiply by 9 (216 MHz)"]
    Mul9 = 0x08,
    #[doc = "Multiply by 10 (240 MHz)"]
    Mul10 = 0x09,
    #[doc = "Multiply by 11 (264 MHz)"]
    Mul11 = 0x0a,
    #[doc = "Multiply by 12 (288 MHz)"]
    Mul12 = 0x0b,
    #[doc = "Multiply by 13 (312 MHz)"]
    Mul13 = 0x0c,
    #[doc = "Multiply by 14 (336 MHz)"]
    Mul14 = 0x0d,
    #[doc = "Multiply by 15 (360 MHz)"]
    Mul15 = 0x0e,
    #[doc = "Multiply by 16 (384 MHz)"]
    Mul16 = 0x0f,
}
impl Dllstg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dllstg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dllstg {
    #[inline(always)]
    fn from(val: u8) -> Dllstg {
        Dllstg::from_bits(val)
    }
}
impl From<Dllstg> for u8 {
    #[inline(always)]
    fn from(val: Dllstg) -> u8 {
        Dllstg::to_bits(val)
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
    #[doc = "96MHz doubler (not implemented)"]
    Dbl96 = 0x02,
    #[doc = "DLL1 output"]
    Dll1 = 0x03,
}
impl Sysclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysclk {
        unsafe { core::mem::transmute(val & 0x03) }
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
