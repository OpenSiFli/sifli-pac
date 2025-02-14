#[doc = "Auto-reload register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc = "Auto-reload value ARR is the value to be loaded in the actual auto-reload register."]
    #[inline(always)]
    pub const fn arr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Auto-reload value ARR is the value to be loaded in the actual auto-reload register."]
    #[inline(always)]
    pub fn set_arr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            arr: u16,
        }
        let proxy = Arr { arr: self.arr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture/Compare enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccer(pub u32);
impl Ccer {
    #[doc = "Capture/Compare 1 output enable. CC1 channel configured as output: 0: Off - OC1 is not active 1: On - OC1 signal is output on the corresponding output pin CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (CCR1) or not. 0: Capture disabled 1: Capture enabled"]
    #[inline(always)]
    pub const fn cce(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output enable. CC1 channel configured as output: 0: Off - OC1 is not active 1: On - OC1 signal is output on the corresponding output pin CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (CCR1) or not. 0: Capture disabled 1: Capture enabled"]
    #[inline(always)]
    pub fn set_cce(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Capture/Compare 1 output Polarity. CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations. 00: noninverted/rising edge Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode). 01: inverted/falling edge Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode). 10: reserved, do not use this configuration. 11: noninverted/both edges Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode."]
    #[inline(always)]
    pub const fn ccp(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 1usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output Polarity. CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations. 00: noninverted/rising edge Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode). 01: inverted/falling edge Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode). 10: reserved, do not use this configuration. 11: noninverted/both edges Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode."]
    #[inline(always)]
    pub fn set_ccp(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 1usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description."]
    #[inline(always)]
    pub const fn ccnp(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define TI1FP1/TI2FP1 polarity. refer to CC1P description."]
    #[inline(always)]
    pub fn set_ccnp(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 3usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ccer {
    #[inline(always)]
    fn default() -> Ccer {
        Ccer(0)
    }
}
impl core::fmt::Debug for Ccer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccer")
            .field(
                "cce",
                &[
                    self.cce(0usize),
                    self.cce(1usize),
                    self.cce(2usize),
                    self.cce(3usize),
                ],
            )
            .field(
                "ccp",
                &[
                    self.ccp(0usize),
                    self.ccp(1usize),
                    self.ccp(2usize),
                    self.ccp(3usize),
                ],
            )
            .field(
                "ccnp",
                &[
                    self.ccnp(0usize),
                    self.ccnp(1usize),
                    self.ccnp(2usize),
                    self.ccnp(3usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccer {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ccer {
            cce: [bool; 4usize],
            ccp: [bool; 4usize],
            ccnp: [bool; 4usize],
        }
        let proxy = Ccer {
            cce: [
                self.cce(0usize),
                self.cce(1usize),
                self.cce(2usize),
                self.cce(3usize),
            ],
            ccp: [
                self.ccp(0usize),
                self.ccp(1usize),
                self.ccp(2usize),
                self.ccp(3usize),
            ],
            ccnp: [
                self.ccnp(0usize),
                self.ccnp(1usize),
                self.ccnp(2usize),
                self.ccnp(3usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TIM capture/compare mode register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr1(pub u32);
impl Ccmr1 {
    #[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 10: CC1 channel is configured as input, IC1 is mapped on TI2 11: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)"]
    #[inline(always)]
    pub const fn ccs(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 10: CC1 channel is configured as input, IC1 is mapped on TI2 11: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)"]
    #[inline(always)]
    pub fn set_ccs(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
    }
    #[doc = "Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0. 00: no prescaler, capture is done each time an edge is detected on the capture input 01: capture is done once every 2 events 10: capture is done once every 4 events 11: capture is done once every 8 events"]
    #[inline(always)]
    pub const fn icpsc(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0. 00: no prescaler, capture is done each time an edge is detected on the capture input 01: capture is done once every 2 events 10: capture is done once every 4 events 11: capture is done once every 8 events"]
    #[inline(always)]
    pub fn set_icpsc(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
    }
    #[doc = "Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, sampling is done at fCLK 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8"]
    #[inline(always)]
    pub const fn icf(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 4usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, sampling is done at fCLK 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8"]
    #[inline(always)]
    pub fn set_icf(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 4usize + n * 8usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
    }
    #[doc = "Output compare 1 clear enable 0: OC1Ref is not affected by the ETRF input 1: OC1Ref is cleared as soon as a High level is detected on ETRF input"]
    #[inline(always)]
    pub const fn occe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 clear enable 0: OC1Ref is not affected by the ETRF input 1: OC1Ref is cleared as soon as a High level is detected on ETRF input"]
    #[inline(always)]
    pub fn set_occe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 1 preload enable 0: Preload register on CCR1 disabled. CCR1 can be written at anytime, the new value is taken in account immediately. 1: Preload register on CCR1 enabled. Read/Write operations access the preload register. CCR1 preload value is loaded in the active register at each update event."]
    #[inline(always)]
    pub const fn ocpe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 preload enable 0: Preload register on CCR1 disabled. CCR1 can be written at anytime, the new value is taken in account immediately. 1: Preload register on CCR1 enabled. Read/Write operations access the preload register. CCR1 preload value is loaded in the active register at each update event."]
    #[inline(always)]
    pub fn set_ocpe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. 0000: Frozen - The comparison between the output compare register CCR1 and the counter CNT has no effect on the outputs.(this mode is used to generate a timing base). 0001: Set channel 1 to active level on match. OC1REF signal is forced high when the counter CNT matches the capture/compare register 1 (CCR1). 0010: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter CNT matches the capture/compare register 1 (CCR1). 0011: Toggle - OC1REF toggles when CNT=CCR1. 0100: Force inactive level - OC1REF is forced low. 0101: Force active level - OC1REF is forced high. 0110: PWM mode 1 - In upcounting, channel 1 is active as long as CNT(CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF='0) as long as CNT>CCR1 else active (OC1REF=1). 0111: PWM mode 2 - In upcounting, channel 1 is inactive as long as CNT(CCR1 else active. In downcounting, channel 1 is active as long as CNT>CCR1 else inactive. 1000: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update. 1001: Retriggerable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. 1010: Reserved, 1011: Reserved, 1100: Combined PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC is the logical OR between OC1REF and OC2REF. 1101: Combined PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC is the logical AND between OC1REF and OC2REF. 1110: Asymmetric PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down. 1111: Asymmetric PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    #[inline(always)]
    pub const fn ocm(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 20usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. 0000: Frozen - The comparison between the output compare register CCR1 and the counter CNT has no effect on the outputs.(this mode is used to generate a timing base). 0001: Set channel 1 to active level on match. OC1REF signal is forced high when the counter CNT matches the capture/compare register 1 (CCR1). 0010: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter CNT matches the capture/compare register 1 (CCR1). 0011: Toggle - OC1REF toggles when CNT=CCR1. 0100: Force inactive level - OC1REF is forced low. 0101: Force active level - OC1REF is forced high. 0110: PWM mode 1 - In upcounting, channel 1 is active as long as CNT(CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF='0) as long as CNT>CCR1 else active (OC1REF=1). 0111: PWM mode 2 - In upcounting, channel 1 is inactive as long as CNT(CCR1 else active. In downcounting, channel 1 is active as long as CNT>CCR1 else inactive. 1000: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update. 1001: Retriggerable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. 1010: Reserved, 1011: Reserved, 1100: Combined PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC is the logical OR between OC1REF and OC2REF. 1101: Combined PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC is the logical AND between OC1REF and OC2REF. 1110: Asymmetric PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down. 1111: Asymmetric PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down."]
    #[inline(always)]
    pub fn set_ocm(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 20usize + n * 8usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
    }
}
impl Default for Ccmr1 {
    #[inline(always)]
    fn default() -> Ccmr1 {
        Ccmr1(0)
    }
}
impl core::fmt::Debug for Ccmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr1")
            .field("ccs", &[self.ccs(0usize), self.ccs(1usize)])
            .field("icpsc", &[self.icpsc(0usize), self.icpsc(1usize)])
            .field("icf", &[self.icf(0usize), self.icf(1usize)])
            .field("occe", &[self.occe(0usize), self.occe(1usize)])
            .field("ocpe", &[self.ocpe(0usize), self.ocpe(1usize)])
            .field("ocm", &[self.ocm(0usize), self.ocm(1usize)])
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ccmr1 {
            ccs: [u8; 2usize],
            icpsc: [u8; 2usize],
            icf: [u8; 2usize],
            occe: [bool; 2usize],
            ocpe: [bool; 2usize],
            ocm: [u8; 2usize],
        }
        let proxy = Ccmr1 {
            ccs: [self.ccs(0usize), self.ccs(1usize)],
            icpsc: [self.icpsc(0usize), self.icpsc(1usize)],
            icf: [self.icf(0usize), self.icf(1usize)],
            occe: [self.occe(0usize), self.occe(1usize)],
            ocpe: [self.ocpe(0usize), self.ocpe(1usize)],
            ocm: [self.ocm(0usize), self.ocm(1usize)],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TIM capture/compare mode register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr2(pub u32);
impl Ccmr2 {
    #[doc = "Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)"]
    #[inline(always)]
    pub const fn ccs(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)"]
    #[inline(always)]
    pub fn set_ccs(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
    }
    #[doc = "Input capture 3 prescaler"]
    #[inline(always)]
    pub const fn icpsc(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Input capture 3 prescaler"]
    #[inline(always)]
    pub fn set_icpsc(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
    }
    #[doc = "Input capture 3 filter"]
    #[inline(always)]
    pub const fn icf(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 4usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 3 filter"]
    #[inline(always)]
    pub fn set_icf(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 4usize + n * 8usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
    }
    #[doc = "Output compare 3 clear enable"]
    #[inline(always)]
    pub const fn occe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 clear enable"]
    #[inline(always)]
    pub fn set_occe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 3 preload enable"]
    #[inline(always)]
    pub const fn ocpe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 preload enable"]
    #[inline(always)]
    pub fn set_ocpe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 3 mode"]
    #[inline(always)]
    pub const fn ocm(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 20usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Output compare 3 mode"]
    #[inline(always)]
    pub fn set_ocm(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 20usize + n * 8usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
    }
}
impl Default for Ccmr2 {
    #[inline(always)]
    fn default() -> Ccmr2 {
        Ccmr2(0)
    }
}
impl core::fmt::Debug for Ccmr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr2")
            .field("ccs", &[self.ccs(0usize), self.ccs(1usize)])
            .field("icpsc", &[self.icpsc(0usize), self.icpsc(1usize)])
            .field("icf", &[self.icf(0usize), self.icf(1usize)])
            .field("occe", &[self.occe(0usize), self.occe(1usize)])
            .field("ocpe", &[self.ocpe(0usize), self.ocpe(1usize)])
            .field("ocm", &[self.ocm(0usize), self.ocm(1usize)])
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ccmr2 {
            ccs: [u8; 2usize],
            icpsc: [u8; 2usize],
            icf: [u8; 2usize],
            occe: [bool; 2usize],
            ocpe: [bool; 2usize],
            ocm: [u8; 2usize],
        }
        let proxy = Ccmr2 {
            ccs: [self.ccs(0usize), self.ccs(1usize)],
            icpsc: [self.icpsc(0usize), self.icpsc(1usize)],
            icf: [self.icf(0usize), self.icf(1usize)],
            occe: [self.occe(0usize), self.occe(1usize)],
            ocpe: [self.ocpe(0usize), self.ocpe(1usize)],
            ocm: [self.ocm(0usize), self.ocm(1usize)],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture/Compare register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value).It is loaded permanently if the preload feature is not selected in the CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter CNT and signaled on OC1 output. If channel CC1is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1). The CCR1 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub const fn ccr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value).It is loaded permanently if the preload feature is not selected in the CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter CNT and signaled on OC1 output. If channel CC1is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1). The CCR1 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn set_ccr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr").field("ccr", &self.ccr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ccr {
            ccr: u16,
        }
        let proxy = Ccr { ccr: self.ccr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "counter value"]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "counter value"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Value depends on IUFREMAP in CR1. If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the ISR register"]
    #[inline(always)]
    pub const fn uifcpy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Value depends on IUFREMAP in CR1. If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the ISR register"]
    #[inline(always)]
    pub fn set_uifcpy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
        f.debug_struct("Cnt")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cnt {
            cnt: u16,
            uifcpy: bool,
        }
        let proxy = Cnt {
            cnt: self.cnt(),
            uifcpy: self.uifcpy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TIM control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Counter enable 0: Counter disabled 1: Counter enabled Note: External clock, gated mode and encoder mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs."]
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter enable 0: Counter disabled 1: Counter enabled Note: External clock, gated mode and encoder mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs."]
    #[inline(always)]
    pub fn set_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Update disable This bit is set and cleared by software to enable/disable UEV event generation. 0: UEV enabled. The Update (UEV) event is generated by one of the following events: Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values. 1: UEV disabled. The Update event is not generated, shadow registers keep their value (ARR, PSC, CCRx). However the counter and the prescaler are reinitialized if the UG bit is set or if a hardware reset is received from the slave mode controller."]
    #[inline(always)]
    pub const fn udis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Update disable This bit is set and cleared by software to enable/disable UEV event generation. 0: UEV enabled. The Update (UEV) event is generated by one of the following events: Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values. 1: UEV disabled. The Update event is not generated, shadow registers keep their value (ARR, PSC, CCRx). However the counter and the prescaler are reinitialized if the UG bit is set or if a hardware reset is received from the slave mode controller."]
    #[inline(always)]
    pub fn set_udis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Update request source This bit is set and cleared by software to select the UEV event sources. 0: Any of the following events generate an update interrupt or DMA request if enabled. These events can be: Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller 1: Only counter overflow/underflow generates an update interrupt or DMA request if enabled."]
    #[inline(always)]
    pub const fn urs(&self) -> super::super::tim_common::vals::URS {
        let val = (self.0 >> 2usize) & 0x01;
        super::super::tim_common::vals::URS::from_bits(val as u8)
    }
    #[doc = "Update request source This bit is set and cleared by software to select the UEV event sources. 0: Any of the following events generate an update interrupt or DMA request if enabled. These events can be: Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller 1: Only counter overflow/underflow generates an update interrupt or DMA request if enabled."]
    #[inline(always)]
    pub fn set_urs(&mut self, val: super::super::tim_common::vals::URS) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "One-pulse mode 0: Counter is not stopped at update event 1: Counter stops counting at the next update event (clearing the bit CEN)"]
    #[inline(always)]
    pub const fn opm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "One-pulse mode 0: Counter is not stopped at update event 1: Counter stops counting at the next update event (clearing the bit CEN)"]
    #[inline(always)]
    pub fn set_opm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Direction 0: Counter used as upcounter 1: Counter used as downcounter"]
    #[inline(always)]
    pub const fn dir(&self) -> super::super::tim_common::vals::DIR {
        let val = (self.0 >> 4usize) & 0x01;
        super::super::tim_common::vals::DIR::from_bits(val as u8)
    }
    #[doc = "Direction 0: Counter used as upcounter 1: Counter used as downcounter"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: super::super::tim_common::vals::DIR) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Center-aligned mode selection 00: Edge-aligned mode. The counter counts up or down depending on the direction bit (DIR). 01: Center-aligned mode 1. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set only when the counter is counting down. 10: Center-aligned mode 2. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set only when the counter is counting up. 11: Center-aligned mode 3. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set both when the counter is counting up or down."]
    #[inline(always)]
    pub const fn cms(&self) -> super::super::tim_common::vals::CMS {
        let val = (self.0 >> 5usize) & 0x03;
        super::super::tim_common::vals::CMS::from_bits(val as u8)
    }
    #[doc = "Center-aligned mode selection 00: Edge-aligned mode. The counter counts up or down depending on the direction bit (DIR). 01: Center-aligned mode 1. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set only when the counter is counting down. 10: Center-aligned mode 2. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set only when the counter is counting up. 11: Center-aligned mode 3. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set both when the counter is counting up or down."]
    #[inline(always)]
    pub fn set_cms(&mut self, val: super::super::tim_common::vals::CMS) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Auto-reload preload enable 0: ARR register is not buffered 1: ARR register is buffered"]
    #[inline(always)]
    pub const fn arpe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-reload preload enable 0: ARR register is not buffered 1: ARR register is buffered"]
    #[inline(always)]
    pub fn set_arpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UIF status bit remapping 0: No remapping. UIF status bit is not copied to CNT register bit 31 1: Remapping enabled. UIF status bit is copied to CNT register bit 31"]
    #[inline(always)]
    pub const fn uifremap(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "UIF status bit remapping 0: No remapping. UIF status bit is not copied to CNT register bit 31 1: Remapping enabled. UIF status bit is copied to CNT register bit 31"]
    #[inline(always)]
    pub fn set_uifremap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("cen", &self.cen())
            .field("udis", &self.udis())
            .field("urs", &self.urs())
            .field("opm", &self.opm())
            .field("dir", &self.dir())
            .field("cms", &self.cms())
            .field("arpe", &self.arpe())
            .field("uifremap", &self.uifremap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr1 {
            cen: bool,
            udis: bool,
            urs: super::super::tim_common::vals::URS,
            opm: bool,
            dir: super::super::tim_common::vals::DIR,
            cms: super::super::tim_common::vals::CMS,
            arpe: bool,
            uifremap: bool,
        }
        let proxy = Cr1 {
            cen: self.cen(),
            udis: self.udis(),
            urs: self.urs(),
            opm: self.opm(),
            dir: self.dir(),
            cms: self.cms(),
            arpe: self.arpe(),
            uifremap: self.uifremap(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TIM control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Capture/compare DMA selection 0: CCx DMA request sent when CCx event occurs 1: CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub const fn ccds(&self) -> super::super::tim_common::vals::CCDS {
        let val = (self.0 >> 3usize) & 0x01;
        super::super::tim_common::vals::CCDS::from_bits(val as u8)
    }
    #[doc = "Capture/compare DMA selection 0: CCx DMA request sent when CCx event occurs 1: CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn set_ccds(&mut self, val: super::super::tim_common::vals::CCDS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: 000: Reset - the UG bit from the EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset. 001: Enable - the Counter enable signal is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic OR between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected. 010: Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer. 011: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO) 100: Compare - OC1REF signal is used as trigger output (TRGO) 101: Compare - OC2REF signal is used as trigger output (TRGO) 110: Compare - OC3REF signal is used as trigger output (TRGO) 111: Compare - OC4REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub const fn mms(&self) -> super::super::tim_common::vals::MMS {
        let val = (self.0 >> 4usize) & 0x07;
        super::super::tim_common::vals::MMS::from_bits(val as u8)
    }
    #[doc = "Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: 000: Reset - the UG bit from the EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset. 001: Enable - the Counter enable signal is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic OR between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected. 010: Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer. 011: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO) 100: Compare - OC1REF signal is used as trigger output (TRGO) 101: Compare - OC2REF signal is used as trigger output (TRGO) 110: Compare - OC3REF signal is used as trigger output (TRGO) 111: Compare - OC4REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn set_mms(&mut self, val: super::super::tim_common::vals::MMS) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "TI1 selection 0: The CH1 pin is connected to TI1 input 1: The CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub const fn ti1s(&self) -> super::super::tim_common::vals::TI1S {
        let val = (self.0 >> 7usize) & 0x01;
        super::super::tim_common::vals::TI1S::from_bits(val as u8)
    }
    #[doc = "TI1 selection 0: The CH1 pin is connected to TI1 input 1: The CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub fn set_ti1s(&mut self, val: super::super::tim_common::vals::TI1S) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
impl core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr2")
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr2 {
            ccds: super::super::tim_common::vals::CCDS,
            mms: super::super::tim_common::vals::MMS,
            ti1s: super::super::tim_common::vals::TI1S,
        }
        let proxy = Cr2 {
            ccds: self.ccds(),
            mms: self.mms(),
            ti1s: self.ti1s(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TIM DMA/Interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dier(pub u32);
impl Dier {
    #[doc = "Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled"]
    #[inline(always)]
    pub const fn uie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled"]
    #[inline(always)]
    pub fn set_uie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled"]
    #[inline(always)]
    pub const fn ccie(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled"]
    #[inline(always)]
    pub fn set_ccie(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled"]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled"]
    #[inline(always)]
    pub fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled"]
    #[inline(always)]
    pub const fn ude(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled"]
    #[inline(always)]
    pub fn set_ude(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled. 1: CC1 DMA request enabled."]
    #[inline(always)]
    pub const fn ccde(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 9usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled. 1: CC1 DMA request enabled."]
    #[inline(always)]
    pub fn set_ccde(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 9usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled."]
    #[inline(always)]
    pub const fn tde(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled."]
    #[inline(always)]
    pub fn set_tde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Dier {
    #[inline(always)]
    fn default() -> Dier {
        Dier(0)
    }
}
impl core::fmt::Debug for Dier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dier")
            .field("uie", &self.uie())
            .field(
                "ccie",
                &[
                    self.ccie(0usize),
                    self.ccie(1usize),
                    self.ccie(2usize),
                    self.ccie(3usize),
                ],
            )
            .field("tie", &self.tie())
            .field("ude", &self.ude())
            .field(
                "ccde",
                &[
                    self.ccde(0usize),
                    self.ccde(1usize),
                    self.ccde(2usize),
                    self.ccde(3usize),
                ],
            )
            .field("tde", &self.tde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dier {
            uie: bool,
            ccie: [bool; 4usize],
            tie: bool,
            ude: bool,
            ccde: [bool; 4usize],
            tde: bool,
        }
        let proxy = Dier {
            uie: self.uie(),
            ccie: [
                self.ccie(0usize),
                self.ccie(1usize),
                self.ccie(2usize),
                self.ccie(3usize),
            ],
            tie: self.tie(),
            ude: self.ude(),
            ccde: [
                self.ccde(0usize),
                self.ccde(1usize),
                self.ccde(2usize),
                self.ccde(3usize),
            ],
            tde: self.tde(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Event generation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Egr(pub u32);
impl Egr {
    #[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: Re-initialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (ARR) if DIR=1 (downcounting)."]
    #[inline(always)]
    pub const fn ug(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: Re-initialize the counter and generates an update of the registers. Note that the prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (ARR) if DIR=1 (downcounting)."]
    #[inline(always)]
    pub fn set_ug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    pub const fn ccg(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    pub fn set_ccg(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub const fn tg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn set_tg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Egr {
    #[inline(always)]
    fn default() -> Egr {
        Egr(0)
    }
}
impl core::fmt::Debug for Egr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Egr")
            .field("ug", &self.ug())
            .field(
                "ccg",
                &[
                    self.ccg(0usize),
                    self.ccg(1usize),
                    self.ccg(2usize),
                    self.ccg(3usize),
                ],
            )
            .field("tg", &self.tg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Egr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Egr {
            ug: bool,
            ccg: [bool; 4usize],
            tg: bool,
        }
        let proxy = Egr {
            ug: self.ug(),
            ccg: [
                self.ccg(0usize),
                self.ccg(1usize),
                self.ccg(2usize),
                self.ccg(3usize),
            ],
            tg: self.tg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Prescaler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psc(pub u32);
impl Psc {
    #[doc = "Prescaler value The counter clock frequency is equal to fCLK / (PSC\\[15:0\\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of EGR register or through trigger controller when configured in 'reset mode')."]
    #[inline(always)]
    pub const fn psc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Prescaler value The counter clock frequency is equal to fCLK / (PSC\\[15:0\\] + 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of EGR register or through trigger controller when configured in 'reset mode')."]
    #[inline(always)]
    pub fn set_psc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Psc {
    #[inline(always)]
    fn default() -> Psc {
        Psc(0)
    }
}
impl core::fmt::Debug for Psc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psc").field("psc", &self.psc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psc {
            psc: u16,
        }
        let proxy = Psc { psc: self.psc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Repetition counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Repetition counter value These bits allow the user to set-up the update rate of the compare registers (i.e. periodic transfers from preload to active registers) when preload registers are enable, as well as the update interrupt generation rate, if this interrupt is enable. Each time the REP_CNT related downcounter reaches zero, an update event is generated and it restarts counting from REP value. As REP_CNT is reloaded with REP value only at the repetition update event, any write to the RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode."]
    #[inline(always)]
    pub const fn rep(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Repetition counter value These bits allow the user to set-up the update rate of the compare registers (i.e. periodic transfers from preload to active registers) when preload registers are enable, as well as the update interrupt generation rate, if this interrupt is enable. Each time the REP_CNT related downcounter reaches zero, an update event is generated and it restarts counting from REP value. As REP_CNT is reloaded with REP value only at the repetition update event, any write to the RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode."]
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
#[doc = "TIM slave mode control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcr(pub u32);
impl Smcr {
    #[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 000: Internal Trigger 0 (ITR0) 001: Internal Trigger 1 (ITR1) 010: Internal Trigger 2 (ITR2) 011: Internal Trigger 3 (ITR3) 100: TI1 Edge Detector (TI1F_ED) 101: Filtered Timer Input 1 (TI1FP1) 110: Filtered Timer Input 2 (TI2FP2) 111: External Trigger input (ETRF)"]
    #[inline(always)]
    pub const fn ts(&self) -> super::super::tim_common::vals::TS {
        let val = (self.0 >> 4usize) & 0x07;
        super::super::tim_common::vals::TS::from_bits(val as u8)
    }
    #[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 000: Internal Trigger 0 (ITR0) 001: Internal Trigger 1 (ITR1) 010: Internal Trigger 2 (ITR2) 011: Internal Trigger 3 (ITR3) 100: TI1 Edge Detector (TI1F_ED) 101: Filtered Timer Input 1 (TI1FP1) 110: Filtered Timer Input 2 (TI2FP2) 111: External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn set_ts(&mut self, val: super::super::tim_common::vals::TS) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master/Slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub const fn msm(&self) -> super::super::tim_common::vals::MSM {
        let val = (self.0 >> 7usize) & 0x01;
        super::super::tim_common::vals::MSM::from_bits(val as u8)
    }
    #[doc = "Master/Slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn set_msm(&mut self, val: super::super::tim_common::vals::MSM) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8"]
    #[inline(always)]
    pub const fn etf(&self) -> super::super::tim_common::vals::ETF {
        let val = (self.0 >> 8usize) & 0x0f;
        super::super::tim_common::vals::ETF::from_bits(val as u8)
    }
    #[doc = "External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8"]
    #[inline(always)]
    pub fn set_etf(&mut self, val: super::super::tim_common::vals::ETF) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8"]
    #[inline(always)]
    pub const fn etps(&self) -> super::super::tim_common::vals::ETPS {
        let val = (self.0 >> 12usize) & 0x03;
        super::super::tim_common::vals::ETPS::from_bits(val as u8)
    }
    #[doc = "External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn set_etps(&mut self, val: super::super::tim_common::vals::ETPS) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub const fn ece(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn set_ece(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "External trigger polarity 0: ETR is non-inverted, active at high level or rising edge 1: ETR is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub const fn etp(&self) -> super::super::tim_common::vals::ETP {
        let val = (self.0 >> 15usize) & 0x01;
        super::super::tim_common::vals::ETP::from_bits(val as u8)
    }
    #[doc = "External trigger polarity 0: ETR is non-inverted, active at high level or rising edge 1: ETR is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn set_etp(&mut self, val: super::super::tim_common::vals::ETP) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input. 0000: Slave mode disabled. 0001: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0010: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter."]
    #[inline(always)]
    pub const fn sms(&self) -> super::super::tim_common::vals::SMS {
        let val = (self.0 >> 16usize) & 0x0f;
        super::super::tim_common::vals::SMS::from_bits(val as u8)
    }
    #[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input. 0000: Slave mode disabled. 0001: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0010: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter."]
    #[inline(always)]
    pub fn set_sms(&mut self, val: super::super::tim_common::vals::SMS) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Smcr {
    #[inline(always)]
    fn default() -> Smcr {
        Smcr(0)
    }
}
impl core::fmt::Debug for Smcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcr")
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("etf", &self.etf())
            .field("etps", &self.etps())
            .field("ece", &self.ece())
            .field("etp", &self.etp())
            .field("sms", &self.sms())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smcr {
            ts: super::super::tim_common::vals::TS,
            msm: super::super::tim_common::vals::MSM,
            etf: super::super::tim_common::vals::ETF,
            etps: super::super::tim_common::vals::ETPS,
            ece: bool,
            etp: super::super::tim_common::vals::ETP,
            sms: super::super::tim_common::vals::SMS,
        }
        let proxy = Smcr {
            ts: self.ts(),
            msm: self.msm(),
            etf: self.etf(),
            etps: self.etps(),
            ece: self.ece(),
            etp: self.etp(),
            sms: self.sms(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TIM status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow or underflow and if UDIS=0 in the CR1 register. When CNT is reinitialized by software using the UG bit in EGR register, if URS=0 and UDIS=0 in the CR1 register. When CNT is reinitialized by a trigger event, if URS=0 and UDIS=0 in the CR1 register."]
    #[inline(always)]
    pub const fn uif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred 1: Update interrupt pending. This bit is set by hardware when the registers are updated: At overflow or underflow and if UDIS=0 in the CR1 register. When CNT is reinitialized by software using the UG bit in EGR register, if URS=0 and UDIS=0 in the CR1 register. When CNT is reinitialized by a trigger event, if URS=0 and UDIS=0 in the CR1 register."]
    #[inline(always)]
    pub fn set_uif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value. It is cleared by software. 0: No match. 1: The content of the counter CNT has matched the content of the CCR1 register. If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the CCR1 register. 0: No input capture occurred. 1: The counter value has been captured in CCR1 register (An edge has been detected on IC1 which matches the selected polarity)."]
    #[inline(always)]
    pub const fn ccif(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value. It is cleared by software. 0: No match. 1: The content of the counter CNT has matched the content of the CCR1 register. If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the CCR1 register. 0: No input capture occurred. 1: The counter value has been captured in CCR1 register (An edge has been detected on IC1 which matches the selected polarity)."]
    #[inline(always)]
    pub fn set_ccif(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode). It is set when the counter starts or stops when gated mode is selected. It is cleared by software. 0: No trigger event occurred. 1: Trigger interrupt pending."]
    #[inline(always)]
    pub const fn tif(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger interrupt flag This flag is set by hardware on trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode). It is set when the counter starts or stops when gated mode is selected. It is cleared by software. 0: No trigger event occurred. 1: Trigger interrupt pending."]
    #[inline(always)]
    pub fn set_tif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected. 1: The counter value has been captured in CCR1 register while CC1IF flag was already set"]
    #[inline(always)]
    pub const fn ccof(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 9usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected. 1: The counter value has been captured in CCR1 register while CC1IF flag was already set"]
    #[inline(always)]
    pub fn set_ccof(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 9usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("uif", &self.uif())
            .field(
                "ccif",
                &[
                    self.ccif(0usize),
                    self.ccif(1usize),
                    self.ccif(2usize),
                    self.ccif(3usize),
                ],
            )
            .field("tif", &self.tif())
            .field(
                "ccof",
                &[
                    self.ccof(0usize),
                    self.ccof(1usize),
                    self.ccof(2usize),
                    self.ccof(3usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sr {
            uif: bool,
            ccif: [bool; 4usize],
            tif: bool,
            ccof: [bool; 4usize],
        }
        let proxy = Sr {
            uif: self.uif(),
            ccif: [
                self.ccif(0usize),
                self.ccif(1usize),
                self.ccif(2usize),
                self.ccif(3usize),
            ],
            tif: self.tif(),
            ccof: [
                self.ccof(0usize),
                self.ccof(1usize),
                self.ccof(2usize),
                self.ccof(3usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
