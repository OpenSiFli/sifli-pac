#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaDataSel {
    #[doc = "DMA data is combined data"]
    Combined = 0x0,
    #[doc = "DMA data is raw data"]
    Raw = 0x01,
}
impl DmaDataSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaDataSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaDataSel {
    #[inline(always)]
    fn from(val: u8) -> DmaDataSel {
        DmaDataSel::from_bits(val)
    }
}
impl From<DmaDataSel> for u8 {
    #[inline(always)]
    fn from(val: DmaDataSel) -> u8 {
        DmaDataSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpMode {
    #[doc = "Single conversion mode"]
    Single = 0x0,
    #[doc = "Continuous conversion mode"]
    Continuous = 0x01,
}
impl OpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpMode {
    #[inline(always)]
    fn from(val: u8) -> OpMode {
        OpMode::from_bits(val)
    }
}
impl From<OpMode> for u8 {
    #[inline(always)]
    fn from(val: OpMode) -> u8 {
        OpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimerTrigTyp {
    #[doc = "Timer trigger type is pulse, no edge detect needed"]
    Pulse = 0x0,
    #[doc = "Timer trigger type is level, edge detect needed"]
    Level = 0x01,
}
impl TimerTrigTyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimerTrigTyp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimerTrigTyp {
    #[inline(always)]
    fn from(val: u8) -> TimerTrigTyp {
        TimerTrigTyp::from_bits(val)
    }
}
impl From<TimerTrigTyp> for u8 {
    #[inline(always)]
    fn from(val: TimerTrigTyp) -> u8 {
        TimerTrigTyp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vsp {
    #[doc = "Set comparator input CM in sampling phase to 0.539V"]
    V0_539 = 0x0,
    #[doc = "Set comparator input CM in sampling phase to 0.578V"]
    V0_578 = 0x01,
    #[doc = "Set comparator input CM in sampling phase to 0.642V"]
    V0_642 = 0x02,
    #[doc = "Set comparator input CM in sampling phase to 0.784V"]
    V0_784 = 0x03,
}
impl Vsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vsp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vsp {
    #[inline(always)]
    fn from(val: u8) -> Vsp {
        Vsp::from_bits(val)
    }
}
impl From<Vsp> for u8 {
    #[inline(always)]
    fn from(val: Vsp) -> u8 {
        Vsp::to_bits(val)
    }
}
