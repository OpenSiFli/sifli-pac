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
pub enum Rtcsel {
    #[doc = "Use LRC10K (100 kHz)"]
    Lrc10 = 0x0,
    #[doc = "Use LXT32K (32.768 kHz)"]
    Lxt32 = 0x01,
}
impl Rtcsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcsel {
    #[inline(always)]
    fn from(val: u8) -> Rtcsel {
        Rtcsel::from_bits(val)
    }
}
impl From<Rtcsel> for u8 {
    #[inline(always)]
    fn from(val: Rtcsel) -> u8 {
        Rtcsel::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdtsel {
    #[doc = "Use LRC10K (100 kHz)"]
    Lrc10 = 0x0,
    #[doc = "Use LRC32K (32 kHz)"]
    Lrc32 = 0x01,
}
impl Wdtsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdtsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdtsel {
    #[inline(always)]
    fn from(val: u8) -> Wdtsel {
        Wdtsel::from_bits(val)
    }
}
impl From<Wdtsel> for u8 {
    #[inline(always)]
    fn from(val: Wdtsel) -> u8 {
        Wdtsel::to_bits(val)
    }
}
