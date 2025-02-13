#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CCDS {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    OnCompare = 0x0,
    #[doc = "CCx DMA request sent when update event occurs"]
    OnUpdate = 0x01,
}
impl CCDS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CCDS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CCDS {
    #[inline(always)]
    fn from(val: u8) -> CCDS {
        CCDS::from_bits(val)
    }
}
impl From<CCDS> for u8 {
    #[inline(always)]
    fn from(val: CCDS) -> u8 {
        CCDS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMS {
    #[doc = "The counter counts up or down depending on the direction bit"]
    EdgeAligned = 0x0,
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
    CenterAligned1 = 0x01,
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
    CenterAligned2 = 0x02,
    #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
    CenterAligned3 = 0x03,
}
impl CMS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMS {
    #[inline(always)]
    fn from(val: u8) -> CMS {
        CMS::from_bits(val)
    }
}
impl From<CMS> for u8 {
    #[inline(always)]
    fn from(val: CMS) -> u8 {
        CMS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DIR {
    #[doc = "Counter used as upcounter"]
    Up = 0x0,
    #[doc = "Counter used as downcounter"]
    Down = 0x01,
}
impl DIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DIR {
    #[inline(always)]
    fn from(val: u8) -> DIR {
        DIR::from_bits(val)
    }
}
impl From<DIR> for u8 {
    #[inline(always)]
    fn from(val: DIR) -> u8 {
        DIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ETF {
    #[doc = "No filter, sampling is done at fDTS"]
    NoFilter = 0x0,
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    FCK_INT_N2 = 0x01,
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    FCK_INT_N4 = 0x02,
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    FCK_INT_N8 = 0x03,
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    FDTS_Div2_N6 = 0x04,
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    FDTS_Div2_N8 = 0x05,
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    FDTS_Div4_N6 = 0x06,
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    FDTS_Div4_N8 = 0x07,
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    FDTS_Div8_N6 = 0x08,
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    FDTS_Div8_N8 = 0x09,
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    FDTS_Div16_N5 = 0x0a,
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    FDTS_Div16_N6 = 0x0b,
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    FDTS_Div16_N8 = 0x0c,
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    FDTS_Div32_N5 = 0x0d,
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    FDTS_Div32_N6 = 0x0e,
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    FDTS_Div32_N8 = 0x0f,
}
impl ETF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ETF {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ETF {
    #[inline(always)]
    fn from(val: u8) -> ETF {
        ETF::from_bits(val)
    }
}
impl From<ETF> for u8 {
    #[inline(always)]
    fn from(val: ETF) -> u8 {
        ETF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ETP {
    #[doc = "ETR is noninverted, active at high level or rising edge"]
    NotInverted = 0x0,
    #[doc = "ETR is inverted, active at low level or falling edge"]
    Inverted = 0x01,
}
impl ETP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ETP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ETP {
    #[inline(always)]
    fn from(val: u8) -> ETP {
        ETP::from_bits(val)
    }
}
impl From<ETP> for u8 {
    #[inline(always)]
    fn from(val: ETP) -> u8 {
        ETP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ETPS {
    #[doc = "Prescaler OFF"]
    Div1 = 0x0,
    #[doc = "ETRP frequency divided by 2"]
    Div2 = 0x01,
    #[doc = "ETRP frequency divided by 4"]
    Div4 = 0x02,
    #[doc = "ETRP frequency divided by 8"]
    Div8 = 0x03,
}
impl ETPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ETPS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ETPS {
    #[inline(always)]
    fn from(val: u8) -> ETPS {
        ETPS::from_bits(val)
    }
}
impl From<ETPS> for u8 {
    #[inline(always)]
    fn from(val: ETPS) -> u8 {
        ETPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MMS {
    #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
    Reset = 0x0,
    #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
    Enable = 0x01,
    #[doc = "The update event is selected as trigger output"]
    Update = 0x02,
    #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    ComparePulse = 0x03,
    #[doc = "OC1REF signal is used as trigger output"]
    CompareOC1 = 0x04,
    #[doc = "OC2REF signal is used as trigger output"]
    CompareOC2 = 0x05,
    #[doc = "OC3REF signal is used as trigger output"]
    CompareOC3 = 0x06,
    #[doc = "OC4REF signal is used as trigger output"]
    CompareOC4 = 0x07,
}
impl MMS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MMS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MMS {
    #[inline(always)]
    fn from(val: u8) -> MMS {
        MMS::from_bits(val)
    }
}
impl From<MMS> for u8 {
    #[inline(always)]
    fn from(val: MMS) -> u8 {
        MMS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSM {
    #[doc = "No action"]
    NoSync = 0x0,
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    Sync = 0x01,
}
impl MSM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSM {
    #[inline(always)]
    fn from(val: u8) -> MSM {
        MSM::from_bits(val)
    }
}
impl From<MSM> for u8 {
    #[inline(always)]
    fn from(val: MSM) -> u8 {
        MSM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SMS {
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    Disabled = 0x0,
    #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
    Encoder_Mode_1 = 0x01,
    #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
    Encoder_Mode_2 = 0x02,
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    Encoder_Mode_3 = 0x03,
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    Reset_Mode = 0x04,
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    Gated_Mode = 0x05,
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    Trigger_Mode = 0x06,
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    Ext_Clock_Mode = 0x07,
    #[doc = "Rising edge of the selected trigger input (tim_trgi) reinitializes the counter, generates an update of the registers and starts the counter."]
    Combined_Reset_Trigger = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SMS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SMS {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SMS {
    #[inline(always)]
    fn from(val: u8) -> SMS {
        SMS::from_bits(val)
    }
}
impl From<SMS> for u8 {
    #[inline(always)]
    fn from(val: SMS) -> u8 {
        SMS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TI1S {
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    Normal = 0x0,
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    XOR = 0x01,
}
impl TI1S {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TI1S {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TI1S {
    #[inline(always)]
    fn from(val: u8) -> TI1S {
        TI1S::from_bits(val)
    }
}
impl From<TI1S> for u8 {
    #[inline(always)]
    fn from(val: TI1S) -> u8 {
        TI1S::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TS {
    #[doc = "Internal Trigger 0"]
    ITR0 = 0x0,
    #[doc = "Internal Trigger 1"]
    ITR1 = 0x01,
    #[doc = "Internal Trigger 2"]
    ITR2 = 0x02,
    #[doc = "Internal Trigger 3"]
    ITR3 = 0x03,
    #[doc = "TI1 Edge Detector"]
    TI1F_ED = 0x04,
    #[doc = "Filtered Timer Input 1"]
    TI1FP1 = 0x05,
    #[doc = "Filtered Timer Input 2"]
    TI2FP2 = 0x06,
    #[doc = "External Trigger input"]
    ETRF = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl TS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TS {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TS {
    #[inline(always)]
    fn from(val: u8) -> TS {
        TS::from_bits(val)
    }
}
impl From<TS> for u8 {
    #[inline(always)]
    fn from(val: TS) -> u8 {
        TS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum URS {
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    AnyEvent = 0x0,
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    CounterOnly = 0x01,
}
impl URS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> URS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for URS {
    #[inline(always)]
    fn from(val: u8) -> URS {
        URS::from_bits(val)
    }
}
impl From<URS> for u8 {
    #[inline(always)]
    fn from(val: URS) -> u8 {
        URS::to_bits(val)
    }
}
