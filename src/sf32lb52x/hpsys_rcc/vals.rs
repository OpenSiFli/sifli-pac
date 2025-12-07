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
pub enum Mpisel {
    #[doc = "Use clk_peri_hpsys"]
    Peri = 0x0,
    #[doc = "Use DLL1 clock"]
    Dll1 = 0x01,
    #[doc = "Use DLL2 clock"]
    Dll2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mpisel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mpisel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mpisel {
    #[inline(always)]
    fn from(val: u8) -> Mpisel {
        Mpisel::from_bits(val)
    }
}
impl From<Mpisel> for u8 {
    #[inline(always)]
    fn from(val: Mpisel) -> u8 {
        Mpisel::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbsel {
    #[doc = "Use system clock"]
    Sysclk = 0x0,
    #[doc = "Use DLL2 clock"]
    Dll2 = 0x01,
}
impl Usbsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbsel {
    #[inline(always)]
    fn from(val: u8) -> Usbsel {
        Usbsel::from_bits(val)
    }
}
impl From<Usbsel> for u8 {
    #[inline(always)]
    fn from(val: Usbsel) -> u8 {
        Usbsel::to_bits(val)
    }
}
