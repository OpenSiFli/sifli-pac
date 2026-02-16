#[doc = "Clock Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys"]
    #[must_use]
    #[inline(always)]
    pub const fn hdiv1(&self) -> super::vals::Hdiv {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Hdiv::from_bits(val as u8)
    }
    #[doc = "hclk_lpsys = clk_lpsys / HDIV if HDIV=0, hclk_lpsys = clk_lpsys"]
    #[inline(always)]
    pub const fn set_hdiv1(&mut self, val: super::vals::Hdiv) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2"]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv1(&self) -> super::vals::Pdiv {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pdiv::from_bits(val as u8)
    }
    #[doc = "pclk1_lpsys = hclk_lpsys / (2^PDIV1), by default divided by 2"]
    #[inline(always)]
    pub const fn set_pdiv1(&mut self, val: super::vals::Pdiv) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32"]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv2(&self) -> super::vals::Pdiv {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pdiv::from_bits(val as u8)
    }
    #[doc = "pclk2_lpsys = hclk_lpsys / (2^PDIV2), by default divided by 32"]
    #[inline(always)]
    pub const fn set_pdiv2(&mut self, val: super::vals::Pdiv) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "MAC clock divider MACCLK = hclk_lpsys / MACDIV"]
    #[must_use]
    #[inline(always)]
    pub const fn macdiv(&self) -> super::vals::Macdiv {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Macdiv::from_bits(val as u8)
    }
    #[doc = "MAC clock divider MACCLK = hclk_lpsys / MACDIV"]
    #[inline(always)]
    pub const fn set_macdiv(&mut self, val: super::vals::Macdiv) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "clock frequency of MAC clock"]
    #[must_use]
    #[inline(always)]
    pub const fn macfreq(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "clock frequency of MAC clock"]
    #[inline(always)]
    pub const fn set_macfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[doc = "systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV"]
    #[must_use]
    #[inline(always)]
    pub const fn tickdiv(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x3f;
        val as u8
    }
    #[doc = "systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV"]
    #[inline(always)]
    pub const fn set_tickdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 25usize)) | (((val as u32) & 0x3f) << 25usize);
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
            .field("hdiv1", &self.hdiv1())
            .field("pdiv1", &self.pdiv1())
            .field("pdiv2", &self.pdiv2())
            .field("macdiv", &self.macdiv())
            .field("macfreq", &self.macfreq())
            .field("tickdiv", &self.tickdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfgr {{ hdiv1: {:?}, pdiv1: {:?}, pdiv2: {:?}, macdiv: {:?}, macfreq: {=u8:?}, tickdiv: {=u8:?} }}" , self . hdiv1 () , self . pdiv1 () , self . pdiv2 () , self . macdiv () , self . macfreq () , self . tickdiv ())
    }
}
#[doc = "Clock Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "select clk_lpsys source 0 - clk_hrc48; 1 - clk_hxt48"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_sys(&self) -> super::vals::Sysclk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sysclk::from_bits(val as u8)
    }
    #[doc = "select clk_lpsys source 0 - clk_hrc48; 1 - clk_hxt48"]
    #[inline(always)]
    pub const fn set_sel_sys(&mut self, val: super::vals::Sysclk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "select clk_lpsys source 0 - selected by SEL_SYS; 1 - clk_wdt"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_sys_lp(&self) -> super::vals::mux::Lpsel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::mux::Lpsel::from_bits(val as u8)
    }
    #[doc = "select clk_lpsys source 0 - selected by SEL_SYS; 1 - clk_wdt"]
    #[inline(always)]
    pub const fn set_sel_sys_lp(&mut self, val: super::vals::mux::Lpsel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "select clk_peri_lpsys source 0 - clk_hrc48; 1 - clk_hxt48"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_peri(&self) -> super::vals::mux::Perisel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::mux::Perisel::from_bits(val as u8)
    }
    #[doc = "select clk_peri_lpsys source 0 - clk_hrc48; 1 - clk_hxt48"]
    #[inline(always)]
    pub const fn set_sel_peri(&mut self, val: super::vals::mux::Perisel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_tick(&self) -> super::vals::mux::Ticksel {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::mux::Ticksel::from_bits(val as u8)
    }
    #[doc = "select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48"]
    #[inline(always)]
    pub const fn set_sel_tick(&mut self, val: super::vals::mux::Ticksel) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr")
            .field("sel_sys", &self.sel_sys())
            .field("sel_sys_lp", &self.sel_sys_lp())
            .field("sel_peri", &self.sel_peri())
            .field("sel_tick", &self.sel_tick())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ sel_sys: {:?}, sel_sys_lp: {:?}, sel_peri: {:?}, sel_tick: {:?} }}",
            self.sel_sys(),
            self.sel_sys_lp(),
            self.sel_peri(),
            self.sel_tick()
        )
    }
}
#[doc = "Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgr(pub u32);
impl Dbgr {
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn sysclk_aon(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_sysclk_aon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn sysclk_swlp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_sysclk_swlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn force_bus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_force_bus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn force_mac(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_force_mac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn force_gpio(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_force_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If set to 1, clk_lpsys will: switch from clk_hrc48 to clk_hxt48 when MAC active; switch from clk_hxt48 to clk_hrc48 when MAC sleep;"]
    #[must_use]
    #[inline(always)]
    pub const fn sysclk_swbt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, clk_lpsys will: switch from clk_hrc48 to clk_hxt48 when MAC active; switch from clk_hxt48 to clk_hrc48 when MAC sleep;"]
    #[inline(always)]
    pub const fn set_sysclk_swbt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Dbgr {
    #[inline(always)]
    fn default() -> Dbgr {
        Dbgr(0)
    }
}
impl core::fmt::Debug for Dbgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgr")
            .field("sysclk_aon", &self.sysclk_aon())
            .field("sysclk_swlp", &self.sysclk_swlp())
            .field("force_bus", &self.force_bus())
            .field("force_mac", &self.force_mac())
            .field("force_gpio", &self.force_gpio())
            .field("sysclk_swbt", &self.sysclk_swbt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dbgr {{ sysclk_aon: {=bool:?}, sysclk_swlp: {=bool:?}, force_bus: {=bool:?}, force_mac: {=bool:?}, force_gpio: {=bool:?}, sysclk_swbt: {=bool:?} }}" , self . sysclk_aon () , self . sysclk_swlp () , self . force_bus () , self . force_mac () , self . force_gpio () , self . sysclk_swbt ())
    }
}
#[doc = "Enable Clear Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr1(pub u32);
impl Ecr1 {
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_dmac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_mailbox2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn pinmux2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_pinmux2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn patch(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_patch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn usart4(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_usart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn usart5(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_usart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn secu2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_secu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn ptc2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_ptc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn btim3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_btim3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn btim4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_btim4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn syscfg2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_syscfg2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn rfc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_rfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn phy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn mac(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_mac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn crc2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_crc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ecr1 {
    #[inline(always)]
    fn default() -> Ecr1 {
        Ecr1(0)
    }
}
impl core::fmt::Debug for Ecr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecr1")
            .field("dmac2", &self.dmac2())
            .field("mailbox2", &self.mailbox2())
            .field("pinmux2", &self.pinmux2())
            .field("patch", &self.patch())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("secu2", &self.secu2())
            .field("ptc2", &self.ptc2())
            .field("btim3", &self.btim3())
            .field("btim4", &self.btim4())
            .field("syscfg2", &self.syscfg2())
            .field("gpio2", &self.gpio2())
            .field("rfc", &self.rfc())
            .field("phy", &self.phy())
            .field("mac", &self.mac())
            .field("crc2", &self.crc2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ecr1 {{ dmac2: {=bool:?}, mailbox2: {=bool:?}, pinmux2: {=bool:?}, patch: {=bool:?}, usart4: {=bool:?}, usart5: {=bool:?}, secu2: {=bool:?}, ptc2: {=bool:?}, btim3: {=bool:?}, btim4: {=bool:?}, syscfg2: {=bool:?}, gpio2: {=bool:?}, rfc: {=bool:?}, phy: {=bool:?}, mac: {=bool:?}, crc2: {=bool:?} }}" , self . dmac2 () , self . mailbox2 () , self . pinmux2 () , self . patch () , self . usart4 () , self . usart5 () , self . secu2 () , self . ptc2 () , self . btim3 () , self . btim4 () , self . syscfg2 () , self . gpio2 () , self . rfc () , self . phy () , self . mac () , self . crc2 ())
    }
}
#[doc = "Enable Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enr1(pub u32);
impl Enr1 {
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_dmac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_mailbox2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn pinmux2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_pinmux2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn patch(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_patch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn usart4(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_usart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn usart5(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_usart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn secu2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_secu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn ptc2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_ptc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn btim3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_btim3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn btim4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_btim4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn syscfg2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_syscfg2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rfc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_rfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn phy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn mac(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_mac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn crc2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 - disabled; 1 - enabled"]
    #[inline(always)]
    pub const fn set_crc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Enr1 {
    #[inline(always)]
    fn default() -> Enr1 {
        Enr1(0)
    }
}
impl core::fmt::Debug for Enr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enr1")
            .field("dmac2", &self.dmac2())
            .field("mailbox2", &self.mailbox2())
            .field("pinmux2", &self.pinmux2())
            .field("patch", &self.patch())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("secu2", &self.secu2())
            .field("ptc2", &self.ptc2())
            .field("btim3", &self.btim3())
            .field("btim4", &self.btim4())
            .field("syscfg2", &self.syscfg2())
            .field("gpio2", &self.gpio2())
            .field("rfc", &self.rfc())
            .field("phy", &self.phy())
            .field("mac", &self.mac())
            .field("crc2", &self.crc2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Enr1 {{ dmac2: {=bool:?}, mailbox2: {=bool:?}, pinmux2: {=bool:?}, patch: {=bool:?}, usart4: {=bool:?}, usart5: {=bool:?}, secu2: {=bool:?}, ptc2: {=bool:?}, btim3: {=bool:?}, btim4: {=bool:?}, syscfg2: {=bool:?}, gpio2: {=bool:?}, rfc: {=bool:?}, phy: {=bool:?}, mac: {=bool:?}, crc2: {=bool:?} }}" , self . dmac2 () , self . mailbox2 () , self . pinmux2 () , self . patch () , self . usart4 () , self . usart5 () , self . secu2 () , self . ptc2 () , self . btim3 () , self . btim4 () , self . syscfg2 () , self . gpio2 () , self . rfc () , self . phy () , self . mac () , self . crc2 ())
    }
}
#[doc = "Enable Set Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr1(pub u32);
impl Esr1 {
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_dmac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_mailbox2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn pinmux2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_pinmux2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn patch(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_patch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn usart4(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_usart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn usart5(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_usart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn secu2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_secu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn ptc2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_ptc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn btim3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_btim3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn btim4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_btim4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn syscfg2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_syscfg2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn rfc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_rfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn phy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn mac(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_mac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[must_use]
    #[inline(always)]
    pub const fn crc2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn set_crc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Esr1 {
    #[inline(always)]
    fn default() -> Esr1 {
        Esr1(0)
    }
}
impl core::fmt::Debug for Esr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Esr1")
            .field("dmac2", &self.dmac2())
            .field("mailbox2", &self.mailbox2())
            .field("pinmux2", &self.pinmux2())
            .field("patch", &self.patch())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("secu2", &self.secu2())
            .field("ptc2", &self.ptc2())
            .field("btim3", &self.btim3())
            .field("btim4", &self.btim4())
            .field("syscfg2", &self.syscfg2())
            .field("gpio2", &self.gpio2())
            .field("rfc", &self.rfc())
            .field("phy", &self.phy())
            .field("mac", &self.mac())
            .field("crc2", &self.crc2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Esr1 {{ dmac2: {=bool:?}, mailbox2: {=bool:?}, pinmux2: {=bool:?}, patch: {=bool:?}, usart4: {=bool:?}, usart5: {=bool:?}, secu2: {=bool:?}, ptc2: {=bool:?}, btim3: {=bool:?}, btim4: {=bool:?}, syscfg2: {=bool:?}, gpio2: {=bool:?}, rfc: {=bool:?}, phy: {=bool:?}, mac: {=bool:?}, crc2: {=bool:?} }}" , self . dmac2 () , self . mailbox2 () , self . pinmux2 () , self . patch () , self . usart4 () , self . usart5 () , self . secu2 () , self . ptc2 () , self . btim3 () , self . btim4 () , self . syscfg2 () , self . gpio2 () , self . rfc () , self . phy () , self . mac () , self . crc2 ())
    }
}
#[doc = "Reset Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstr1(pub u32);
impl Rstr1 {
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lcpu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_lcpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_dmac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_mailbox2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn pinmux2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_pinmux2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn patch(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_patch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn usart4(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_usart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn usart5(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_usart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ptc2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_ptc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn btim3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_btim3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn btim4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_btim4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn syscfg2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_syscfg2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rfc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_rfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn phy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn mac(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_mac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[must_use]
    #[inline(always)]
    pub const fn crc2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn set_crc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Rstr1 {
    #[inline(always)]
    fn default() -> Rstr1 {
        Rstr1(0)
    }
}
impl core::fmt::Debug for Rstr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rstr1")
            .field("lcpu", &self.lcpu())
            .field("dmac2", &self.dmac2())
            .field("mailbox2", &self.mailbox2())
            .field("pinmux2", &self.pinmux2())
            .field("patch", &self.patch())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("ptc2", &self.ptc2())
            .field("btim3", &self.btim3())
            .field("btim4", &self.btim4())
            .field("syscfg2", &self.syscfg2())
            .field("gpio2", &self.gpio2())
            .field("rfc", &self.rfc())
            .field("phy", &self.phy())
            .field("mac", &self.mac())
            .field("crc2", &self.crc2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Rstr1 {{ lcpu: {=bool:?}, dmac2: {=bool:?}, mailbox2: {=bool:?}, pinmux2: {=bool:?}, patch: {=bool:?}, usart4: {=bool:?}, usart5: {=bool:?}, ptc2: {=bool:?}, btim3: {=bool:?}, btim4: {=bool:?}, syscfg2: {=bool:?}, gpio2: {=bool:?}, rfc: {=bool:?}, phy: {=bool:?}, mac: {=bool:?}, crc2: {=bool:?} }}" , self . lcpu () , self . dmac2 () , self . mailbox2 () , self . pinmux2 () , self . patch () , self . usart4 () , self . usart5 () , self . ptc2 () , self . btim3 () , self . btim4 () , self . syscfg2 () , self . gpio2 () , self . rfc () , self . phy () , self . mac () , self . crc2 ())
    }
}
