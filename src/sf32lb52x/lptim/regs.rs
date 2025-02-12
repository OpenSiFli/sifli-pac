#[doc = "LPTIM autoreload register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc = "Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CMP\\[15:0\\] value."]
    #[inline(always)]
    pub const fn arr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CMP\\[15:0\\] value."]
    #[inline(always)]
    pub fn set_arr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Arr {
    #[inline(always)]
    fn default() -> Arr {
        Arr(0)
    }
}
impl core::fmt::Debug for Arr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Arr").field("arr", &self.arr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Arr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Arr {
            arr: u32,
        }
        let proxy = Arr { arr: self.arr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM will use: 0: LPTIM is clocked by internal clock source, according to INTCKSEL 1: LPTIM is clocked by external clock source, according to EXTCKSEL"]
    #[inline(always)]
    pub const fn cksel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM will use: 0: LPTIM is clocked by internal clock source, according to INTCKSEL 1: LPTIM is clocked by external clock source, according to EXTCKSEL"]
    #[inline(always)]
    pub fn set_cksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock Polarity If LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: 00: the rising edge is the active edge used for counting 01: the falling edge is the active edge used for counting 10: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four time the external clock frequency. 11: not allowed"]
    #[inline(always)]
    pub const fn ckpol(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Clock Polarity If LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: 00: the rising edge is the active edge used for counting 01: the falling edge is the active edge used for counting 10: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four time the external clock frequency. 11: not allowed"]
    #[inline(always)]
    pub fn set_ckpol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any external clock signal level change is considered as a valid transition 01: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition. 10: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition. 11: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub const fn ckflt(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any external clock signal level change is considered as a valid transition 01: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition. 10: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition. 11: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn set_ckflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Internal clock source selector 0: internal clock source is clk_lp 1: internal clock source is pclk2"]
    #[inline(always)]
    pub const fn intcksel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Internal clock source selector 0: internal clock source is clk_lp 1: internal clock source is pclk2"]
    #[inline(always)]
    pub fn set_intcksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any trigger active level change is considered as a valid trigger 01: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger. 10: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger. 11: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub const fn trgflt(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature 00: any trigger active level change is considered as a valid trigger 01: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger. 10: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger. 11: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn set_trgflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "External clock source selector 0: external clock source is from lptim_in 1: external clock source is from LPCOMP (if LPCOMP integrated)"]
    #[inline(always)]
    pub const fn extcksel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "External clock source selector 0: external clock source is from lptim_in 1: external clock source is from LPCOMP (if LPCOMP integrated)"]
    #[inline(always)]
    pub fn set_extcksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors: 000: /1 001: /2 010: /4 011: /8 100: /16 101: /32 110: /64 111: /128"]
    #[inline(always)]
    pub const fn presc(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors: 000: /1 001: /2 010: /4 011: /8 100: /16 101: /32 110: /64 111: /128"]
    #[inline(always)]
    pub fn set_presc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: 000: lptim_ext0 001: lptim_ext1 010: lptim_ext2 011: lptim_ext3 100: lptim_ext4 101: lptim_ext5 110: lptim_ext6 111: lptim_ext7"]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: 000: lptim_ext0 001: lptim_ext1 010: lptim_ext2 011: lptim_ext3 100: lptim_ext4 101: lptim_ext5 110: lptim_ext6 111: lptim_ext7"]
    #[inline(always)]
    pub fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge: 00: software trigger (counting start is initiated by software) 01: rising edge is the active edge 10: falling edge is the active edge 11: both edges are active edges"]
    #[inline(always)]
    pub const fn trigen(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge: 00: software trigger (counting start is initiated by software) 01: rising edge is the active edge 10: falling edge is the active edge 11: both edges are active edges"]
    #[inline(always)]
    pub fn set_trigen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "Timeout enable The TIMOUT bit controls the Timeout feature 0: A trigger event arriving when the timer is already started will be ignored 1: A trigger event arriving when the timer is already started will reset and restart the LPTIM counter and the repetition counter"]
    #[inline(always)]
    pub const fn timout(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout enable The TIMOUT bit controls the Timeout feature 0: A trigger event arriving when the timer is already started will be ignored 1: A trigger event arriving when the timer is already started will reset and restart the LPTIM counter and the repetition counter"]
    #[inline(always)]
    pub fn set_timout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Waveform shape The WAVE bit controls the output shape 0: Deactivate Set-once mode 1: Activate the Set-once mode"]
    #[inline(always)]
    pub const fn wave(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Waveform shape The WAVE bit controls the output shape 0: Deactivate Set-once mode 1: Activate the Set-once mode"]
    #[inline(always)]
    pub fn set_wave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Waveform shape polarity The WAVEPOL bit controls the output polarity 0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers 1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    #[inline(always)]
    pub const fn wavpol(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Waveform shape polarity The WAVEPOL bit controls the output polarity 0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers 1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn set_wavpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "counter mode in internal clock source mode (CKSEL=0). If CKSEL=1, this bit has no effect. 0: the counter is incremented following each internal clock pulse 1: the counter is incremented following each valid pulse on the external clock"]
    #[inline(always)]
    pub const fn countmode(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "counter mode in internal clock source mode (CKSEL=0). If CKSEL=1, this bit has no effect. 0: the counter is incremented following each internal clock pulse 1: the counter is incremented following each valid pulse on the external clock"]
    #[inline(always)]
    pub fn set_countmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Cfgr {
    #[inline(always)]
    fn default() -> Cfgr {
        Cfgr(0)
    }
}
impl core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgr")
            .field("cksel", &self.cksel())
            .field("ckpol", &self.ckpol())
            .field("ckflt", &self.ckflt())
            .field("intcksel", &self.intcksel())
            .field("trgflt", &self.trgflt())
            .field("extcksel", &self.extcksel())
            .field("presc", &self.presc())
            .field("trigsel", &self.trigsel())
            .field("trigen", &self.trigen())
            .field("timout", &self.timout())
            .field("wave", &self.wave())
            .field("wavpol", &self.wavpol())
            .field("countmode", &self.countmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cfgr {
            cksel: bool,
            ckpol: u8,
            ckflt: u8,
            intcksel: bool,
            trgflt: u8,
            extcksel: bool,
            presc: u8,
            trigsel: u8,
            trigen: u8,
            timout: bool,
            wave: bool,
            wavpol: bool,
            countmode: bool,
        }
        let proxy = Cfgr {
            cksel: self.cksel(),
            ckpol: self.ckpol(),
            ckflt: self.ckflt(),
            intcksel: self.intcksel(),
            trgflt: self.trgflt(),
            extcksel: self.extcksel(),
            presc: self.presc(),
            trigsel: self.trigsel(),
            trigen: self.trigen(),
            timout: self.timout(),
            wave: self.wave(),
            wavpol: self.wavpol(),
            countmode: self.countmode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM compare register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp(pub u32);
impl Cmp {
    #[doc = "Compare value CMP is the compare value used by the LPTIM."]
    #[inline(always)]
    pub const fn cmp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare value CMP is the compare value used by the LPTIM."]
    #[inline(always)]
    pub fn set_cmp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Cmp {
    #[inline(always)]
    fn default() -> Cmp {
        Cmp(0)
    }
}
impl core::fmt::Debug for Cmp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp").field("cmp", &self.cmp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmp {
            cmp: u32,
        }
        let proxy = Cmp { cmp: self.cmp() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Counter value When the LPTIM is running with an asynchronous clock, reading the CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter value When the LPTIM is running with an asynchronous clock, reading the CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
impl core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cnt {
            cnt: u32,
        }
        let proxy = Cnt { cnt: self.cnt() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "LPTIM enable The ENABLE bit is set and cleared by software. 0:LPTIM is disabled 1:LPTIM is enabled"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM enable The ENABLE bit is set and cleared by software. 0:LPTIM is disabled 1:LPTIM is enabled"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\] different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between ARR and CNT registers. If this bit is set simultaneously with CNTSTRT, then LPTIM will be in continuous counting mode."]
    #[inline(always)]
    pub const fn sngstrt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = 00), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\] different than 00), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between ARR and CNT registers. If this bit is set simultaneously with CNTSTRT, then LPTIM will be in continuous counting mode."]
    #[inline(always)]
    pub fn set_sngstrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\] different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between ARR and CNT registers and the LPTIM counter keeps counting in Continuous mode."]
    #[inline(always)]
    pub const fn cntstrt(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = 00), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\] different than 00), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between ARR and CNT registers and the LPTIM counter keeps counting in Continuous mode."]
    #[inline(always)]
    pub fn set_cntstrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Counter reset This bit is set by software and cleared by hardware. When set to 1 this bit will trigger a synchronous reset of the CNT register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay. COUNTRST must never be set to 1 by software before it is already cleared to 0 by hardware. Software should consequently check that COUNTRST bit is already cleared to 0 before attempting to set it to 1."]
    #[inline(always)]
    pub const fn countrst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Counter reset This bit is set by software and cleared by hardware. When set to 1 this bit will trigger a synchronous reset of the CNT register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay. COUNTRST must never be set to 1 by software before it is already cleared to 0 by hardware. Software should consequently check that COUNTRST bit is already cleared to 0 before attempting to set it to 1."]
    #[inline(always)]
    pub fn set_countrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("enable", &self.enable())
            .field("sngstrt", &self.sngstrt())
            .field("cntstrt", &self.cntstrt())
            .field("countrst", &self.countrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr {
            enable: bool,
            sngstrt: bool,
            cntstrt: bool,
            countrst: bool,
        }
        let proxy = Cr {
            enable: self.enable(),
            sngstrt: self.sngstrt(),
            cntstrt: self.cntstrt(),
            countrst: self.countrst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM interrupt and status clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
    #[inline(always)]
    pub const fn ueclr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
    #[inline(always)]
    pub fn set_ueclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overflow clear flag Writing 1 to this bit clears the OF flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub const fn ofclr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow clear flag Writing 1 to this bit clears the OF flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub fn set_ofclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output compare clear flag Writing 1 to this bit clears the OC flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub const fn occlr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare clear flag Writing 1 to this bit clears the OC flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub fn set_occlr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "External trigger valid edge clear flag Writing 1 to this bit clears the ET flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub const fn etclr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger valid edge clear flag Writing 1 to this bit clears the ET flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub fn set_etclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register."]
    #[inline(always)]
    pub const fn wkupclr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "wakeup status clear flag Writing 1 to this bit clears all wakeup status flags in the LPTIM_ISR register."]
    #[inline(always)]
    pub fn set_wkupclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("ueclr", &self.ueclr())
            .field("ofclr", &self.ofclr())
            .field("occlr", &self.occlr())
            .field("etclr", &self.etclr())
            .field("wkupclr", &self.wkupclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Icr {
            ueclr: bool,
            ofclr: bool,
            occlr: bool,
            etclr: bool,
            wkupclr: bool,
        }
        let proxy = Icr {
            ueclr: self.ueclr(),
            ofclr: self.ofclr(),
            occlr: self.occlr(),
            etclr: self.etclr(),
            wkupclr: self.wkupclr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM interrupt and wakeup enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Update event interrupt enable 0: Update event interrupt disabled 1: Update event interrupt enabled"]
    #[inline(always)]
    pub const fn ueie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update event interrupt enable 0: Update event interrupt disabled 1: Update event interrupt enabled"]
    #[inline(always)]
    pub fn set_ueie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overflow Interrupt Enable 0: Overflow interrupt disabled 1: Overflow interrupt enabled"]
    #[inline(always)]
    pub const fn ofie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Interrupt Enable 0: Overflow interrupt disabled 1: Overflow interrupt enabled"]
    #[inline(always)]
    pub fn set_ofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output compare Interrupt Enable 0: Output compare interrupt disabled 1: Output compare interrupt enabled"]
    #[inline(always)]
    pub const fn ocie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare Interrupt Enable 0: Output compare interrupt disabled 1: Output compare interrupt enabled"]
    #[inline(always)]
    pub fn set_ocie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "External trigger valid edge Interrupt Enable 0: External trigger interrupt disabled 1: External trigger interrupt enabled"]
    #[inline(always)]
    pub const fn etie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger valid edge Interrupt Enable 0: External trigger interrupt disabled 1: External trigger interrupt enabled"]
    #[inline(always)]
    pub fn set_etie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Update event Wakeup enable 0: Update event Wakeup disabled 1: Update event Wakeup enabled"]
    #[inline(always)]
    pub const fn uewe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Update event Wakeup enable 0: Update event Wakeup disabled 1: Update event Wakeup enabled"]
    #[inline(always)]
    pub fn set_uewe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Overflow Wakeup Enable 0: Overflow Wakeup disabled 1: Overflow Wakeup enabled"]
    #[inline(always)]
    pub const fn ofwe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Wakeup Enable 0: Overflow Wakeup disabled 1: Overflow Wakeup enabled"]
    #[inline(always)]
    pub fn set_ofwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Output compare Wakeup Enable 0: Output compare wakeup disabled 1: Output compare wakeup enabled"]
    #[inline(always)]
    pub const fn ocwe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare Wakeup Enable 0: Output compare wakeup disabled 1: Output compare wakeup enabled"]
    #[inline(always)]
    pub fn set_ocwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("ueie", &self.ueie())
            .field("ofie", &self.ofie())
            .field("ocie", &self.ocie())
            .field("etie", &self.etie())
            .field("uewe", &self.uewe())
            .field("ofwe", &self.ofwe())
            .field("ocwe", &self.ocwe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ier {
            ueie: bool,
            ofie: bool,
            ocie: bool,
            etie: bool,
            uewe: bool,
            ofwe: bool,
            ocwe: bool,
        }
        let proxy = Ier {
            ueie: self.ueie(),
            ofie: self.ofie(),
            ocie: self.ocie(),
            etie: self.etie(),
            uewe: self.uewe(),
            ofwe: self.ofwe(),
            ocwe: self.ocwe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM interrupt and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "LPTIM update event occurred UE is set by hardware to inform application that an update event was generated when overflow occurred while repetition counter reached zero. UE flag can be cleared by writing 1 to the UECLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM update event occurred UE is set by hardware to inform application that an update event was generated when overflow occurred while repetition counter reached zero. UE flag can be cleared by writing 1 to the UECLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overflow occurred OF is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. OF flag can be cleared by writing 1 to the OFCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow occurred OF is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. OF flag can be cleared by writing 1 to the OFCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output compare match The OC bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register’s value. OC flag can be cleared by writing 1 to the OCCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub const fn oc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare match The OC bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP register’s value. OC flag can be cleared by writing 1 to the OCCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn set_oc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "External trigger edge event ET is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. ET flag can be cleared by writing 1 to the ETCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub const fn et(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger edge event ET is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. ET flag can be cleared by writing 1 to the ETCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn set_et(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates update event wakeup occurred UEWKUP is set by hardware when an update event was generated (overflow occurred while repetition counter reached zero). To clear UEWKUP, first write 0 to the UEWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub const fn uewkup(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates update event wakeup occurred UEWKUP is set by hardware when an update event was generated (overflow occurred while repetition counter reached zero). To clear UEWKUP, first write 0 to the UEWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn set_uewkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates overflow wakeup occurred OFWKUP is set by hardware when LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. To clear OFWKUP, first write 0 to the OFWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub const fn ofwkup(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates overflow wakeup occurred OFWKUP is set by hardware when LPTIM_CNT register’s value reached the LPTIM_ARR register’s value and count from zero again. To clear OFWKUP, first write 0 to the OFWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn set_ofwkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates output compare wakeup occurred The OCWKUP bit is set by hardware when LPTIM_CNT register value reached the LPTIM_CMP register’s value. To clear OCWKUP, first write 0 to the OCWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub const fn ocwkup(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates output compare wakeup occurred The OCWKUP bit is set by hardware when LPTIM_CNT register value reached the LPTIM_CMP register’s value. To clear OCWKUP, first write 0 to the OCWE bit in the LPTIM_IER register to disable, then write 1 to the WKUPCLR bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn set_ocwkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
impl core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr")
            .field("ue", &self.ue())
            .field("of", &self.of())
            .field("oc", &self.oc())
            .field("et", &self.et())
            .field("uewkup", &self.uewkup())
            .field("ofwkup", &self.ofwkup())
            .field("ocwkup", &self.ocwkup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isr {
            ue: bool,
            of: bool,
            oc: bool,
            et: bool,
            uewkup: bool,
            ofwkup: bool,
            ocwkup: bool,
        }
        let proxy = Isr {
            ue: self.ue(),
            of: self.of(),
            oc: self.oc(),
            et: self.et(),
            uewkup: self.uewkup(),
            ofwkup: self.ofwkup(),
            ocwkup: self.ocwkup(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPTIM repetition register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Repetition register value REP is the repetition value for the LPTIM. Read REP will return left repetition times. It should be noted that for a reliable REP register read access, two consecutive read accesses must be performed and compared. A read access can be considered reliable when the values of the two consecutive read accesses are equal."]
    #[inline(always)]
    pub const fn rep(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Repetition register value REP is the repetition value for the LPTIM. Read REP will return left repetition times. It should be noted that for a reliable REP register read access, two consecutive read accesses must be performed and compared. A read access can be considered reliable when the values of the two consecutive read accesses are equal."]
    #[inline(always)]
    pub fn set_rep(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rcr {
    #[inline(always)]
    fn default() -> Rcr {
        Rcr(0)
    }
}
impl core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr").field("rep", &self.rep()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rcr {
            rep: u8,
        }
        let proxy = Rcr { rep: self.rep() };
        defmt::write!(f, "{}", proxy)
    }
}
