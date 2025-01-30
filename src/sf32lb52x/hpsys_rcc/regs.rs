#[doc = "Clock Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "hclk_hpsys = clk_hpsys / HDIV if HDIV=0, hclk_hpsys = clk_hpsys"]
    #[inline(always)]
    pub const fn hdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "hclk_hpsys = clk_hpsys / HDIV if HDIV=0, hclk_hpsys = clk_hpsys"]
    #[inline(always)]
    pub fn set_hdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "pclk_hpsys = hclk_hpsys / (2^PDIV1), by default divided by 2"]
    #[inline(always)]
    pub const fn pdiv1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "pclk_hpsys = hclk_hpsys / (2^PDIV1), by default divided by 2"]
    #[inline(always)]
    pub fn set_pdiv1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "pclk2_hpsys = hclk_hpsys / (2^PDIV2), by default divided by 16"]
    #[inline(always)]
    pub const fn pdiv2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "pclk2_hpsys = hclk_hpsys / (2^PDIV2), by default divided by 16"]
    #[inline(always)]
    pub fn set_pdiv2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV"]
    #[inline(always)]
    pub const fn tickdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "systick reference clock is systick reference clock source (selected by SEL_TICK) devided by TICKDIV"]
    #[inline(always)]
    pub fn set_tickdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
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
            .field("hdiv", &self.hdiv())
            .field("pdiv1", &self.pdiv1())
            .field("pdiv2", &self.pdiv2())
            .field("tickdiv", &self.tickdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cfgr {
            hdiv: u8,
            pdiv1: u8,
            pdiv2: u8,
            tickdiv: u8,
        }
        let proxy = Cfgr {
            hdiv: self.hdiv(),
            pdiv1: self.pdiv1(),
            pdiv2: self.pdiv2(),
            tickdiv: self.tickdiv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "select clk_hpsys source 0 - clk_hrc48; 1 - clk_hxt48; 2 - reserved; 3 - clk_dll1"]
    #[inline(always)]
    pub const fn sel_sys(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "select clk_hpsys source 0 - clk_hrc48; 1 - clk_hxt48; 2 - reserved; 3 - clk_dll1"]
    #[inline(always)]
    pub fn set_sel_sys(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "select clk_hpsys source 0 - selected by SEL_SYS; 1 - clk_wdt"]
    #[inline(always)]
    pub const fn sel_sys_lp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "select clk_hpsys source 0 - selected by SEL_SYS; 1 - clk_wdt"]
    #[inline(always)]
    pub fn set_sel_sys_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "selet MPI1 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved"]
    #[inline(always)]
    pub const fn sel_mpi1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "selet MPI1 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved"]
    #[inline(always)]
    pub fn set_sel_mpi1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "selet MPI2 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved"]
    #[inline(always)]
    pub const fn sel_mpi2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "selet MPI2 function clock 0 - clk_peri_hpsys; 1 - clk_dll1; 2 - clk_dll2; 3 - reserved"]
    #[inline(always)]
    pub fn set_sel_mpi2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "select clk_peri_hpsys source used by USART/SPI/I2C/GPTIM2/BTIM2 0 - clk_hrc48; 1 - clk_hxt48"]
    #[inline(always)]
    pub const fn sel_peri(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "select clk_peri_hpsys source used by USART/SPI/I2C/GPTIM2/BTIM2 0 - clk_hrc48; 1 - clk_hxt48"]
    #[inline(always)]
    pub fn set_sel_peri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48"]
    #[inline(always)]
    pub const fn sel_tick(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "select clock source for systick reference 0 - clk_rtc; 1 - reserved; 2 - clk_hrc48; 3 - clk_hxt48"]
    #[inline(always)]
    pub fn set_sel_tick(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "select USB source clock 0 - clk_hpsys; 1 - clk_dll2"]
    #[inline(always)]
    pub const fn sel_usbc(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "select USB source clock 0 - clk_hpsys; 1 - clk_dll2"]
    #[inline(always)]
    pub fn set_sel_usbc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("sel_mpi1", &self.sel_mpi1())
            .field("sel_mpi2", &self.sel_mpi2())
            .field("sel_peri", &self.sel_peri())
            .field("sel_tick", &self.sel_tick())
            .field("sel_usbc", &self.sel_usbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Csr {
            sel_sys: u8,
            sel_sys_lp: bool,
            sel_mpi1: u8,
            sel_mpi2: u8,
            sel_peri: bool,
            sel_tick: u8,
            sel_usbc: bool,
        }
        let proxy = Csr {
            sel_sys: self.sel_sys(),
            sel_sys_lp: self.sel_sys_lp(),
            sel_mpi1: self.sel_mpi1(),
            sel_mpi2: self.sel_mpi2(),
            sel_peri: self.sel_peri(),
            sel_tick: self.sel_tick(),
            sel_usbc: self.sel_usbc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Debug Clock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgclkr(pub u32);
impl Dbgclkr {
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn clk_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_dbg(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_ldo_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_ldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_out_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_loop_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_loop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_out_rstb(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_out_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_cg_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_cg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_out_str(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_out_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_dbg(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_ldo_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_ldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_out_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_loop_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_loop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_out_rstb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_out_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_cg_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_cg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_out_str(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_out_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for Dbgclkr {
    #[inline(always)]
    fn default() -> Dbgclkr {
        Dbgclkr(0)
    }
}
impl core::fmt::Debug for Dbgclkr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgclkr")
            .field("clk_sel", &self.clk_sel())
            .field("clk_en", &self.clk_en())
            .field("dll1_dbg", &self.dll1_dbg())
            .field("dll1_ldo_en", &self.dll1_ldo_en())
            .field("dll1_out_en", &self.dll1_out_en())
            .field("dll1_loop_en", &self.dll1_loop_en())
            .field("dll1_out_rstb", &self.dll1_out_rstb())
            .field("dll1_cg_en", &self.dll1_cg_en())
            .field("dll1_out_str", &self.dll1_out_str())
            .field("dll2_dbg", &self.dll2_dbg())
            .field("dll2_ldo_en", &self.dll2_ldo_en())
            .field("dll2_out_en", &self.dll2_out_en())
            .field("dll2_loop_en", &self.dll2_loop_en())
            .field("dll2_out_rstb", &self.dll2_out_rstb())
            .field("dll2_cg_en", &self.dll2_cg_en())
            .field("dll2_out_str", &self.dll2_out_str())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgclkr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbgclkr {
            clk_sel: u8,
            clk_en: bool,
            dll1_dbg: bool,
            dll1_ldo_en: bool,
            dll1_out_en: bool,
            dll1_loop_en: bool,
            dll1_out_rstb: bool,
            dll1_cg_en: bool,
            dll1_out_str: u8,
            dll2_dbg: bool,
            dll2_ldo_en: bool,
            dll2_out_en: bool,
            dll2_loop_en: bool,
            dll2_out_rstb: bool,
            dll2_cg_en: bool,
            dll2_out_str: u8,
        }
        let proxy = Dbgclkr {
            clk_sel: self.clk_sel(),
            clk_en: self.clk_en(),
            dll1_dbg: self.dll1_dbg(),
            dll1_ldo_en: self.dll1_ldo_en(),
            dll1_out_en: self.dll1_out_en(),
            dll1_loop_en: self.dll1_loop_en(),
            dll1_out_rstb: self.dll1_out_rstb(),
            dll1_cg_en: self.dll1_cg_en(),
            dll1_out_str: self.dll1_out_str(),
            dll2_dbg: self.dll2_dbg(),
            dll2_ldo_en: self.dll2_ldo_en(),
            dll2_out_en: self.dll2_out_en(),
            dll2_loop_en: self.dll2_loop_en(),
            dll2_out_rstb: self.dll2_out_rstb(),
            dll2_cg_en: self.dll2_cg_en(),
            dll2_out_str: self.dll2_out_str(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgr(pub u32);
impl Dbgr {
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn sysclk_aon(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_sysclk_aon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn sysclk_swlp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_sysclk_swlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn force_bus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_force_bus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn force_gpio(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_force_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn force_hp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_force_hp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("force_gpio", &self.force_gpio())
            .field("force_hp", &self.force_hp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbgr {
            sysclk_aon: bool,
            sysclk_swlp: bool,
            force_bus: bool,
            force_gpio: bool,
            force_hp: bool,
        }
        let proxy = Dbgr {
            sysclk_aon: self.sysclk_aon(),
            sysclk_swlp: self.sysclk_swlp(),
            force_bus: self.force_bus(),
            force_gpio: self.force_gpio(),
            force_hp: self.force_hp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DLL1 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dll1cr(pub u32);
impl Dll1cr {
    #[doc = "0: dll disabled 1: dll enabled"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: dll disabled 1: dll enabled"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DLL lock freqency is decided by STG. DLL output frequency is (STG+1)*24MHz e.g. STG=9,DLL output is 240M"]
    #[inline(always)]
    pub const fn stg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL lock freqency is decided by STG. DLL output frequency is (STG+1)*24MHz e.g. STG=9,DLL output is 240M"]
    #[inline(always)]
    pub fn set_stg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[inline(always)]
    pub const fn xtalin_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xtalin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn mode48m_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_mode48m_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn ldo_vref(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_ldo_vref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[inline(always)]
    pub const fn in_div2_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_in_div2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0: dll output not divided 1: dll output divided by 2"]
    #[inline(always)]
    pub const fn out_div2_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0: dll output not divided 1: dll output divided by 2"]
    #[inline(always)]
    pub fn set_out_div2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn mcu_prchg_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_mcu_prchg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn mcu_prchg(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_mcu_prchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn prchg_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prchg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn prchg_ext(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prchg_ext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn vst_sel(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vst_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn dtest_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dtest_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn dtest_tr(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_dtest_tr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
    #[inline(always)]
    pub const fn pu_dly(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pu_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[inline(always)]
    pub const fn lock_dly(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_lock_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "0: dll not ready 1: dll ready"]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0: dll not ready 1: dll ready"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dll1cr {
    #[inline(always)]
    fn default() -> Dll1cr {
        Dll1cr(0)
    }
}
impl core::fmt::Debug for Dll1cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dll1cr")
            .field("en", &self.en())
            .field("sw", &self.sw())
            .field("stg", &self.stg())
            .field("xtalin_en", &self.xtalin_en())
            .field("mode48m_en", &self.mode48m_en())
            .field("ldo_vref", &self.ldo_vref())
            .field("in_div2_en", &self.in_div2_en())
            .field("out_div2_en", &self.out_div2_en())
            .field("mcu_prchg_en", &self.mcu_prchg_en())
            .field("mcu_prchg", &self.mcu_prchg())
            .field("prchg_en", &self.prchg_en())
            .field("prchg_ext", &self.prchg_ext())
            .field("vst_sel", &self.vst_sel())
            .field("bypass", &self.bypass())
            .field("dtest_en", &self.dtest_en())
            .field("dtest_tr", &self.dtest_tr())
            .field("pu_dly", &self.pu_dly())
            .field("lock_dly", &self.lock_dly())
            .field("ready", &self.ready())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dll1cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dll1cr {
            en: bool,
            sw: bool,
            stg: u8,
            xtalin_en: bool,
            mode48m_en: bool,
            ldo_vref: u8,
            in_div2_en: bool,
            out_div2_en: bool,
            mcu_prchg_en: bool,
            mcu_prchg: bool,
            prchg_en: bool,
            prchg_ext: bool,
            vst_sel: bool,
            bypass: bool,
            dtest_en: bool,
            dtest_tr: u8,
            pu_dly: u8,
            lock_dly: u8,
            ready: bool,
        }
        let proxy = Dll1cr {
            en: self.en(),
            sw: self.sw(),
            stg: self.stg(),
            xtalin_en: self.xtalin_en(),
            mode48m_en: self.mode48m_en(),
            ldo_vref: self.ldo_vref(),
            in_div2_en: self.in_div2_en(),
            out_div2_en: self.out_div2_en(),
            mcu_prchg_en: self.mcu_prchg_en(),
            mcu_prchg: self.mcu_prchg(),
            prchg_en: self.prchg_en(),
            prchg_ext: self.prchg_ext(),
            vst_sel: self.vst_sel(),
            bypass: self.bypass(),
            dtest_en: self.dtest_en(),
            dtest_tr: self.dtest_tr(),
            pu_dly: self.pu_dly(),
            lock_dly: self.lock_dly(),
            ready: self.ready(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DLL2 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dll2cr(pub u32);
impl Dll2cr {
    #[doc = "0: dll disabled 1: dll enabled"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: dll disabled 1: dll enabled"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DLL lock freqency is decided by STG. DLL output frequency is (STG+1)*24MHz e.g. STG=9,DLL output is 240M"]
    #[inline(always)]
    pub const fn stg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL lock freqency is decided by STG. DLL output frequency is (STG+1)*24MHz e.g. STG=9,DLL output is 240M"]
    #[inline(always)]
    pub fn set_stg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[inline(always)]
    pub const fn xtalin_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xtalin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn mode48m_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_mode48m_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn ldo_vref(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_ldo_vref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[inline(always)]
    pub const fn in_div2_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_in_div2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0: dll output not divided 1: dll output divided by 2"]
    #[inline(always)]
    pub const fn out_div2_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0: dll output not divided 1: dll output divided by 2"]
    #[inline(always)]
    pub fn set_out_div2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn mcu_prchg_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_mcu_prchg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn mcu_prchg(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_mcu_prchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn prchg_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prchg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn prchg_ext(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prchg_ext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn vst_sel(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vst_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn dtest_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dtest_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn dtest_tr(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_dtest_tr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
    #[inline(always)]
    pub const fn pu_dly(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pu_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[inline(always)]
    pub const fn lock_dly(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_lock_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "0: dll not ready 1: dll ready"]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0: dll not ready 1: dll ready"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dll2cr {
    #[inline(always)]
    fn default() -> Dll2cr {
        Dll2cr(0)
    }
}
impl core::fmt::Debug for Dll2cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dll2cr")
            .field("en", &self.en())
            .field("sw", &self.sw())
            .field("stg", &self.stg())
            .field("xtalin_en", &self.xtalin_en())
            .field("mode48m_en", &self.mode48m_en())
            .field("ldo_vref", &self.ldo_vref())
            .field("in_div2_en", &self.in_div2_en())
            .field("out_div2_en", &self.out_div2_en())
            .field("mcu_prchg_en", &self.mcu_prchg_en())
            .field("mcu_prchg", &self.mcu_prchg())
            .field("prchg_en", &self.prchg_en())
            .field("prchg_ext", &self.prchg_ext())
            .field("vst_sel", &self.vst_sel())
            .field("bypass", &self.bypass())
            .field("dtest_en", &self.dtest_en())
            .field("dtest_tr", &self.dtest_tr())
            .field("pu_dly", &self.pu_dly())
            .field("lock_dly", &self.lock_dly())
            .field("ready", &self.ready())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dll2cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dll2cr {
            en: bool,
            sw: bool,
            stg: u8,
            xtalin_en: bool,
            mode48m_en: bool,
            ldo_vref: u8,
            in_div2_en: bool,
            out_div2_en: bool,
            mcu_prchg_en: bool,
            mcu_prchg: bool,
            prchg_en: bool,
            prchg_ext: bool,
            vst_sel: bool,
            bypass: bool,
            dtest_en: bool,
            dtest_tr: u8,
            pu_dly: u8,
            lock_dly: u8,
            ready: bool,
        }
        let proxy = Dll2cr {
            en: self.en(),
            sw: self.sw(),
            stg: self.stg(),
            xtalin_en: self.xtalin_en(),
            mode48m_en: self.mode48m_en(),
            ldo_vref: self.ldo_vref(),
            in_div2_en: self.in_div2_en(),
            out_div2_en: self.out_div2_en(),
            mcu_prchg_en: self.mcu_prchg_en(),
            mcu_prchg: self.mcu_prchg(),
            prchg_en: self.prchg_en(),
            prchg_ext: self.prchg_ext(),
            vst_sel: self.vst_sel(),
            bypass: self.bypass(),
            dtest_en: self.dtest_en(),
            dtest_tr: self.dtest_tr(),
            pu_dly: self.pu_dly(),
            lock_dly: self.lock_dly(),
            ready: self.ready(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Deep WFI mode Clock Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dwcfgr(pub u32);
impl Dwcfgr {
    #[doc = "hclk_hpsys = clk_hpsys / HDIV during deep wfi"]
    #[inline(always)]
    pub const fn hdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "hclk_hpsys = clk_hpsys / HDIV during deep wfi"]
    #[inline(always)]
    pub fn set_hdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "pclk_hpsys = hclk_hpsys / (2^PDIV1) during deep wfi"]
    #[inline(always)]
    pub const fn pdiv1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "pclk_hpsys = hclk_hpsys / (2^PDIV1) during deep wfi"]
    #[inline(always)]
    pub fn set_pdiv1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "pclk2_hpsys = hclk_hpsys / (2^PDIV2) during deep wfi"]
    #[inline(always)]
    pub const fn pdiv2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "pclk2_hpsys = hclk_hpsys / (2^PDIV2) during deep wfi"]
    #[inline(always)]
    pub fn set_pdiv2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "enable PDIV1, PDIV2 and HDIV reconfiguration during deep wfi"]
    #[inline(always)]
    pub const fn div_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "enable PDIV1, PDIV2 and HDIV reconfiguration during deep wfi"]
    #[inline(always)]
    pub fn set_div_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "select clk_hpsys source during deep WFI 0 - clk_hrc48; 1 - clk_hxt48; 2 - RSVD; 3 - clk_dll1"]
    #[inline(always)]
    pub const fn sel_sys(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "select clk_hpsys source during deep WFI 0 - clk_hrc48; 1 - clk_hxt48; 2 - RSVD; 3 - clk_dll1"]
    #[inline(always)]
    pub fn set_sel_sys(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "select clk_hpsys source during deep WFI 0 - selected by SEL_SYS; 1 - clk_wdt"]
    #[inline(always)]
    pub const fn sel_sys_lp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "select clk_hpsys source during deep WFI 0 - selected by SEL_SYS; 1 - clk_wdt"]
    #[inline(always)]
    pub fn set_sel_sys_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_out_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll1_out_rstb(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll1_out_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_out_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn dll2_out_rstb(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub fn set_dll2_out_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Dwcfgr {
    #[inline(always)]
    fn default() -> Dwcfgr {
        Dwcfgr(0)
    }
}
impl core::fmt::Debug for Dwcfgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dwcfgr")
            .field("hdiv", &self.hdiv())
            .field("pdiv1", &self.pdiv1())
            .field("pdiv2", &self.pdiv2())
            .field("div_en", &self.div_en())
            .field("sel_sys", &self.sel_sys())
            .field("sel_sys_lp", &self.sel_sys_lp())
            .field("dll1_out_en", &self.dll1_out_en())
            .field("dll1_out_rstb", &self.dll1_out_rstb())
            .field("dll2_out_en", &self.dll2_out_en())
            .field("dll2_out_rstb", &self.dll2_out_rstb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dwcfgr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dwcfgr {
            hdiv: u8,
            pdiv1: u8,
            pdiv2: u8,
            div_en: bool,
            sel_sys: u8,
            sel_sys_lp: bool,
            dll1_out_en: bool,
            dll1_out_rstb: bool,
            dll2_out_en: bool,
            dll2_out_rstb: bool,
        }
        let proxy = Dwcfgr {
            hdiv: self.hdiv(),
            pdiv1: self.pdiv1(),
            pdiv2: self.pdiv2(),
            div_en: self.div_en(),
            sel_sys: self.sel_sys(),
            sel_sys_lp: self.sel_sys_lp(),
            dll1_out_en: self.dll1_out_en(),
            dll1_out_rstb: self.dll1_out_rstb(),
            dll2_out_en: self.dll2_out_en(),
            dll2_out_rstb: self.dll2_out_rstb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable Clear Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr1(pub u32);
impl Ecr1 {
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn dmac1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_dmac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn mailbox1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_mailbox1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn pinmux1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_pinmux1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn usart2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_usart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn ezip1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_ezip1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn epic(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_epic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn lcdc1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_lcdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2s1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn syscfg1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_syscfg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn efusec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_efusec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn aes(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_aes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn crc1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_crc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn gptim1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_gptim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn gptim2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_gptim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn btim1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_btim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn btim2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_btim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn spi2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_spi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn extdma(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_extdma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn secu1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_secu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn pdm1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_pdm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2c2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn ptc1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_ptc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("dmac1", &self.dmac1())
            .field("mailbox1", &self.mailbox1())
            .field("pinmux1", &self.pinmux1())
            .field("usart2", &self.usart2())
            .field("ezip1", &self.ezip1())
            .field("epic", &self.epic())
            .field("lcdc1", &self.lcdc1())
            .field("i2s1", &self.i2s1())
            .field("syscfg1", &self.syscfg1())
            .field("efusec", &self.efusec())
            .field("aes", &self.aes())
            .field("crc1", &self.crc1())
            .field("trng", &self.trng())
            .field("gptim1", &self.gptim1())
            .field("gptim2", &self.gptim2())
            .field("btim1", &self.btim1())
            .field("btim2", &self.btim2())
            .field("spi1", &self.spi1())
            .field("spi2", &self.spi2())
            .field("extdma", &self.extdma())
            .field("secu1", &self.secu1())
            .field("pdm1", &self.pdm1())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("ptc1", &self.ptc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ecr1 {
            dmac1: bool,
            mailbox1: bool,
            pinmux1: bool,
            usart2: bool,
            ezip1: bool,
            epic: bool,
            lcdc1: bool,
            i2s1: bool,
            syscfg1: bool,
            efusec: bool,
            aes: bool,
            crc1: bool,
            trng: bool,
            gptim1: bool,
            gptim2: bool,
            btim1: bool,
            btim2: bool,
            spi1: bool,
            spi2: bool,
            extdma: bool,
            secu1: bool,
            pdm1: bool,
            i2c1: bool,
            i2c2: bool,
            ptc1: bool,
        }
        let proxy = Ecr1 {
            dmac1: self.dmac1(),
            mailbox1: self.mailbox1(),
            pinmux1: self.pinmux1(),
            usart2: self.usart2(),
            ezip1: self.ezip1(),
            epic: self.epic(),
            lcdc1: self.lcdc1(),
            i2s1: self.i2s1(),
            syscfg1: self.syscfg1(),
            efusec: self.efusec(),
            aes: self.aes(),
            crc1: self.crc1(),
            trng: self.trng(),
            gptim1: self.gptim1(),
            gptim2: self.gptim2(),
            btim1: self.btim1(),
            btim2: self.btim2(),
            spi1: self.spi1(),
            spi2: self.spi2(),
            extdma: self.extdma(),
            secu1: self.secu1(),
            pdm1: self.pdm1(),
            i2c1: self.i2c1(),
            i2c2: self.i2c2(),
            ptc1: self.ptc1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable Clear Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr2(pub u32);
impl Ecr2 {
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn mpi1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_mpi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn mpi2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_mpi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn sdmmc1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_sdmmc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn usbc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_usbc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2c3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn atim1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_atim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn usart3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_usart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn audcodec(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_audcodec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn audprc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_audprc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn gpadc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_gpadc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn tsen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_tsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2c4(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to clear module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2c4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Ecr2 {
    #[inline(always)]
    fn default() -> Ecr2 {
        Ecr2(0)
    }
}
impl core::fmt::Debug for Ecr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecr2")
            .field("gpio1", &self.gpio1())
            .field("mpi1", &self.mpi1())
            .field("mpi2", &self.mpi2())
            .field("sdmmc1", &self.sdmmc1())
            .field("usbc", &self.usbc())
            .field("i2c3", &self.i2c3())
            .field("atim1", &self.atim1())
            .field("usart3", &self.usart3())
            .field("audcodec", &self.audcodec())
            .field("audprc", &self.audprc())
            .field("gpadc", &self.gpadc())
            .field("tsen", &self.tsen())
            .field("i2c4", &self.i2c4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ecr2 {
            gpio1: bool,
            mpi1: bool,
            mpi2: bool,
            sdmmc1: bool,
            usbc: bool,
            i2c3: bool,
            atim1: bool,
            usart3: bool,
            audcodec: bool,
            audprc: bool,
            gpadc: bool,
            tsen: bool,
            i2c4: bool,
        }
        let proxy = Ecr2 {
            gpio1: self.gpio1(),
            mpi1: self.mpi1(),
            mpi2: self.mpi2(),
            sdmmc1: self.sdmmc1(),
            usbc: self.usbc(),
            i2c3: self.i2c3(),
            atim1: self.atim1(),
            usart3: self.usart3(),
            audcodec: self.audcodec(),
            audprc: self.audprc(),
            gpadc: self.gpadc(),
            tsen: self.tsen(),
            i2c4: self.i2c4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enr1(pub u32);
impl Enr1 {
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn dmac1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_dmac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn mailbox1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_mailbox1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn pinmux1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_pinmux1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn usart1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_usart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn usart2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_usart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn ezip1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_ezip1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn epic(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_epic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn lcdc1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_lcdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn i2s1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_i2s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn syscfg1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_syscfg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn efusec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_efusec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn aes(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_aes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn crc1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_crc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn gptim1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_gptim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn gptim2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_gptim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn btim1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_btim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn btim2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_btim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn wdt1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_wdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn spi2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_spi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn extdma(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_extdma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn secu1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_secu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn pdm1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_pdm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn i2c2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_i2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn ptc1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_ptc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("dmac1", &self.dmac1())
            .field("mailbox1", &self.mailbox1())
            .field("pinmux1", &self.pinmux1())
            .field("usart1", &self.usart1())
            .field("usart2", &self.usart2())
            .field("ezip1", &self.ezip1())
            .field("epic", &self.epic())
            .field("lcdc1", &self.lcdc1())
            .field("i2s1", &self.i2s1())
            .field("syscfg1", &self.syscfg1())
            .field("efusec", &self.efusec())
            .field("aes", &self.aes())
            .field("crc1", &self.crc1())
            .field("trng", &self.trng())
            .field("gptim1", &self.gptim1())
            .field("gptim2", &self.gptim2())
            .field("btim1", &self.btim1())
            .field("btim2", &self.btim2())
            .field("wdt1", &self.wdt1())
            .field("spi1", &self.spi1())
            .field("spi2", &self.spi2())
            .field("extdma", &self.extdma())
            .field("secu1", &self.secu1())
            .field("pdm1", &self.pdm1())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("ptc1", &self.ptc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Enr1 {
            dmac1: bool,
            mailbox1: bool,
            pinmux1: bool,
            usart1: bool,
            usart2: bool,
            ezip1: bool,
            epic: bool,
            lcdc1: bool,
            i2s1: bool,
            syscfg1: bool,
            efusec: bool,
            aes: bool,
            crc1: bool,
            trng: bool,
            gptim1: bool,
            gptim2: bool,
            btim1: bool,
            btim2: bool,
            wdt1: bool,
            spi1: bool,
            spi2: bool,
            extdma: bool,
            secu1: bool,
            pdm1: bool,
            i2c1: bool,
            i2c2: bool,
            ptc1: bool,
        }
        let proxy = Enr1 {
            dmac1: self.dmac1(),
            mailbox1: self.mailbox1(),
            pinmux1: self.pinmux1(),
            usart1: self.usart1(),
            usart2: self.usart2(),
            ezip1: self.ezip1(),
            epic: self.epic(),
            lcdc1: self.lcdc1(),
            i2s1: self.i2s1(),
            syscfg1: self.syscfg1(),
            efusec: self.efusec(),
            aes: self.aes(),
            crc1: self.crc1(),
            trng: self.trng(),
            gptim1: self.gptim1(),
            gptim2: self.gptim2(),
            btim1: self.btim1(),
            btim2: self.btim2(),
            wdt1: self.wdt1(),
            spi1: self.spi1(),
            spi2: self.spi2(),
            extdma: self.extdma(),
            secu1: self.secu1(),
            pdm1: self.pdm1(),
            i2c1: self.i2c1(),
            i2c2: self.i2c2(),
            ptc1: self.ptc1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enr2(pub u32);
impl Enr2 {
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn mpi1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_mpi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn mpi2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_mpi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn sdmmc1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_sdmmc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn usbc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_usbc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn i2c3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_i2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn atim1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_atim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn usart3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_usart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn audcodec(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_audcodec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn audprc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_audprc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn gpadc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_gpadc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn tsen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_tsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub const fn i2c4(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 to disable module"]
    #[inline(always)]
    pub fn set_i2c4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Enr2 {
    #[inline(always)]
    fn default() -> Enr2 {
        Enr2(0)
    }
}
impl core::fmt::Debug for Enr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enr2")
            .field("gpio1", &self.gpio1())
            .field("mpi1", &self.mpi1())
            .field("mpi2", &self.mpi2())
            .field("sdmmc1", &self.sdmmc1())
            .field("usbc", &self.usbc())
            .field("i2c3", &self.i2c3())
            .field("atim1", &self.atim1())
            .field("usart3", &self.usart3())
            .field("audcodec", &self.audcodec())
            .field("audprc", &self.audprc())
            .field("gpadc", &self.gpadc())
            .field("tsen", &self.tsen())
            .field("i2c4", &self.i2c4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Enr2 {
            gpio1: bool,
            mpi1: bool,
            mpi2: bool,
            sdmmc1: bool,
            usbc: bool,
            i2c3: bool,
            atim1: bool,
            usart3: bool,
            audcodec: bool,
            audprc: bool,
            gpadc: bool,
            tsen: bool,
            i2c4: bool,
        }
        let proxy = Enr2 {
            gpio1: self.gpio1(),
            mpi1: self.mpi1(),
            mpi2: self.mpi2(),
            sdmmc1: self.sdmmc1(),
            usbc: self.usbc(),
            i2c3: self.i2c3(),
            atim1: self.atim1(),
            usart3: self.usart3(),
            audcodec: self.audcodec(),
            audprc: self.audprc(),
            gpadc: self.gpadc(),
            tsen: self.tsen(),
            i2c4: self.i2c4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable Set Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr1(pub u32);
impl Esr1 {
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn dmac1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_dmac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn mailbox1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_mailbox1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn pinmux1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_pinmux1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn usart2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_usart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn ezip1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_ezip1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn epic(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_epic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn lcdc1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_lcdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2s1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn syscfg1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_syscfg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn efusec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_efusec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn aes(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_aes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn crc1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_crc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn gptim1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_gptim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn gptim2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_gptim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn btim1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_btim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn btim2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_btim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn spi2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_spi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn extdma(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_extdma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn secu1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_secu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn pdm1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_pdm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2c2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn ptc1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_ptc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("dmac1", &self.dmac1())
            .field("mailbox1", &self.mailbox1())
            .field("pinmux1", &self.pinmux1())
            .field("usart2", &self.usart2())
            .field("ezip1", &self.ezip1())
            .field("epic", &self.epic())
            .field("lcdc1", &self.lcdc1())
            .field("i2s1", &self.i2s1())
            .field("syscfg1", &self.syscfg1())
            .field("efusec", &self.efusec())
            .field("aes", &self.aes())
            .field("crc1", &self.crc1())
            .field("trng", &self.trng())
            .field("gptim1", &self.gptim1())
            .field("gptim2", &self.gptim2())
            .field("btim1", &self.btim1())
            .field("btim2", &self.btim2())
            .field("spi1", &self.spi1())
            .field("spi2", &self.spi2())
            .field("extdma", &self.extdma())
            .field("secu1", &self.secu1())
            .field("pdm1", &self.pdm1())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("ptc1", &self.ptc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Esr1 {
            dmac1: bool,
            mailbox1: bool,
            pinmux1: bool,
            usart2: bool,
            ezip1: bool,
            epic: bool,
            lcdc1: bool,
            i2s1: bool,
            syscfg1: bool,
            efusec: bool,
            aes: bool,
            crc1: bool,
            trng: bool,
            gptim1: bool,
            gptim2: bool,
            btim1: bool,
            btim2: bool,
            spi1: bool,
            spi2: bool,
            extdma: bool,
            secu1: bool,
            pdm1: bool,
            i2c1: bool,
            i2c2: bool,
            ptc1: bool,
        }
        let proxy = Esr1 {
            dmac1: self.dmac1(),
            mailbox1: self.mailbox1(),
            pinmux1: self.pinmux1(),
            usart2: self.usart2(),
            ezip1: self.ezip1(),
            epic: self.epic(),
            lcdc1: self.lcdc1(),
            i2s1: self.i2s1(),
            syscfg1: self.syscfg1(),
            efusec: self.efusec(),
            aes: self.aes(),
            crc1: self.crc1(),
            trng: self.trng(),
            gptim1: self.gptim1(),
            gptim2: self.gptim2(),
            btim1: self.btim1(),
            btim2: self.btim2(),
            spi1: self.spi1(),
            spi2: self.spi2(),
            extdma: self.extdma(),
            secu1: self.secu1(),
            pdm1: self.pdm1(),
            i2c1: self.i2c1(),
            i2c2: self.i2c2(),
            ptc1: self.ptc1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable Set Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr2(pub u32);
impl Esr2 {
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn mpi1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_mpi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn mpi2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_mpi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn sdmmc1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_sdmmc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn usbc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_usbc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2c3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn atim1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_atim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn usart3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_usart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn audcodec(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_audcodec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn audprc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_audprc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn gpadc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_gpadc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn tsen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_tsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub const fn i2c4(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to set module enable, write 0 has no effect"]
    #[inline(always)]
    pub fn set_i2c4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Esr2 {
    #[inline(always)]
    fn default() -> Esr2 {
        Esr2(0)
    }
}
impl core::fmt::Debug for Esr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Esr2")
            .field("gpio1", &self.gpio1())
            .field("mpi1", &self.mpi1())
            .field("mpi2", &self.mpi2())
            .field("sdmmc1", &self.sdmmc1())
            .field("usbc", &self.usbc())
            .field("i2c3", &self.i2c3())
            .field("atim1", &self.atim1())
            .field("usart3", &self.usart3())
            .field("audcodec", &self.audcodec())
            .field("audprc", &self.audprc())
            .field("gpadc", &self.gpadc())
            .field("tsen", &self.tsen())
            .field("i2c4", &self.i2c4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Esr2 {
            gpio1: bool,
            mpi1: bool,
            mpi2: bool,
            sdmmc1: bool,
            usbc: bool,
            i2c3: bool,
            atim1: bool,
            usart3: bool,
            audcodec: bool,
            audprc: bool,
            gpadc: bool,
            tsen: bool,
            i2c4: bool,
        }
        let proxy = Esr2 {
            gpio1: self.gpio1(),
            mpi1: self.mpi1(),
            mpi2: self.mpi2(),
            sdmmc1: self.sdmmc1(),
            usbc: self.usbc(),
            i2c3: self.i2c3(),
            atim1: self.atim1(),
            usart3: self.usart3(),
            audcodec: self.audcodec(),
            audprc: self.audprc(),
            gpadc: self.gpadc(),
            tsen: self.tsen(),
            i2c4: self.i2c4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HRC Calibration Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrccal1(pub u32);
impl Hrccal1 {
    #[doc = "Target clk_hxt48 cycles during calibration"]
    #[inline(always)]
    pub const fn cal_length(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Target clk_hxt48 cycles during calibration"]
    #[inline(always)]
    pub fn set_cal_length(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Calibration enble. Set to 0 to clear result, then set to 1 to start a new calibration"]
    #[inline(always)]
    pub const fn cal_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration enble. Set to 0 to clear result, then set to 1 to start a new calibration"]
    #[inline(always)]
    pub fn set_cal_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Calibration done. After a new calibration started, results should be processed only when cal_done asserted."]
    #[inline(always)]
    pub const fn cal_done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration done. After a new calibration started, results should be processed only when cal_done asserted."]
    #[inline(always)]
    pub fn set_cal_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hrccal1 {
    #[inline(always)]
    fn default() -> Hrccal1 {
        Hrccal1(0)
    }
}
impl core::fmt::Debug for Hrccal1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hrccal1")
            .field("cal_length", &self.cal_length())
            .field("cal_en", &self.cal_en())
            .field("cal_done", &self.cal_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hrccal1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hrccal1 {
            cal_length: u16,
            cal_en: bool,
            cal_done: bool,
        }
        let proxy = Hrccal1 {
            cal_length: self.cal_length(),
            cal_en: self.cal_en(),
            cal_done: self.cal_done(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HRC Calibration Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrccal2(pub u32);
impl Hrccal2 {
    #[doc = "Total clk_hrc48 cycles during calibration"]
    #[inline(always)]
    pub const fn hrc_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total clk_hrc48 cycles during calibration"]
    #[inline(always)]
    pub fn set_hrc_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Total clk_hxt48 cycles during calibration"]
    #[inline(always)]
    pub const fn hxt_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Total clk_hxt48 cycles during calibration"]
    #[inline(always)]
    pub fn set_hxt_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Hrccal2 {
    #[inline(always)]
    fn default() -> Hrccal2 {
        Hrccal2(0)
    }
}
impl core::fmt::Debug for Hrccal2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hrccal2")
            .field("hrc_cnt", &self.hrc_cnt())
            .field("hxt_cnt", &self.hxt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hrccal2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hrccal2 {
            hrc_cnt: u16,
            hxt_cnt: u16,
        }
        let proxy = Hrccal2 {
            hrc_cnt: self.hrc_cnt(),
            hxt_cnt: self.hxt_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Reset Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstr1(pub u32);
impl Rstr1 {
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn dmac1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_dmac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn mailbox1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_mailbox1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn pinmux1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_pinmux1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn usart1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_usart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn usart2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_usart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn ezip1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_ezip1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn epic(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_epic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn lcdc1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_lcdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn i2s1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_i2s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn syscfg1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_syscfg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn efusec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_efusec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn aes(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_aes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn crc1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_crc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn gptim1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_gptim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn gptim2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_gptim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn btim1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_btim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn btim2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_btim2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn wdt1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_wdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn spi2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_spi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn extdma(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_extdma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn pdm1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_pdm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn i2c2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_i2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn ptc1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_ptc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("dmac1", &self.dmac1())
            .field("mailbox1", &self.mailbox1())
            .field("pinmux1", &self.pinmux1())
            .field("usart1", &self.usart1())
            .field("usart2", &self.usart2())
            .field("ezip1", &self.ezip1())
            .field("epic", &self.epic())
            .field("lcdc1", &self.lcdc1())
            .field("i2s1", &self.i2s1())
            .field("syscfg1", &self.syscfg1())
            .field("efusec", &self.efusec())
            .field("aes", &self.aes())
            .field("crc1", &self.crc1())
            .field("trng", &self.trng())
            .field("gptim1", &self.gptim1())
            .field("gptim2", &self.gptim2())
            .field("btim1", &self.btim1())
            .field("btim2", &self.btim2())
            .field("wdt1", &self.wdt1())
            .field("spi1", &self.spi1())
            .field("spi2", &self.spi2())
            .field("extdma", &self.extdma())
            .field("pdm1", &self.pdm1())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("ptc1", &self.ptc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rstr1 {
            dmac1: bool,
            mailbox1: bool,
            pinmux1: bool,
            usart1: bool,
            usart2: bool,
            ezip1: bool,
            epic: bool,
            lcdc1: bool,
            i2s1: bool,
            syscfg1: bool,
            efusec: bool,
            aes: bool,
            crc1: bool,
            trng: bool,
            gptim1: bool,
            gptim2: bool,
            btim1: bool,
            btim2: bool,
            wdt1: bool,
            spi1: bool,
            spi2: bool,
            extdma: bool,
            pdm1: bool,
            i2c1: bool,
            i2c2: bool,
            ptc1: bool,
        }
        let proxy = Rstr1 {
            dmac1: self.dmac1(),
            mailbox1: self.mailbox1(),
            pinmux1: self.pinmux1(),
            usart1: self.usart1(),
            usart2: self.usart2(),
            ezip1: self.ezip1(),
            epic: self.epic(),
            lcdc1: self.lcdc1(),
            i2s1: self.i2s1(),
            syscfg1: self.syscfg1(),
            efusec: self.efusec(),
            aes: self.aes(),
            crc1: self.crc1(),
            trng: self.trng(),
            gptim1: self.gptim1(),
            gptim2: self.gptim2(),
            btim1: self.btim1(),
            btim2: self.btim2(),
            wdt1: self.wdt1(),
            spi1: self.spi1(),
            spi2: self.spi2(),
            extdma: self.extdma(),
            pdm1: self.pdm1(),
            i2c1: self.i2c1(),
            i2c2: self.i2c2(),
            ptc1: self.ptc1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Reset Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstr2(pub u32);
impl Rstr2 {
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn mpi1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_mpi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn mpi2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_mpi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn sdmmc1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_sdmmc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn usbc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_usbc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn i2c3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_i2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn atim1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_atim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn usart3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_usart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn audcodec(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_audcodec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn audprc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_audprc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn gpadc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_gpadc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn tsen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_tsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub const fn i2c4(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "0 - no reset; 1 - reset"]
    #[inline(always)]
    pub fn set_i2c4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Rstr2 {
    #[inline(always)]
    fn default() -> Rstr2 {
        Rstr2(0)
    }
}
impl core::fmt::Debug for Rstr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rstr2")
            .field("gpio1", &self.gpio1())
            .field("mpi1", &self.mpi1())
            .field("mpi2", &self.mpi2())
            .field("sdmmc1", &self.sdmmc1())
            .field("usbc", &self.usbc())
            .field("i2c3", &self.i2c3())
            .field("atim1", &self.atim1())
            .field("usart3", &self.usart3())
            .field("audcodec", &self.audcodec())
            .field("audprc", &self.audprc())
            .field("gpadc", &self.gpadc())
            .field("tsen", &self.tsen())
            .field("i2c4", &self.i2c4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rstr2 {
            gpio1: bool,
            mpi1: bool,
            mpi2: bool,
            sdmmc1: bool,
            usbc: bool,
            i2c3: bool,
            atim1: bool,
            usart3: bool,
            audcodec: bool,
            audprc: bool,
            gpadc: bool,
            tsen: bool,
            i2c4: bool,
        }
        let proxy = Rstr2 {
            gpio1: self.gpio1(),
            mpi1: self.mpi1(),
            mpi2: self.mpi2(),
            sdmmc1: self.sdmmc1(),
            usbc: self.usbc(),
            i2c3: self.i2c3(),
            atim1: self.atim1(),
            usart3: self.usart3(),
            audcodec: self.audcodec(),
            audprc: self.audprc(),
            gpadc: self.gpadc(),
            tsen: self.tsen(),
            i2c4: self.i2c4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USBC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbcr(pub u32);
impl Usbcr {
    #[doc = "USB function clock is USB source clock divided by DIV. After divider, USB function clock must be 60MHz. For example, if USBC clock source is 240MHz clk_dll2, DIV should be 4."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "USB function clock is USB source clock divided by DIV. After divider, USB function clock must be 60MHz. For example, if USBC clock source is 240MHz clk_dll2, DIV should be 4."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Usbcr {
    #[inline(always)]
    fn default() -> Usbcr {
        Usbcr(0)
    }
}
impl core::fmt::Debug for Usbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbcr").field("div", &self.div()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usbcr {
            div: u8,
        }
        let proxy = Usbcr { div: self.div() };
        defmt::write!(f, "{}", proxy)
    }
}
