#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockCore {
    #[doc = "Lock is free (returned when successfully acquiring lock)"]
    Unlocked = 0x0,
    #[doc = "Locked by HCPU (High Performance CPU)"]
    Hcpu = 0x01,
    #[doc = "Locked by LCPU (Low Power CPU)"]
    Lcpu = 0x02,
    #[doc = "Locked by BCPU (Bluetooth CPU)"]
    Bcpu = 0x03,
}
impl LockCore {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockCore {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockCore {
    #[inline(always)]
    fn from(val: u8) -> LockCore {
        LockCore::from_bits(val)
    }
}
impl From<LockCore> for u8 {
    #[inline(always)]
    fn from(val: LockCore) -> u8 {
        LockCore::to_bits(val)
    }
}
