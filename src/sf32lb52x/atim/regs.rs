#[doc = "Alternate function option register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Af1(pub u32);
impl Af1 {
    #[doc = "BRK BKIN input enable This bit enables the BKIN input. BKIN input is 'Ored' with the other BRK sources. 0: BKIN input disabled 1: BKIN input enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bkine(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BRK BKIN input enable This bit enables the BKIN input. BKIN input is 'Ored' with the other BRK sources. 0: BKIN input disabled 1: BKIN input enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bkine(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BRK LPCOMP output1 enable This bit enables the LPCOMP output1 (if LPCOMP integrated) for the timer's BRK input. LPCOMP output1 is 'ORed' with the other BRK sources. 0: LPCOMP output1 disabled 1: LPCOMP output1 enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bkcmp1e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BRK LPCOMP output1 enable This bit enables the LPCOMP output1 (if LPCOMP integrated) for the timer's BRK input. LPCOMP output1 is 'ORed' with the other BRK sources. 0: LPCOMP output1 disabled 1: LPCOMP output1 enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bkcmp1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BRK LPCOMP output2 enable This bit enables the LPCOMP output2 (if LPCOMP integrated) for the timer's BRK input. LPCOMP output2 is 'ORed' with the other BRK sources. 0: LPCOMP output2 disabled 1: LPCOMP output2 enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bkcmp2e(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BRK LPCOMP output2 enable This bit enables the LPCOMP output2 (if LPCOMP integrated) for the timer's BRK input. LPCOMP output2 is 'ORed' with the other BRK sources. 0: LPCOMP output2 disabled 1: LPCOMP output2 enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bkcmp2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BRK BKIN input polarity This bit selects the BKIN input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active high 1: BKIN input is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bkinp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "BRK BKIN input polarity This bit selects the BKIN input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active high 1: BKIN input is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bkinp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "BRK LPCOMP output1 polarity This bit selects the LPCOMP output1 sensitivity (if LPCOMP integrated). It must be programmed together with the BKP polarity bit. 0: LPCOMP output1 is active high 1: LPCOMP output1 is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bkcmp1p(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "BRK LPCOMP output1 polarity This bit selects the LPCOMP output1 sensitivity (if LPCOMP integrated). It must be programmed together with the BKP polarity bit. 0: LPCOMP output1 is active high 1: LPCOMP output1 is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bkcmp1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "BRK LPCOMP output2 polarity This bit selects the LPCOMP output2 sensitivity (if LPCOMP integrated). It must be programmed together with the BKP polarity bit. 0: LPCOMP output2 is active high 1: LPCOMP output2 is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bkcmp2p(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "BRK LPCOMP output2 polarity This bit selects the LPCOMP output2 sensitivity (if LPCOMP integrated). It must be programmed together with the BKP polarity bit. 0: LPCOMP output2 is active high 1: LPCOMP output2 is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bkcmp2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ETR source selection 00: ETR input is connected to I/O 01: LPCOMP output1 (if LPCOMP integrated) 10: LPCOMP output2 (if LPCOMP integrated) 11: ETR input is connected to I/O This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn etrsel(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "ETR source selection 00: ETR input is connected to I/O 01: LPCOMP output1 (if LPCOMP integrated) 10: LPCOMP output2 (if LPCOMP integrated) 11: ETR input is connected to I/O This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_etrsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Lock configuration These bits offer a write protection against software errors. 00: LOCK OFF - No bit is write protected. 01: LOCK Level 1 = OISx and OISxN bits in CR2 register, BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR, DTPSC and DTG bits in BDTR register, AF1 register and AF2 register can no longer be written. 10: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written. 11: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written. The LOCK bits can be written to non-zero only once after reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Lock configuration These bits offer a write protection against software errors. 00: LOCK OFF - No bit is write protected. 01: LOCK Level 1 = OISx and OISxN bits in CR2 register, BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR, DTPSC and DTG bits in BDTR register, AF1 register and AF2 register can no longer be written. 10: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written. 11: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written. The LOCK bits can be written to non-zero only once after reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Af1 {
    #[inline(always)]
    fn default() -> Af1 {
        Af1(0)
    }
}
impl core::fmt::Debug for Af1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Af1")
            .field("bkine", &self.bkine())
            .field("bkcmp1e", &self.bkcmp1e())
            .field("bkcmp2e", &self.bkcmp2e())
            .field("bkinp", &self.bkinp())
            .field("bkcmp1p", &self.bkcmp1p())
            .field("bkcmp2p", &self.bkcmp2p())
            .field("etrsel", &self.etrsel())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Af1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Af1 {{ bkine: {=bool:?}, bkcmp1e: {=bool:?}, bkcmp2e: {=bool:?}, bkinp: {=bool:?}, bkcmp1p: {=bool:?}, bkcmp2p: {=bool:?}, etrsel: {=u8:?}, lock: {=u8:?} }}" , self . bkine () , self . bkcmp1e () , self . bkcmp2e () , self . bkinp () , self . bkcmp1p () , self . bkcmp2p () , self . etrsel () , self . lock ())
    }
}
#[doc = "Alternate function option register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Af2(pub u32);
impl Af2 {
    #[doc = "BRK2 BKIN input enable This bit enables the BKIN2 input. BKIN2 input is 'Ored' with the other BRK2 sources. 0: BKIN2 input disabled 1: BKIN2 input enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2ine(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 BKIN input enable This bit enables the BKIN2 input. BKIN2 input is 'Ored' with the other BRK2 sources. 0: BKIN2 input disabled 1: BKIN2 input enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2ine(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BRK2 LPCOMP output1 enable This bit enables the LPCOMP output1 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output1 is 'ORed' with the other BRK2 sources. 0: LPCOMP output1 disabled 1: LPCOMP output1 enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2cmp1e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 LPCOMP output1 enable This bit enables the LPCOMP output1 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output1 is 'ORed' with the other BRK2 sources. 0: LPCOMP output1 disabled 1: LPCOMP output1 enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2cmp1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BRK2 LPCOMP output2 enable This bit enables the LPCOMP output2 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output2 is 'ORed' with the other BRK2 sources. 0: LPCOMP output2 disabled 1: LPCOMP output2 enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2cmp2e(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 LPCOMP output2 enable This bit enables the LPCOMP output2 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output2 is 'ORed' with the other BRK2 sources. 0: LPCOMP output2 disabled 1: LPCOMP output2 enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2cmp2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BRK2 BKIN2 input polarity This bit selects the BKIN2 input sensitivity. It must be programmed together with the BK2P polarity bit. 0: BKIN2 input is active low 1: BKIN2 input is active high This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2inp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 BKIN2 input polarity This bit selects the BKIN2 input sensitivity. It must be programmed together with the BK2P polarity bit. 0: BKIN2 input is active low 1: BKIN2 input is active high This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2inp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "BRK2 LPCOMP output1 polarity This bit selects the LPCOMP output1 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output1 is active high 1: LPCOMP output1 is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2cmp1p(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 LPCOMP output1 polarity This bit selects the LPCOMP output1 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output1 is active high 1: LPCOMP output1 is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2cmp1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "BRK2 LPCOMP output2 polarity This bit selects the LPCOMP output2 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output2 is active high 1: LPCOMP output2 is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2cmp2p(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 LPCOMP output2 polarity This bit selects the LPCOMP output2 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output2 is active high 1: LPCOMP output2 is active low This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2cmp2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Af2 {
    #[inline(always)]
    fn default() -> Af2 {
        Af2(0)
    }
}
impl core::fmt::Debug for Af2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Af2")
            .field("bk2ine", &self.bk2ine())
            .field("bk2cmp1e", &self.bk2cmp1e())
            .field("bk2cmp2e", &self.bk2cmp2e())
            .field("bk2inp", &self.bk2inp())
            .field("bk2cmp1p", &self.bk2cmp1p())
            .field("bk2cmp2p", &self.bk2cmp2p())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Af2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Af2 {{ bk2ine: {=bool:?}, bk2cmp1e: {=bool:?}, bk2cmp2e: {=bool:?}, bk2inp: {=bool:?}, bk2cmp1p: {=bool:?}, bk2cmp2p: {=bool:?} }}" , self . bk2ine () , self . bk2cmp1e () , self . bk2cmp2e () , self . bk2inp () , self . bk2cmp1p () , self . bk2cmp2p ())
    }
}
#[doc = "Auto-reload register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc = "Auto-reload value ARR is the value to be loaded in the actual auto-reload register."]
    #[must_use]
    #[inline(always)]
    pub const fn arr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Auto-reload value ARR is the value to be loaded in the actual auto-reload register."]
    #[inline(always)]
    pub const fn set_arr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Arr {{ arr: {=u32:?} }}", self.arr())
    }
}
#[doc = "TIM break and dead-time register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtr(pub u32);
impl Bdtr {
    #[doc = "Dead-time generator setup This bit-field, together with DTPSC, defines the duration of the dead-time inserted between the complementary outputs. If DTG=0, dead-time is disabled. Example if tCLK=8.33ns (120MHz), dead-time possible values are: 16.67ns to 8533.33 ns by 8.33 ns steps if DTPSC=0, 266.67ns to 136.53 us by 133.33 ns steps if DTPSC=1 This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn dtg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Dead-time generator setup This bit-field, together with DTPSC, defines the duration of the dead-time inserted between the complementary outputs. If DTG=0, dead-time is disabled. Example if tCLK=8.33ns (120MHz), dead-time possible values are: 16.67ns to 8533.33 ns by 8.33 ns steps if DTPSC=0, 266.67ns to 136.53 us by 133.33 ns steps if DTPSC=1 This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_dtg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Dead-time prescaler This bit-field enables dead-time prescaler. 0: dead-time is tCLK*(DTG+1) if DTG is not zero 1: dead-time is tCLK*(DTG+1)*16 if DTG is not zero This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn dtpsc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Dead-time prescaler This bit-field enables dead-time prescaler. 0: dead-time is tCLK*(DTG+1) if DTG is not zero 1: dead-time is tCLK*(DTG+1)*16 if DTG is not zero This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_dtpsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Break enable This bit enables the complete break protection. 0: Break function disabled 1: Break function enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bke(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Break enable This bit enables the complete break protection. 0: Break function disabled 1: Break function enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Break polarity 0: Break input BRK is active low 1: Break input BRK is active high This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bkp(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Break polarity 0: Break input BRK is active low 1: Break input BRK is active high This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bkp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Automatic output enable 0: MOE can be set only by software 1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active) This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn aoe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic output enable 0: MOE can be set only by software 1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active) This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_aoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: In response to a break 2 event. OC and OCN outputs are disabled In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in CCER register)."]
    #[must_use]
    #[inline(always)]
    pub const fn moe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (BRK or BRK2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: In response to a break 2 event. OC and OCN outputs are disabled In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in CCER register)."]
    #[inline(always)]
    pub const fn set_moe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bkf(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Break filter This bit-field defines the frequency used to sample BRK input and the length of the digital filter applied to BRK. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bkf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK2 acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2f(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Break 2 filter This bit-field defines the frequency used to sample BRK2 input and the length of the digital filter applied to BRK2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, BRK2 acts asynchronously 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8 This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Break 2 enable This bit enables the complete break 2 protection. 0: Break2 function disabled 1: Break2 function enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2e(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Break 2 enable This bit enables the complete break 2 protection. 0: Break2 function disabled 1: Break2 function enabled This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "BK2P: Break 2 polarity 0: Break input BRK2 is active low 1: Break input BRK2 is active high This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn bk2p(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "BK2P: Break 2 polarity 0: Break input BRK2 is active low 1: Break input BRK2 is active high This bit cannot be modified as long as LOCK level 1 has been programmed."]
    #[inline(always)]
    pub const fn set_bk2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared."]
    #[must_use]
    #[inline(always)]
    pub const fn bkdsrm(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared."]
    #[inline(always)]
    pub const fn set_bkdsrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Break2 Disarm"]
    #[must_use]
    #[inline(always)]
    pub const fn bk2dsrm(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Break2 Disarm"]
    #[inline(always)]
    pub const fn set_bk2dsrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in BDTR register)."]
    #[must_use]
    #[inline(always)]
    pub const fn bkbid(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in BDTR register)."]
    #[inline(always)]
    pub const fn set_bkbid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Break2 bidirectional"]
    #[must_use]
    #[inline(always)]
    pub const fn bk2bid(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Break2 bidirectional"]
    #[inline(always)]
    pub const fn set_bk2bid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, imposes a Hi-Z state). 1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output. This bit can not be modified as soon as the LOCK level 2 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn ossi(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Off-state selection for Idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, imposes a Hi-Z state). 1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output. This bit can not be modified as soon as the LOCK level 2 has been programmed."]
    #[inline(always)]
    pub const fn set_ossi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, forces a Hi-Z state). 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). This bit can not be modified as soon as the LOCK level 2 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn ossr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control, forces a Hi-Z state). 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). This bit can not be modified as soon as the LOCK level 2 has been programmed."]
    #[inline(always)]
    pub const fn set_ossr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Bdtr {
    #[inline(always)]
    fn default() -> Bdtr {
        Bdtr(0)
    }
}
impl core::fmt::Debug for Bdtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdtr")
            .field("dtg", &self.dtg())
            .field("dtpsc", &self.dtpsc())
            .field("bke", &self.bke())
            .field("bkp", &self.bkp())
            .field("aoe", &self.aoe())
            .field("moe", &self.moe())
            .field("bkf", &self.bkf())
            .field("bk2f", &self.bk2f())
            .field("bk2e", &self.bk2e())
            .field("bk2p", &self.bk2p())
            .field("bkdsrm", &self.bkdsrm())
            .field("bk2dsrm", &self.bk2dsrm())
            .field("bkbid", &self.bkbid())
            .field("bk2bid", &self.bk2bid())
            .field("ossi", &self.ossi())
            .field("ossr", &self.ossr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdtr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bdtr {{ dtg: {=u16:?}, dtpsc: {=bool:?}, bke: {=bool:?}, bkp: {=bool:?}, aoe: {=bool:?}, moe: {=bool:?}, bkf: {=u8:?}, bk2f: {=u8:?}, bk2e: {=bool:?}, bk2p: {=bool:?}, bkdsrm: {=bool:?}, bk2dsrm: {=bool:?}, bkbid: {=bool:?}, bk2bid: {=bool:?}, ossi: {=bool:?}, ossr: {=bool:?} }}" , self . dtg () , self . dtpsc () , self . bke () , self . bkp () , self . aoe () , self . moe () , self . bkf () , self . bk2f () , self . bk2e () , self . bk2p () , self . bkdsrm () , self . bk2dsrm () , self . bkbid () , self . bk2bid () , self . ossi () , self . ossr ())
    }
}
#[doc = "Capture/Compare enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccer(pub u32);
impl Ccer {
    #[doc = "Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (CCR1) or not. 0: Capture disabled. 1: Capture enabled. On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn cce(&self, n: usize) -> bool {
        assert!(n < 6usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output enable CC1 channel configured as output: 0: Off - OC1 is not active. OC1 level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. 1: On - OC1 signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits. CC1 channel configured as input: This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (CCR1) or not. 0: Capture disabled. 1: Capture enabled. On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub const fn set_cce(&mut self, n: usize, val: bool) {
        assert!(n < 6usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Capture/Compare 1 output Polarity. CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations. 00: noninverted/rising edge. Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode). 01: inverted/falling edge. Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode). 10: reserved, do not use this configuration. 11: noninverted/both edges. Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode. On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated. This bit as well as other CCxP is not writable as soon as LOCK level 2 or 3 has been programmed."]
    #[must_use]
    #[inline(always)]
    pub const fn ccp(&self, n: usize) -> bool {
        assert!(n < 6usize);
        let offs = 1usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output Polarity. CC1 channel configured as output: 0: OC1 active high 1: OC1 active low CC1 channel configured as input: CC1NP/CC1P bits select TI1FP1 and TI2FP1 polarity for trigger or capture operations. 00: noninverted/rising edge. Circuit is sensitive to TIxFP1 rising edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode, encoder mode). 01: inverted/falling edge. Circuit is sensitive to TIxFP1 falling edge (capture, trigger in reset, external clock or trigger mode), TIxFP1 is inverted (trigger in gated mode, encoder mode). 10: reserved, do not use this configuration. 11: noninverted/both edges. Circuit is sensitive to both TIxFP1 rising and falling edges (capture, trigger in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger in gated mode). This configuration must not be used for encoder mode. On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated. This bit as well as other CCxP is not writable as soon as LOCK level 2 or 3 has been programmed."]
    #[inline(always)]
    pub const fn set_ccp(&mut self, n: usize, val: bool) {
        assert!(n < 6usize);
        let offs = 1usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Capture/Compare 1 complementary output enable 0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits. 1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits. On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn ccne(&self, n: usize) -> bool {
        assert!(n < 3usize);
        let offs = 2usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 complementary output enable 0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits. 1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits. On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub const fn set_ccne(&mut self, n: usize, val: bool) {
        assert!(n < 3usize);
        let offs = 2usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Capture/Compare 1 complementary output polarity CC1 channel configured as output: 0: OC1N active high. 1: OC1N active low. CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated. This bit as well as other CCxNP is not writable as soon as LOCK level 2 or 3 has been programmed and CC1S=00 (channel configured as output)."]
    #[must_use]
    #[inline(always)]
    pub const fn ccnp(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 complementary output polarity CC1 channel configured as output: 0: OC1N active high. 1: OC1N active low. CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated. This bit as well as other CCxNP is not writable as soon as LOCK level 2 or 3 has been programmed and CC1S=00 (channel configured as output)."]
    #[inline(always)]
    pub const fn set_ccnp(&mut self, n: usize, val: bool) {
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
            .field("cce[0]", &self.cce(0usize))
            .field("cce[1]", &self.cce(1usize))
            .field("cce[2]", &self.cce(2usize))
            .field("cce[3]", &self.cce(3usize))
            .field("cce[4]", &self.cce(4usize))
            .field("cce[5]", &self.cce(5usize))
            .field("ccp[0]", &self.ccp(0usize))
            .field("ccp[1]", &self.ccp(1usize))
            .field("ccp[2]", &self.ccp(2usize))
            .field("ccp[3]", &self.ccp(3usize))
            .field("ccp[4]", &self.ccp(4usize))
            .field("ccp[5]", &self.ccp(5usize))
            .field("ccne[0]", &self.ccne(0usize))
            .field("ccne[1]", &self.ccne(1usize))
            .field("ccne[2]", &self.ccne(2usize))
            .field("ccnp[0]", &self.ccnp(0usize))
            .field("ccnp[1]", &self.ccnp(1usize))
            .field("ccnp[2]", &self.ccnp(2usize))
            .field("ccnp[3]", &self.ccnp(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccer {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccer {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, cce[2]: {=bool:?}, cce[3]: {=bool:?}, cce[4]: {=bool:?}, cce[5]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccp[2]: {=bool:?}, ccp[3]: {=bool:?}, ccp[4]: {=bool:?}, ccp[5]: {=bool:?}, ccne[0]: {=bool:?}, ccne[1]: {=bool:?}, ccne[2]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?}, ccnp[2]: {=bool:?}, ccnp[3]: {=bool:?} }}" , self . cce (0usize) , self . cce (1usize) , self . cce (2usize) , self . cce (3usize) , self . cce (4usize) , self . cce (5usize) , self . ccp (0usize) , self . ccp (1usize) , self . ccp (2usize) , self . ccp (3usize) , self . ccp (4usize) , self . ccp (5usize) , self . ccne (0usize) , self . ccne (1usize) , self . ccne (2usize) , self . ccnp (0usize) , self . ccnp (1usize) , self . ccnp (2usize) , self . ccnp (3usize))
    }
}
#[doc = "TIM capture/compare mode register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr1(pub u32);
impl Ccmr1 {
    #[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 10: CC1 channel is configured as input, IC1 is mapped on TI2 11: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)"]
    #[must_use]
    #[inline(always)]
    pub const fn ccs(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC1 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 10: CC1 channel is configured as input, IC1 is mapped on TI2 11: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)"]
    #[inline(always)]
    pub const fn set_ccs(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
    }
    #[doc = "Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0 (CCER register). 00: no prescaler, capture is done each time an edge is detected on the capture input 01: capture is done once every 2 events 10: capture is done once every 4 events 11: capture is done once every 8 events"]
    #[must_use]
    #[inline(always)]
    pub const fn icpsc(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0 (CCER register). 00: no prescaler, capture is done each time an edge is detected on the capture input 01: capture is done once every 2 events 10: capture is done once every 4 events 11: capture is done once every 8 events"]
    #[inline(always)]
    pub const fn set_icpsc(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
    }
    #[doc = "Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, sampling is done at fCLK 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8"]
    #[must_use]
    #[inline(always)]
    pub const fn icf(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 4usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, sampling is done at fCLK 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8"]
    #[inline(always)]
    pub const fn set_icf(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 4usize + n * 8usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
    }
    #[doc = "Output compare 1 clear enable 0: OC1Ref is not affected by the ETRF input 1: OC1Ref is cleared as soon as a High level is detected on ETRF input"]
    #[must_use]
    #[inline(always)]
    pub const fn occe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 clear enable 0: OC1Ref is not affected by the ETRF input 1: OC1Ref is cleared as soon as a High level is detected on ETRF input"]
    #[inline(always)]
    pub const fn set_occe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 1 preload enable 0: Preload register on CCR1 disabled. CCR1 can be written at anytime, the new value is taken in account immediately. 1: Preload register on CCR1 enabled. Read/Write operations access the preload register. CCR1 preload value is loaded in the active register at each update event. These bits can not be modified as long as LOCK level 3 has been programmed and CC1S='00' (the channel is configured in output)."]
    #[must_use]
    #[inline(always)]
    pub const fn ocpe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 preload enable 0: Preload register on CCR1 disabled. CCR1 can be written at anytime, the new value is taken in account immediately. 1: Preload register on CCR1 enabled. Read/Write operations access the preload register. CCR1 preload value is loaded in the active register at each update event. These bits can not be modified as long as LOCK level 3 has been programmed and CC1S='00' (the channel is configured in output)."]
    #[inline(always)]
    pub const fn set_ocpe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. 0000: Frozen - The comparison between the output compare register CCR1 and the counter CNT has no effect on the outputs. 0001: Set channel 1 to active level on match. OC1REF signal is forced high when the counter CNT matches the capture/compare register 1 (CCR1). 0010: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter CNT matches the capture/compare register 1 (CCR1). 0011: Toggle - OC1REF toggles when CNT=CCR1. 0100: Force inactive level - OC1REF is forced low. 0101: Force active level - OC1REF is forced high. 0110: PWM mode 1 - In upcounting, channel 1 is active as long as CNTltCCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF=0) as long as CNT>CCR1 else active (OC1REF=1). 0111: PWM mode 2 - In upcounting, channel 1 is inactive as long as CNTltCCR1 else active. In downcounting, channel 1 is active as long as CNT>CCR1 else inactive. 1000: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update. 1001: Retriggerable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. 1010: Reserved, 1011: Reserved, 1100: Combined PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC is the logical OR between OC1REF and OC2REF. 1101: Combined PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC is the logical AND between OC1REF and OC2REF. 1110: Asymmetric PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down. 1111: Asymmetric PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down. These bits can not be modified as long as LOCK level 3 has been programmed and CC1S=00 (the channel is configured in output). On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn ocm(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 20usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Output compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. 0000: Frozen - The comparison between the output compare register CCR1 and the counter CNT has no effect on the outputs. 0001: Set channel 1 to active level on match. OC1REF signal is forced high when the counter CNT matches the capture/compare register 1 (CCR1). 0010: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter CNT matches the capture/compare register 1 (CCR1). 0011: Toggle - OC1REF toggles when CNT=CCR1. 0100: Force inactive level - OC1REF is forced low. 0101: Force active level - OC1REF is forced high. 0110: PWM mode 1 - In upcounting, channel 1 is active as long as CNTltCCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF=0) as long as CNT>CCR1 else active (OC1REF=1). 0111: PWM mode 2 - In upcounting, channel 1 is inactive as long as CNTltCCR1 else active. In downcounting, channel 1 is active as long as CNT>CCR1 else inactive. 1000: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update. In down-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes inactive again at the next update. 1001: Retriggerable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 2 and the channels becomes inactive again at the next update. In down-counting mode, the channel is active until a trigger event is detected (on TRGI signal). Then, a comparison is performed as in PWM mode 1 and the channels becomes active again at the next update. 1010: Reserved, 1011: Reserved, 1100: Combined PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC is the logical OR between OC1REF and OC2REF. 1101: Combined PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC is the logical AND between OC1REF and OC2REF. 1110: Asymmetric PWM mode 1 - OC1REF has the same behavior as in PWM mode 1. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down. 1111: Asymmetric PWM mode 2 - OC1REF has the same behavior as in PWM mode 2. OC1REFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down. These bits can not be modified as long as LOCK level 3 has been programmed and CC1S=00 (the channel is configured in output). On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated."]
    #[inline(always)]
    pub const fn set_ocm(&mut self, n: usize, val: u8) {
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
            .field("ccs[0]", &self.ccs(0usize))
            .field("ccs[1]", &self.ccs(1usize))
            .field("icpsc[0]", &self.icpsc(0usize))
            .field("icpsc[1]", &self.icpsc(1usize))
            .field("icf[0]", &self.icf(0usize))
            .field("icf[1]", &self.icf(1usize))
            .field("occe[0]", &self.occe(0usize))
            .field("occe[1]", &self.occe(1usize))
            .field("ocpe[0]", &self.ocpe(0usize))
            .field("ocpe[1]", &self.ocpe(1usize))
            .field("ocm[0]", &self.ocm(0usize))
            .field("ocm[1]", &self.ocm(1usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccmr1 {{ ccs[0]: {=u8:?}, ccs[1]: {=u8:?}, icpsc[0]: {=u8:?}, icpsc[1]: {=u8:?}, icf[0]: {=u8:?}, icf[1]: {=u8:?}, occe[0]: {=bool:?}, occe[1]: {=bool:?}, ocpe[0]: {=bool:?}, ocpe[1]: {=bool:?}, ocm[0]: {=u8:?}, ocm[1]: {=u8:?} }}" , self . ccs (0usize) , self . ccs (1usize) , self . icpsc (0usize) , self . icpsc (1usize) , self . icf (0usize) , self . icf (1usize) , self . occe (0usize) , self . occe (1usize) , self . ocpe (0usize) , self . ocpe (1usize) , self . ocm (0usize) , self . ocm (1usize))
    }
}
#[doc = "TIM capture/compare mode register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr2(pub u32);
impl Ccmr2 {
    #[doc = "Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)"]
    #[must_use]
    #[inline(always)]
    pub const fn ccs(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. 00: CC3 channel is configured as output 01: CC3 channel is configured as input, IC3 is mapped on TI3 10: CC3 channel is configured as input, IC3 is mapped on TI4 11: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (SMCR register)"]
    #[inline(always)]
    pub const fn set_ccs(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
    }
    #[doc = "Input capture 3 prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn icpsc(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Input capture 3 prescaler"]
    #[inline(always)]
    pub const fn set_icpsc(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
    }
    #[doc = "Input capture 3 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn icf(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 4usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 3 filter"]
    #[inline(always)]
    pub const fn set_icf(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 4usize + n * 8usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
    }
    #[doc = "Output compare 3 clear enable"]
    #[must_use]
    #[inline(always)]
    pub const fn occe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 clear enable"]
    #[inline(always)]
    pub const fn set_occe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 3 preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ocpe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 preload enable"]
    #[inline(always)]
    pub const fn set_ocpe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 3 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ocm(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 20usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Output compare 3 mode"]
    #[inline(always)]
    pub const fn set_ocm(&mut self, n: usize, val: u8) {
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
            .field("ccs[0]", &self.ccs(0usize))
            .field("ccs[1]", &self.ccs(1usize))
            .field("icpsc[0]", &self.icpsc(0usize))
            .field("icpsc[1]", &self.icpsc(1usize))
            .field("icf[0]", &self.icf(0usize))
            .field("icf[1]", &self.icf(1usize))
            .field("occe[0]", &self.occe(0usize))
            .field("occe[1]", &self.occe(1usize))
            .field("ocpe[0]", &self.ocpe(0usize))
            .field("ocpe[1]", &self.ocpe(1usize))
            .field("ocm[0]", &self.ocm(0usize))
            .field("ocm[1]", &self.ocm(1usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccmr2 {{ ccs[0]: {=u8:?}, ccs[1]: {=u8:?}, icpsc[0]: {=u8:?}, icpsc[1]: {=u8:?}, icf[0]: {=u8:?}, icf[1]: {=u8:?}, occe[0]: {=bool:?}, occe[1]: {=bool:?}, ocpe[0]: {=bool:?}, ocpe[1]: {=bool:?}, ocm[0]: {=u8:?}, ocm[1]: {=u8:?} }}" , self . ccs (0usize) , self . ccs (1usize) , self . icpsc (0usize) , self . icpsc (1usize) , self . icf (0usize) , self . icf (1usize) , self . occe (0usize) , self . occe (1usize) , self . ocpe (0usize) , self . ocpe (1usize) , self . ocm (0usize) , self . ocm (1usize))
    }
}
#[doc = "TIM capture/compare mode register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr3(pub u32);
impl Ccmr3 {
    #[doc = "Group Channel 5 and Channel 1 Distortion on Channel 1 output: 0: No effect of OC5REF on OC1REFC5 1: OC1REFC is the logical AND of OC1REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1)."]
    #[must_use]
    #[inline(always)]
    pub const fn gc5c1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Group Channel 5 and Channel 1 Distortion on Channel 1 output: 0: No effect of OC5REF on OC1REFC5 1: OC1REFC is the logical AND of OC1REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1)."]
    #[inline(always)]
    pub const fn set_gc5c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Group Channel 5 and Channel 2 Distortion on Channel 2 output: 0: No effect of OC5REF on OC2REFC 1: OC2REFC is the logical AND of OC2REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1)."]
    #[must_use]
    #[inline(always)]
    pub const fn gc5c2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Group Channel 5 and Channel 2 Distortion on Channel 2 output: 0: No effect of OC5REF on OC2REFC 1: OC2REFC is the logical AND of OC2REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1)."]
    #[inline(always)]
    pub const fn set_gc5c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Group Channel 5 and Channel 3 Distortion on Channel 3 output: 0: No effect of OC5REF on OC3REFC 1: OC3REFC is the logical AND of OC3REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2)."]
    #[must_use]
    #[inline(always)]
    pub const fn gc5c3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Group Channel 5 and Channel 3 Distortion on Channel 3 output: 0: No effect of OC5REF on OC3REFC 1: OC3REFC is the logical AND of OC3REFC and OC5REF This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2)."]
    #[inline(always)]
    pub const fn set_gc5c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Output compare 5 clear enable"]
    #[must_use]
    #[inline(always)]
    pub const fn occe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 5 clear enable"]
    #[inline(always)]
    pub const fn set_occe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 16usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 5 preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ocpe(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output compare 5 preload enable"]
    #[inline(always)]
    pub const fn set_ocpe(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 19usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output compare 5 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ocm(&self, n: usize) -> u8 {
        assert!(n < 2usize);
        let offs = 20usize + n * 8usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Output compare 5 mode"]
    #[inline(always)]
    pub const fn set_ocm(&mut self, n: usize, val: u8) {
        assert!(n < 2usize);
        let offs = 20usize + n * 8usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
    }
}
impl Default for Ccmr3 {
    #[inline(always)]
    fn default() -> Ccmr3 {
        Ccmr3(0)
    }
}
impl core::fmt::Debug for Ccmr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr3")
            .field("gc5c1", &self.gc5c1())
            .field("gc5c2", &self.gc5c2())
            .field("gc5c3", &self.gc5c3())
            .field("occe[0]", &self.occe(0usize))
            .field("occe[1]", &self.occe(1usize))
            .field("ocpe[0]", &self.ocpe(0usize))
            .field("ocpe[1]", &self.ocpe(1usize))
            .field("ocm[0]", &self.ocm(0usize))
            .field("ocm[1]", &self.ocm(1usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccmr3 {{ gc5c1: {=bool:?}, gc5c2: {=bool:?}, gc5c3: {=bool:?}, occe[0]: {=bool:?}, occe[1]: {=bool:?}, ocpe[0]: {=bool:?}, ocpe[1]: {=bool:?}, ocm[0]: {=u8:?}, ocm[1]: {=u8:?} }}" , self . gc5c1 () , self . gc5c2 () , self . gc5c3 () , self . occe (0usize) , self . occe (1usize) , self . ocpe (0usize) , self . ocpe (1usize) , self . ocm (0usize) , self . ocm (1usize))
    }
}
#[doc = "Capture/Compare register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value).It is loaded permanently if the preload feature is not selected in the CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter CNT and signaled on OC1 output. If channel CC1is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1)."]
    #[must_use]
    #[inline(always)]
    pub const fn ccr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value).It is loaded permanently if the preload feature is not selected in the CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter CNT and signaled on OC1 output. If channel CC1is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1)."]
    #[inline(always)]
    pub const fn set_ccr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Ccr {{ ccr: {=u32:?} }}", self.ccr())
    }
}
#[doc = "Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "bit 30 to 0 is the lower bits of counter value bit 31 depends on IUFREMAP in CR1. If UIFREMAP = 1 this bit is a read-only copy of the UIF bit of the ISR register If UIFREMAP = 0 this bit is counter value bit 31"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit 30 to 0 is the lower bits of counter value bit 31 depends on IUFREMAP in CR1. If UIFREMAP = 1 this bit is a read-only copy of the UIF bit of the ISR register If UIFREMAP = 0 this bit is counter value bit 31"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Cnt {{ cnt: {=u32:?} }}", self.cnt())
    }
}
#[doc = "TIM control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Counter enable 0: Counter disabled 1: Counter enabled External clock, gated mode and encoder mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter enable 0: Counter disabled 1: Counter enabled External clock, gated mode and encoder mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs."]
    #[inline(always)]
    pub const fn set_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Update disable This bit is set and cleared by software to enable/disable UEV event generation. 0: UEV enabled. The Update (UEV) event is generated by one of the following events: Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values. 1: UEV disabled. The Update event is not generated, shadow registers keep their value (ARR, PSC, CCRx). However the counter and the prescaler are reinitialized if the UG bit is set or if a hardware reset is received from the slave mode controller."]
    #[must_use]
    #[inline(always)]
    pub const fn udis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Update disable This bit is set and cleared by software to enable/disable UEV event generation. 0: UEV enabled. The Update (UEV) event is generated by one of the following events: Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values. 1: UEV disabled. The Update event is not generated, shadow registers keep their value (ARR, PSC, CCRx). However the counter and the prescaler are reinitialized if the UG bit is set or if a hardware reset is received from the slave mode controller."]
    #[inline(always)]
    pub const fn set_udis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Update request source This bit is set and cleared by software to select the UEV event sources. 0: Any of the following events generate an update interrupt or DMA request if enabled. These events can be: Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller 1: Only counter overflow/underflow generates an update interrupt or DMA request if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn urs(&self) -> super::super::tim_common::vals::URS {
        let val = (self.0 >> 2usize) & 0x01;
        super::super::tim_common::vals::URS::from_bits(val as u8)
    }
    #[doc = "Update request source This bit is set and cleared by software to select the UEV event sources. 0: Any of the following events generate an update interrupt or DMA request if enabled. These events can be: Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller 1: Only counter overflow/underflow generates an update interrupt or DMA request if enabled."]
    #[inline(always)]
    pub const fn set_urs(&mut self, val: super::super::tim_common::vals::URS) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "One-pulse mode 0: Counter is not stopped at update event 1: Counter stops counting at the next update event (clearing the bit CEN)"]
    #[must_use]
    #[inline(always)]
    pub const fn opm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "One-pulse mode 0: Counter is not stopped at update event 1: Counter stops counting at the next update event (clearing the bit CEN)"]
    #[inline(always)]
    pub const fn set_opm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Direction 0: Counter used as upcounter 1: Counter used as downcounter"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::super::tim_common::vals::DIR {
        let val = (self.0 >> 4usize) & 0x01;
        super::super::tim_common::vals::DIR::from_bits(val as u8)
    }
    #[doc = "Direction 0: Counter used as upcounter 1: Counter used as downcounter"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::super::tim_common::vals::DIR) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Center-aligned mode selection 00: Edge-aligned mode. The counter counts up or down depending on the direction bit (DIR). 01: Center-aligned mode 1. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set only when the counter is counting down. 10: Center-aligned mode 2. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set only when the counter is counting up. 11: Center-aligned mode 3. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set both when the counter is counting up or down."]
    #[must_use]
    #[inline(always)]
    pub const fn cms(&self) -> super::super::tim_common::vals::CMS {
        let val = (self.0 >> 5usize) & 0x03;
        super::super::tim_common::vals::CMS::from_bits(val as u8)
    }
    #[doc = "Center-aligned mode selection 00: Edge-aligned mode. The counter counts up or down depending on the direction bit (DIR). 01: Center-aligned mode 1. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set only when the counter is counting down. 10: Center-aligned mode 2. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set only when the counter is counting up. 11: Center-aligned mode 3. The counter counts up and down alternatively. Output compare interrupt flags of channels configured in output (CCxS=00 in CCMRx register) are set both when the counter is counting up or down."]
    #[inline(always)]
    pub const fn set_cms(&mut self, val: super::super::tim_common::vals::CMS) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Auto-reload preload enable 0: ARR register is not buffered 1: ARR register is buffered"]
    #[must_use]
    #[inline(always)]
    pub const fn arpe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-reload preload enable 0: ARR register is not buffered 1: ARR register is buffered"]
    #[inline(always)]
    pub const fn set_arpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UIF status bit remapping 0: No remapping. UIF status bit is not copied to CNT register bit 31 1: Remapping enabled. UIF status bit is copied to CNT register bit 31."]
    #[must_use]
    #[inline(always)]
    pub const fn uifremap(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "UIF status bit remapping 0: No remapping. UIF status bit is not copied to CNT register bit 31 1: Remapping enabled. UIF status bit is copied to CNT register bit 31."]
    #[inline(always)]
    pub const fn set_uifremap(&mut self, val: bool) {
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
        defmt :: write ! (f , "Cr1 {{ cen: {=bool:?}, udis: {=bool:?}, urs: {:?}, opm: {=bool:?}, dir: {:?}, cms: {:?}, arpe: {=bool:?}, uifremap: {=bool:?} }}" , self . cen () , self . udis () , self . urs () , self . opm () , self . dir () , self . cms () , self . arpe () , self . uifremap ())
    }
}
#[doc = "TIM control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Capture/compare preloaded control 0: CCxE, CCxNE and OCxM bits are not preloaded 1: CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or edge detected on TRGI after Trigger selection, depending on the CCUS bit). This bit acts only on channels that have a complementary output."]
    #[must_use]
    #[inline(always)]
    pub const fn ccpc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare preloaded control 0: CCxE, CCxNE and OCxM bits are not preloaded 1: CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or edge detected on TRGI after Trigger selection, depending on the CCUS bit). This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub const fn set_ccpc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/compare control update selection 0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only 1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an edge occurs on TRGI after Trigger selection This bit acts only on channels that have a complementary output."]
    #[must_use]
    #[inline(always)]
    pub const fn ccus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare control update selection 0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only 1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an edge occurs on TRGI after Trigger selection This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub const fn set_ccus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/compare DMA selection 0: CCx DMA request sent when CCx event occurs 1: CCx DMA requests sent when update event occurs"]
    #[must_use]
    #[inline(always)]
    pub const fn ccds(&self) -> super::super::tim_common::vals::CCDS {
        let val = (self.0 >> 3usize) & 0x01;
        super::super::tim_common::vals::CCDS::from_bits(val as u8)
    }
    #[doc = "Capture/compare DMA selection 0: CCx DMA request sent when CCx event occurs 1: CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub const fn set_ccds(&mut self, val: super::super::tim_common::vals::CCDS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: 000: Reset - the UG bit from the EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset. 001: Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic OR between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected. 010: Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer. 011: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO) 100: Compare - OC1REFC signal is used as trigger output (TRGO) 101: Compare - OC2REFC signal is used as trigger output (TRGO) 110: Compare - OC3REFC signal is used as trigger output (TRGO) 111: Compare - OC4REFC signal is used as trigger output (TRGO)"]
    #[must_use]
    #[inline(always)]
    pub const fn mms(&self) -> super::super::tim_common::vals::MMS {
        let val = (self.0 >> 4usize) & 0x07;
        super::super::tim_common::vals::MMS::from_bits(val as u8)
    }
    #[doc = "Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: 000: Reset - the UG bit from the EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset. 001: Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic OR between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected. 010: Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer. 011: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO) 100: Compare - OC1REFC signal is used as trigger output (TRGO) 101: Compare - OC2REFC signal is used as trigger output (TRGO) 110: Compare - OC3REFC signal is used as trigger output (TRGO) 111: Compare - OC4REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub const fn set_mms(&mut self, val: super::super::tim_common::vals::MMS) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "TI1 selection 0: The CH1 pin is connected to TI1 input 1: The CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[must_use]
    #[inline(always)]
    pub const fn ti1s(&self) -> super::super::tim_common::vals::TI1S {
        let val = (self.0 >> 7usize) & 0x01;
        super::super::tim_common::vals::TI1S::from_bits(val as u8)
    }
    #[doc = "TI1 selection 0: The CH1 pin is connected to TI1 input 1: The CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub const fn set_ti1s(&mut self, val: super::super::tim_common::vals::TI1S) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Output Idle state 1 (OC1 output) 0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0 1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0 This bit, as well as other OISx, can not be modified as long as LOCK level 1, 2 or 3 has been programmed"]
    #[must_use]
    #[inline(always)]
    pub const fn ois(&self, n: usize) -> bool {
        assert!(n < 6usize);
        let offs = 8usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 1 (OC1 output) 0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0 1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0 This bit, as well as other OISx, can not be modified as long as LOCK level 1, 2 or 3 has been programmed"]
    #[inline(always)]
    pub const fn set_ois(&mut self, n: usize, val: bool) {
        assert!(n < 6usize);
        let offs = 8usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Output Idle state 1 (OC1N output) 0: OC1N=0 after a dead-time when MOE=0 1: OC1N=1 after a dead-time when MOE=0 This bit, as well as other OISxN, can not be modified as long as LOCK level 1, 2 or 3 has been programmed"]
    #[must_use]
    #[inline(always)]
    pub const fn oisn(&self, n: usize) -> bool {
        assert!(n < 3usize);
        let offs = 9usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 1 (OC1N output) 0: OC1N=0 after a dead-time when MOE=0 1: OC1N=1 after a dead-time when MOE=0 This bit, as well as other OISxN, can not be modified as long as LOCK level 1, 2 or 3 has been programmed"]
    #[inline(always)]
    pub const fn set_oisn(&mut self, n: usize, val: bool) {
        assert!(n < 3usize);
        let offs = 9usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .field("ois[0]", &self.ois(0usize))
            .field("ois[1]", &self.ois(1usize))
            .field("ois[2]", &self.ois(2usize))
            .field("ois[3]", &self.ois(3usize))
            .field("ois[4]", &self.ois(4usize))
            .field("ois[5]", &self.ois(5usize))
            .field("oisn[0]", &self.oisn(0usize))
            .field("oisn[1]", &self.oisn(1usize))
            .field("oisn[2]", &self.oisn(2usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cr2 {{ ccpc: {=bool:?}, ccus: {=bool:?}, ccds: {:?}, mms: {:?}, ti1s: {:?}, ois[0]: {=bool:?}, ois[1]: {=bool:?}, ois[2]: {=bool:?}, ois[3]: {=bool:?}, ois[4]: {=bool:?}, ois[5]: {=bool:?}, oisn[0]: {=bool:?}, oisn[1]: {=bool:?}, oisn[2]: {=bool:?} }}" , self . ccpc () , self . ccus () , self . ccds () , self . mms () , self . ti1s () , self . ois (0usize) , self . ois (1usize) , self . ois (2usize) , self . ois (3usize) , self . ois (4usize) , self . ois (5usize) , self . oisn (0usize) , self . oisn (1usize) , self . oisn (2usize))
    }
}
#[doc = "TIM DMA/Interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dier(pub u32);
impl Dier {
    #[doc = "Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn uie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt enable 0: Update interrupt disabled. 1: Update interrupt enabled"]
    #[inline(always)]
    pub const fn set_uie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn ccie(&self, n: usize) -> bool {
        assert!(n < 6usize);
        let offs = 1usize + ([0usize, 1usize, 2usize, 3usize, 15usize, 16usize][n] as usize);
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled"]
    #[inline(always)]
    pub const fn set_ccie(&mut self, n: usize, val: bool) {
        assert!(n < 6usize);
        let offs = 1usize + ([0usize, 1usize, 2usize, 3usize, 15usize, 16usize][n] as usize);
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn comie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled"]
    #[inline(always)]
    pub const fn set_comie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger interrupt enable 0: Trigger interrupt disabled. 1: Trigger interrupt enabled"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn bie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled"]
    #[inline(always)]
    pub const fn set_bie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn ude(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Update DMA request enable 0: Update DMA request disabled. 1: Update DMA request enabled"]
    #[inline(always)]
    pub const fn set_ude(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled. 1: CC1 DMA request enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ccde(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 9usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled. 1: CC1 DMA request enabled."]
    #[inline(always)]
    pub const fn set_ccde(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 9usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "COM DMA request enable 0: COM DMA request disabled 1: COM DMA request enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn comde(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "COM DMA request enable 0: COM DMA request disabled 1: COM DMA request enabled"]
    #[inline(always)]
    pub const fn set_comde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn tde(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA request enable 0: Trigger DMA request disabled. 1: Trigger DMA request enabled."]
    #[inline(always)]
    pub const fn set_tde(&mut self, val: bool) {
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
            .field("ccie[0]", &self.ccie(0usize))
            .field("ccie[1]", &self.ccie(1usize))
            .field("ccie[2]", &self.ccie(2usize))
            .field("ccie[3]", &self.ccie(3usize))
            .field("ccie[4]", &self.ccie(4usize))
            .field("ccie[5]", &self.ccie(5usize))
            .field("comie", &self.comie())
            .field("tie", &self.tie())
            .field("bie", &self.bie())
            .field("ude", &self.ude())
            .field("ccde[0]", &self.ccde(0usize))
            .field("ccde[1]", &self.ccde(1usize))
            .field("ccde[2]", &self.ccde(2usize))
            .field("ccde[3]", &self.ccde(3usize))
            .field("comde", &self.comde())
            .field("tde", &self.tde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dier {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dier {{ uie: {=bool:?}, ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, ccie[2]: {=bool:?}, ccie[3]: {=bool:?}, ccie[4]: {=bool:?}, ccie[5]: {=bool:?}, comie: {=bool:?}, tie: {=bool:?}, bie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?}, ccde[1]: {=bool:?}, ccde[2]: {=bool:?}, ccde[3]: {=bool:?}, comde: {=bool:?}, tde: {=bool:?} }}" , self . uie () , self . ccie (0usize) , self . ccie (1usize) , self . ccie (2usize) , self . ccie (3usize) , self . ccie (4usize) , self . ccie (5usize) , self . comie () , self . tie () , self . bie () , self . ude () , self . ccde (0usize) , self . ccde (1usize) , self . ccde (2usize) , self . ccde (3usize) , self . comde () , self . tde ())
    }
}
#[doc = "Event generation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Egr(pub u32);
impl Egr {
    #[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: Re-initialize the counter and generates an update of the registers. The prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (ARR) if DIR=1 (downcounting)."]
    #[must_use]
    #[inline(always)]
    pub const fn ug(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update generation This bit can be set by software, it is automatically cleared by hardware. 0: No action 1: Re-initialize the counter and generates an update of the registers. The prescaler counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if the center-aligned mode is selected or if DIR=0 (upcounting), else it takes the auto-reload value (ARR) if DIR=1 (downcounting)."]
    #[inline(always)]
    pub const fn set_ug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[must_use]
    #[inline(always)]
    pub const fn ccg(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A capture/compare event is generated on channel 1: If channel CC1 is configured as output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If channel CC1 is configured as input: The current value of the counter is captured in CCR1 register. The CC1IF flag is set, the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    pub const fn set_ccg(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware 0: No action 1: When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits This bit acts only on channels having a complementary output."]
    #[must_use]
    #[inline(always)]
    pub const fn comg(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare control update generation This bit can be set by software, it is automatically cleared by hardware 0: No action 1: When CCPC bit is set, it allows to update CCxE, CCxNE and OCxM bits This bit acts only on channels having a complementary output."]
    #[inline(always)]
    pub const fn set_comg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn tg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: The TIF flag is set in SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub const fn set_tg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn bg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub const fn set_bg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn b2g(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Break 2 generation This bit is set by software in order to generate an event, it is automatically cleared by hardware. 0: No action 1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled."]
    #[inline(always)]
    pub const fn set_b2g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
            .field("ccg[0]", &self.ccg(0usize))
            .field("ccg[1]", &self.ccg(1usize))
            .field("ccg[2]", &self.ccg(2usize))
            .field("ccg[3]", &self.ccg(3usize))
            .field("comg", &self.comg())
            .field("tg", &self.tg())
            .field("bg", &self.bg())
            .field("b2g", &self.b2g())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Egr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Egr {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, ccg[2]: {=bool:?}, ccg[3]: {=bool:?}, comg: {=bool:?}, tg: {=bool:?}, bg: {=bool:?}, b2g: {=bool:?} }}" , self . ug () , self . ccg (0usize) , self . ccg (1usize) , self . ccg (2usize) , self . ccg (3usize) , self . comg () , self . tg () , self . bg () , self . b2g ())
    }
}
#[doc = "Prescaler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psc(pub u32);
impl Psc {
    #[doc = "Prescaler value The counter clock frequency is fCLK/(PSC+1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of EGR register or through trigger controller when configured in \"reset mode\")."]
    #[must_use]
    #[inline(always)]
    pub const fn psc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Prescaler value The counter clock frequency is fCLK/(PSC+1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of EGR register or through trigger controller when configured in \"reset mode\")."]
    #[inline(always)]
    pub const fn set_psc(&mut self, val: u16) {
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
        defmt::write!(f, "Psc {{ psc: {=u16:?} }}", self.psc())
    }
}
#[doc = "Repetition counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Repetition counter value These bits allow the user to set-up the update rate of the compare registers when preload registers are enable, as well as the update interrupt generation rate, if this interrupt is enable. Each time the REP_CNT related downcounter reaches zero, an update event is generated and it restarts counting from REP value. As REP_CNT is reloaded with REP value only at the repetition update event, any write to the RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode or the number of half PWM period in center-aligned mode.."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Repetition counter value These bits allow the user to set-up the update rate of the compare registers when preload registers are enable, as well as the update interrupt generation rate, if this interrupt is enable. Each time the REP_CNT related downcounter reaches zero, an update event is generated and it restarts counting from REP value. As REP_CNT is reloaded with REP value only at the repetition update event, any write to the RCR register is not taken in account until the next repetition update event. It means in PWM mode (REP+1) corresponds to the number of PWM periods in edge-aligned mode or the number of half PWM period in center-aligned mode.."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
        defmt::write!(f, "Rcr {{ rep: {=u16:?} }}", self.rep())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd1(pub u32);
impl Rsvd1 {}
impl Default for Rsvd1 {
    #[inline(always)]
    fn default() -> Rsvd1 {
        Rsvd1(0)
    }
}
impl core::fmt::Debug for Rsvd1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd1 {{ }}",)
    }
}
#[doc = "TIM slave mode control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcr(pub u32);
impl Smcr {
    #[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 000: Internal Trigger 0 (ITR0) 001: Internal Trigger 1 (ITR1) 010: Internal Trigger 2 (ITR2) 011: Internal Trigger 3 (ITR3) 100: TI1 Edge Detector (TI1F_ED) 101: Filtered Timer Input 1 (TI1FP1) 110: Filtered Timer Input 2 (TI2FP2) 111: External Trigger input (ETRF)"]
    #[must_use]
    #[inline(always)]
    pub const fn ts(&self) -> super::super::tim_common::vals::TS {
        let val = (self.0 >> 4usize) & 0x07;
        super::super::tim_common::vals::TS::from_bits(val as u8)
    }
    #[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 000: Internal Trigger 0 (ITR0) 001: Internal Trigger 1 (ITR1) 010: Internal Trigger 2 (ITR2) 011: Internal Trigger 3 (ITR3) 100: TI1 Edge Detector (TI1F_ED) 101: Filtered Timer Input 1 (TI1FP1) 110: Filtered Timer Input 2 (TI2FP2) 111: External Trigger input (ETRF)"]
    #[inline(always)]
    pub const fn set_ts(&mut self, val: super::super::tim_common::vals::TS) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master/Slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[must_use]
    #[inline(always)]
    pub const fn msm(&self) -> super::super::tim_common::vals::MSM {
        let val = (self.0 >> 7usize) & 0x01;
        super::super::tim_common::vals::MSM::from_bits(val as u8)
    }
    #[doc = "Master/Slave mode 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub const fn set_msm(&mut self, val: super::super::tim_common::vals::MSM) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, sampling is done at fCLK 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8"]
    #[must_use]
    #[inline(always)]
    pub const fn etf(&self) -> super::super::tim_common::vals::ETF {
        let val = (self.0 >> 8usize) & 0x0f;
        super::super::tim_common::vals::ETF::from_bits(val as u8)
    }
    #[doc = "External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: 0000: No filter, sampling is done at fCLK 0001: fSAMPLING=fCLK, N=2 0010: fSAMPLING=fCLK, N=4 0011: fSAMPLING=fCLK, N=8 0100: fSAMPLING=fCLK/2, N=6 0101: fSAMPLING=fCLK/2, N=8 0110: fSAMPLING=fCLK/4, N=6 0111: fSAMPLING=fCLK/4, N=8 1000: fSAMPLING=fCLK/8, N=6 1001: fSAMPLING=fCLK/8, N=8 1010: fSAMPLING=fCLK/16, N=5 1011: fSAMPLING=fCLK/16, N=6 1100: fSAMPLING=fCLK/16, N=8 1101: fSAMPLING=fCLK/32, N=5 1110: fSAMPLING=fCLK/32, N=6 1111: fSAMPLING=fCLK/32, N=8"]
    #[inline(always)]
    pub const fn set_etf(&mut self, val: super::super::tim_common::vals::ETF) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8"]
    #[must_use]
    #[inline(always)]
    pub const fn etps(&self) -> super::super::tim_common::vals::ETPS {
        let val = (self.0 >> 12usize) & 0x03;
        super::super::tim_common::vals::ETPS::from_bits(val as u8)
    }
    #[doc = "External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8"]
    #[inline(always)]
    pub const fn set_etps(&mut self, val: super::super::tim_common::vals::ETPS) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[must_use]
    #[inline(always)]
    pub const fn ece(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub const fn set_ece(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge 1: ETR is inverted, active at low level or falling edge"]
    #[must_use]
    #[inline(always)]
    pub const fn etp(&self) -> super::super::tim_common::vals::ETP {
        let val = (self.0 >> 15usize) & 0x01;
        super::super::tim_common::vals::ETP::from_bits(val as u8)
    }
    #[doc = "External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge 1: ETR is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub const fn set_etp(&mut self, val: super::super::tim_common::vals::ETP) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input. 0000: Slave mode disabled. 0001: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0010: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn sms(&self) -> super::super::tim_common::vals::SMS {
        let val = (self.0 >> 16usize) & 0x0f;
        super::super::tim_common::vals::SMS::from_bits(val as u8)
    }
    #[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input. 0000: Slave mode disabled. 0001: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0010: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter."]
    #[inline(always)]
    pub const fn set_sms(&mut self, val: super::super::tim_common::vals::SMS) {
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
        defmt :: write ! (f , "Smcr {{ ts: {:?}, msm: {:?}, etf: {:?}, etps: {:?}, ece: {=bool:?}, etp: {:?}, sms: {:?} }}" , self . ts () , self . msm () , self . etf () , self . etps () , self . ece () , self . etp () , self . sms ())
    }
}
#[doc = "TIM status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred 1: Update interrupt pending. This bit is set by hardware when the registers are updated: - At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if UDIS=0 in the CR1 register. - When CNT is reinitialized by software using the UG bit in EGR register, if URS=0 and UDIS=0 in the CR1 register. - When CNT is reinitialized by a trigger event, if URS=0 and UDIS=0 in the CR1 register."]
    #[must_use]
    #[inline(always)]
    pub const fn uif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. 0: No update occurred 1: Update interrupt pending. This bit is set by hardware when the registers are updated: - At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if UDIS=0 in the CR1 register. - When CNT is reinitialized by software using the UG bit in EGR register, if URS=0 and UDIS=0 in the CR1 register. - When CNT is reinitialized by a trigger event, if URS=0 and UDIS=0 in the CR1 register."]
    #[inline(always)]
    pub const fn set_uif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value and in retriggerable one pulse mode. It is cleared by software. 0: No match. 1: The content of the counter CNT has matched the content of the CCR1 register. If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the CCR1 register. 0: No input capture occurred. 1: The counter value has been captured in CCR1 register."]
    #[must_use]
    #[inline(always)]
    pub const fn ccif(&self, n: usize) -> bool {
        assert!(n < 6usize);
        let offs = 1usize + ([0usize, 1usize, 2usize, 3usize, 15usize, 16usize][n] as usize);
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt flag If channel CC1 is configured as output: This flag is set by hardware when the counter matches the compare value and in retriggerable one pulse mode. It is cleared by software. 0: No match. 1: The content of the counter CNT has matched the content of the CCR1 register. If channel CC1 is configured as input: This bit is set by hardware on a capture. It is cleared by software or by reading the CCR1 register. 0: No input capture occurred. 1: The counter value has been captured in CCR1 register."]
    #[inline(always)]
    pub const fn set_ccif(&mut self, n: usize, val: bool) {
        assert!(n < 6usize);
        let offs = 1usize + ([0usize, 1usize, 2usize, 3usize, 15usize, 16usize][n] as usize);
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software. 0: No COM event occurred. 1: COM interrupt pending."]
    #[must_use]
    #[inline(always)]
    pub const fn comif(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software. 0: No COM event occurred. 1: COM interrupt pending."]
    #[inline(always)]
    pub const fn set_comif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger interrupt flag This flag is set by hardware on trigger event. It is set when the counter starts or stops when gated mode is selected. It is cleared by software. 0: No trigger event occurred. 1: Trigger interrupt pending."]
    #[must_use]
    #[inline(always)]
    pub const fn tif(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger interrupt flag This flag is set by hardware on trigger event. It is set when the counter starts or stops when gated mode is selected. It is cleared by software. 0: No trigger event occurred. 1: Trigger interrupt pending."]
    #[inline(always)]
    pub const fn set_tif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active. 0: No break event occurred. 1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the DIER register."]
    #[must_use]
    #[inline(always)]
    pub const fn bif(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active. 0: No break event occurred. 1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the DIER register."]
    #[inline(always)]
    pub const fn set_bif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active. 0: No break event occurred. 1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the DIER register."]
    #[must_use]
    #[inline(always)]
    pub const fn b2if(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active. 0: No break event occurred. 1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the DIER register."]
    #[inline(always)]
    pub const fn set_b2if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected. 1: The counter value has been captured in CCR1 register while CC1IF flag was already set"]
    #[must_use]
    #[inline(always)]
    pub const fn ccof(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 9usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'. 0: No overcapture has been detected. 1: The counter value has been captured in CCR1 register while CC1IF flag was already set"]
    #[inline(always)]
    pub const fn set_ccof(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 9usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation. 0: No break event occurred. 1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the DIER register."]
    #[must_use]
    #[inline(always)]
    pub const fn sbif(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation. 0: No break event occurred. 1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the DIER register."]
    #[inline(always)]
    pub const fn set_sbif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("ccif[0]", &self.ccif(0usize))
            .field("ccif[1]", &self.ccif(1usize))
            .field("ccif[2]", &self.ccif(2usize))
            .field("ccif[3]", &self.ccif(3usize))
            .field("ccif[4]", &self.ccif(4usize))
            .field("ccif[5]", &self.ccif(5usize))
            .field("comif", &self.comif())
            .field("tif", &self.tif())
            .field("bif", &self.bif())
            .field("b2if", &self.b2if())
            .field("ccof[0]", &self.ccof(0usize))
            .field("ccof[1]", &self.ccof(1usize))
            .field("ccof[2]", &self.ccof(2usize))
            .field("ccof[3]", &self.ccof(3usize))
            .field("sbif", &self.sbif())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sr {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, ccif[2]: {=bool:?}, ccif[3]: {=bool:?}, ccif[4]: {=bool:?}, ccif[5]: {=bool:?}, comif: {=bool:?}, tif: {=bool:?}, bif: {=bool:?}, b2if: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?}, ccof[2]: {=bool:?}, ccof[3]: {=bool:?}, sbif: {=bool:?} }}" , self . uif () , self . ccif (0usize) , self . ccif (1usize) , self . ccif (2usize) , self . ccif (3usize) , self . ccif (4usize) , self . ccif (5usize) , self . comif () , self . tif () , self . bif () , self . b2if () , self . ccof (0usize) , self . ccof (1usize) , self . ccof (2usize) , self . ccof (3usize) , self . sbif ())
    }
}
