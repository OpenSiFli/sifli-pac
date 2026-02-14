#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Op {
    #[doc = "Direct write data"]
    Write = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Read then XOR with data and write back"]
    Xor = 0x04,
    #[doc = "Read then OR with data and write back"]
    Or = 0x05,
    #[doc = "Read then AND with data and write back"]
    And = 0x06,
    #[doc = "Read then add with data and write back"]
    Add = 0x07,
}
impl Op {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Op {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Op {
    #[inline(always)]
    fn from(val: u8) -> Op {
        Op::from_bits(val)
    }
}
impl From<Op> for u8 {
    #[inline(always)]
    fn from(val: Op) -> u8 {
        Op::to_bits(val)
    }
}
