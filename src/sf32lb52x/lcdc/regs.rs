#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanvasBg(pub u32);
impl CanvasBg {
    #[doc = "blue color"]
    #[must_use]
    #[inline(always)]
    pub const fn blue(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "blue color"]
    #[inline(always)]
    pub const fn set_blue(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "green color"]
    #[must_use]
    #[inline(always)]
    pub const fn green(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "green color"]
    #[inline(always)]
    pub const fn set_green(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Red color"]
    #[must_use]
    #[inline(always)]
    pub const fn red(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Red color"]
    #[inline(always)]
    pub const fn set_red(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "if this bit is set, the layer is not blending with background. The alpha value will be reserved to output."]
    #[must_use]
    #[inline(always)]
    pub const fn bg_blending_bypass(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "if this bit is set, the layer is not blending with background. The alpha value will be reserved to output."]
    #[inline(always)]
    pub const fn set_bg_blending_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "if this bit is set, lcdc is in pure dma mode. No blending operation."]
    #[must_use]
    #[inline(always)]
    pub const fn all_blending_bypass(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "if this bit is set, lcdc is in pure dma mode. No blending operation."]
    #[inline(always)]
    pub const fn set_all_blending_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "line buffer bypass. Set 1 to bypass line buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn lb_bypass(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "line buffer bypass. Set 1 to bypass line buffer."]
    #[inline(always)]
    pub const fn set_lb_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "set 1 to do horizontal mirror for output image"]
    #[must_use]
    #[inline(always)]
    pub const fn h_mirror(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to do horizontal mirror for output image"]
    #[inline(always)]
    pub const fn set_h_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for CanvasBg {
    #[inline(always)]
    fn default() -> CanvasBg {
        CanvasBg(0)
    }
}
impl core::fmt::Debug for CanvasBg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CanvasBg")
            .field("blue", &self.blue())
            .field("green", &self.green())
            .field("red", &self.red())
            .field("bg_blending_bypass", &self.bg_blending_bypass())
            .field("all_blending_bypass", &self.all_blending_bypass())
            .field("lb_bypass", &self.lb_bypass())
            .field("h_mirror", &self.h_mirror())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CanvasBg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CanvasBg {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?}, bg_blending_bypass: {=bool:?}, all_blending_bypass: {=bool:?}, lb_bypass: {=bool:?}, h_mirror: {=bool:?} }}" , self . blue () , self . green () , self . red () , self . bg_blending_bypass () , self . all_blending_bypass () , self . lb_bypass () , self . h_mirror ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanvasBrPos(pub u32);
impl CanvasBrPos {
    #[must_use]
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for CanvasBrPos {
    #[inline(always)]
    fn default() -> CanvasBrPos {
        CanvasBrPos(0)
    }
}
impl core::fmt::Debug for CanvasBrPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CanvasBrPos")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CanvasBrPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CanvasBrPos {{ x1: {=u16:?}, y1: {=u16:?} }}",
            self.x1(),
            self.y1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanvasStat0(pub u32);
impl CanvasStat0 {
    #[doc = "canvas x cordinate"]
    #[must_use]
    #[inline(always)]
    pub const fn x_cor(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "canvas x cordinate"]
    #[inline(always)]
    pub const fn set_x_cor(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "canvas y cordinate"]
    #[must_use]
    #[inline(always)]
    pub const fn y_cor(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "canvas y cordinate"]
    #[inline(always)]
    pub const fn set_y_cor(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for CanvasStat0 {
    #[inline(always)]
    fn default() -> CanvasStat0 {
        CanvasStat0(0)
    }
}
impl core::fmt::Debug for CanvasStat0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CanvasStat0")
            .field("x_cor", &self.x_cor())
            .field("y_cor", &self.y_cor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CanvasStat0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CanvasStat0 {{ x_cor: {=u16:?}, y_cor: {=u16:?} }}",
            self.x_cor(),
            self.y_cor()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanvasStat1(pub u32);
impl CanvasStat1 {
    #[doc = "pre calc fifo count"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "pre calc fifo count"]
    #[inline(always)]
    pub const fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "postc_status"]
    #[must_use]
    #[inline(always)]
    pub const fn postc_stat(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "postc_status"]
    #[inline(always)]
    pub const fn set_postc_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "prec status"]
    #[must_use]
    #[inline(always)]
    pub const fn prec_stat(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "prec status"]
    #[inline(always)]
    pub const fn set_prec_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "fetch status"]
    #[must_use]
    #[inline(always)]
    pub const fn fetch_stat(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "fetch status"]
    #[inline(always)]
    pub const fn set_fetch_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
}
impl Default for CanvasStat1 {
    #[inline(always)]
    fn default() -> CanvasStat1 {
        CanvasStat1(0)
    }
}
impl core::fmt::Debug for CanvasStat1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CanvasStat1")
            .field("fifo_cnt", &self.fifo_cnt())
            .field("postc_stat", &self.postc_stat())
            .field("prec_stat", &self.prec_stat())
            .field("fetch_stat", &self.fetch_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CanvasStat1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CanvasStat1 {{ fifo_cnt: {=u8:?}, postc_stat: {=u8:?}, prec_stat: {=u8:?}, fetch_stat: {=u8:?} }}" , self . fifo_cnt () , self . postc_stat () , self . prec_stat () , self . fetch_stat ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanvasTlPos(pub u32);
impl CanvasTlPos {
    #[must_use]
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for CanvasTlPos {
    #[inline(always)]
    fn default() -> CanvasTlPos {
        CanvasTlPos(0)
    }
}
impl core::fmt::Debug for CanvasTlPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CanvasTlPos")
            .field("x0", &self.x0())
            .field("y0", &self.y0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CanvasTlPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CanvasTlPos {{ x0: {=u16:?}, y0: {=u16:?} }}",
            self.x0(),
            self.y0()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Command(pub u32);
impl Command {
    #[doc = "write 1 to trigger the lcd interface block"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to trigger the lcd interface block"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: reset the whole graphics 0: release the reset"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: reset the whole graphics 0: release the reset"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Command {
    #[inline(always)]
    fn default() -> Command {
        Command(0)
    }
}
impl core::fmt::Debug for Command {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Command")
            .field("start", &self.start())
            .field("reset", &self.reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Command {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Command {{ start: {=bool:?}, reset: {=bool:?} }}",
            self.start(),
            self.reset()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DitherConf(pub u32);
impl DitherConf {
    #[doc = "dither enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "dither enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "blue dither width"]
    #[must_use]
    #[inline(always)]
    pub const fn w_b(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "blue dither width"]
    #[inline(always)]
    pub const fn set_w_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "green dither width"]
    #[must_use]
    #[inline(always)]
    pub const fn w_g(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "green dither width"]
    #[inline(always)]
    pub const fn set_w_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "red dither width"]
    #[must_use]
    #[inline(always)]
    pub const fn w_r(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "red dither width"]
    #[inline(always)]
    pub const fn set_w_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "select lfsr 0: none 1: red 2: green 3: blue"]
    #[must_use]
    #[inline(always)]
    pub const fn lfsr_load_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "select lfsr 0: none 1: red 2: green 3: blue"]
    #[inline(always)]
    pub const fn set_lfsr_load_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "load lfsr init value"]
    #[must_use]
    #[inline(always)]
    pub const fn lfsr_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "load lfsr init value"]
    #[inline(always)]
    pub const fn set_lfsr_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for DitherConf {
    #[inline(always)]
    fn default() -> DitherConf {
        DitherConf(0)
    }
}
impl core::fmt::Debug for DitherConf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DitherConf")
            .field("en", &self.en())
            .field("w_b", &self.w_b())
            .field("w_g", &self.w_g())
            .field("w_r", &self.w_r())
            .field("lfsr_load_sel", &self.lfsr_load_sel())
            .field("lfsr_load", &self.lfsr_load())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DitherConf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DitherConf {{ en: {=bool:?}, w_b: {=u8:?}, w_g: {=u8:?}, w_r: {=u8:?}, lfsr_load_sel: {=u8:?}, lfsr_load: {=bool:?} }}" , self . en () , self . w_b () , self . w_g () , self . w_r () , self . lfsr_load_sel () , self . lfsr_load ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DitherLfsr(pub u32);
impl DitherLfsr {
    #[doc = "lfsr init load value"]
    #[must_use]
    #[inline(always)]
    pub const fn init_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "lfsr init load value"]
    #[inline(always)]
    pub const fn set_init_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DitherLfsr {
    #[inline(always)]
    fn default() -> DitherLfsr {
        DitherLfsr(0)
    }
}
impl core::fmt::Debug for DitherLfsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DitherLfsr")
            .field("init_val", &self.init_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DitherLfsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DitherLfsr {{ init_val: {=u32:?} }}", self.init_val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpiCtrl(pub u32);
impl DpiCtrl {
    #[doc = "dpi interface enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "dpi interface enable"]
    #[inline(always)]
    pub const fn set_dpi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "dpi color mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_cm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "dpi color mode"]
    #[inline(always)]
    pub const fn set_dpi_cm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "dpi shutdown"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_sd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "dpi shutdown"]
    #[inline(always)]
    pub const fn set_dpi_sd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "dpi update config"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_uc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "dpi update config"]
    #[inline(always)]
    pub const fn set_dpi_uc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for DpiCtrl {
    #[inline(always)]
    fn default() -> DpiCtrl {
        DpiCtrl(0)
    }
}
impl core::fmt::Debug for DpiCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DpiCtrl")
            .field("dpi_en", &self.dpi_en())
            .field("dpi_cm", &self.dpi_cm())
            .field("dpi_sd", &self.dpi_sd())
            .field("dpi_uc", &self.dpi_uc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpiCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DpiCtrl {{ dpi_en: {=bool:?}, dpi_cm: {=bool:?}, dpi_sd: {=bool:?}, dpi_uc: {=bool:?} }}" , self . dpi_en () , self . dpi_cm () , self . dpi_sd () , self . dpi_uc ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpiIfConf1(pub u32);
impl DpiIfConf1 {
    #[doc = "dpi vsync height"]
    #[must_use]
    #[inline(always)]
    pub const fn vsh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "dpi vsync height"]
    #[inline(always)]
    pub const fn set_vsh(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "dpi hsync width"]
    #[must_use]
    #[inline(always)]
    pub const fn hsw(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "dpi hsync width"]
    #[inline(always)]
    pub const fn set_hsw(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for DpiIfConf1 {
    #[inline(always)]
    fn default() -> DpiIfConf1 {
        DpiIfConf1(0)
    }
}
impl core::fmt::Debug for DpiIfConf1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DpiIfConf1")
            .field("vsh", &self.vsh())
            .field("hsw", &self.hsw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpiIfConf1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DpiIfConf1 {{ vsh: {=u16:?}, hsw: {=u16:?} }}",
            self.vsh(),
            self.hsw()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpiIfConf2(pub u32);
impl DpiIfConf2 {
    #[doc = "vertical back porch"]
    #[must_use]
    #[inline(always)]
    pub const fn vbp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "vertical back porch"]
    #[inline(always)]
    pub const fn set_vbp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "horizontal back porch"]
    #[must_use]
    #[inline(always)]
    pub const fn hbp(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "horizontal back porch"]
    #[inline(always)]
    pub const fn set_hbp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for DpiIfConf2 {
    #[inline(always)]
    fn default() -> DpiIfConf2 {
        DpiIfConf2(0)
    }
}
impl core::fmt::Debug for DpiIfConf2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DpiIfConf2")
            .field("vbp", &self.vbp())
            .field("hbp", &self.hbp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpiIfConf2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DpiIfConf2 {{ vbp: {=u16:?}, hbp: {=u16:?} }}",
            self.vbp(),
            self.hbp()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpiIfConf3(pub u32);
impl DpiIfConf3 {
    #[doc = "vertical front porch"]
    #[must_use]
    #[inline(always)]
    pub const fn vfp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "vertical front porch"]
    #[inline(always)]
    pub const fn set_vfp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "horizontal front porch"]
    #[must_use]
    #[inline(always)]
    pub const fn hfp(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "horizontal front porch"]
    #[inline(always)]
    pub const fn set_hfp(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for DpiIfConf3 {
    #[inline(always)]
    fn default() -> DpiIfConf3 {
        DpiIfConf3(0)
    }
}
impl core::fmt::Debug for DpiIfConf3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DpiIfConf3")
            .field("vfp", &self.vfp())
            .field("hfp", &self.hfp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpiIfConf3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DpiIfConf3 {{ vfp: {=u16:?}, hfp: {=u16:?} }}",
            self.vfp(),
            self.hfp()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpiIfConf4(pub u32);
impl DpiIfConf4 {
    #[doc = "vertical active height"]
    #[must_use]
    #[inline(always)]
    pub const fn vah(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "vertical active height"]
    #[inline(always)]
    pub const fn set_vah(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "horizontal active width"]
    #[must_use]
    #[inline(always)]
    pub const fn haw(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "horizontal active width"]
    #[inline(always)]
    pub const fn set_haw(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for DpiIfConf4 {
    #[inline(always)]
    fn default() -> DpiIfConf4 {
        DpiIfConf4(0)
    }
}
impl core::fmt::Debug for DpiIfConf4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DpiIfConf4")
            .field("vah", &self.vah())
            .field("haw", &self.haw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpiIfConf4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DpiIfConf4 {{ vah: {=u16:?}, haw: {=u16:?} }}",
            self.vah(),
            self.haw()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpiIfConf5(pub u32);
impl DpiIfConf5 {
    #[doc = "pixel clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pclk_div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "pixel clock divider"]
    #[inline(always)]
    pub const fn set_pclk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "pixel clock polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pclkpol(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "pixel clock polarity"]
    #[inline(always)]
    pub const fn set_pclkpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "de polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn depol(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "de polarity"]
    #[inline(always)]
    pub const fn set_depol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "vsync polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn vspol(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "vsync polarity"]
    #[inline(always)]
    pub const fn set_vspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "hsync polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn hspol(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "hsync polarity"]
    #[inline(always)]
    pub const fn set_hspol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DPI interrupt line number"]
    #[must_use]
    #[inline(always)]
    pub const fn int_line_num(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x07ff;
        val as u16
    }
    #[doc = "DPI interrupt line number"]
    #[inline(always)]
    pub const fn set_int_line_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 12usize)) | (((val as u32) & 0x07ff) << 12usize);
    }
    #[doc = "1: force DPI clock on 0: DPI clock is controlled by hardware"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_force_on(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "1: force DPI clock on 0: DPI clock is controlled by hardware"]
    #[inline(always)]
    pub const fn set_clk_force_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for DpiIfConf5 {
    #[inline(always)]
    fn default() -> DpiIfConf5 {
        DpiIfConf5(0)
    }
}
impl core::fmt::Debug for DpiIfConf5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DpiIfConf5")
            .field("pclk_div", &self.pclk_div())
            .field("pclkpol", &self.pclkpol())
            .field("depol", &self.depol())
            .field("vspol", &self.vspol())
            .field("hspol", &self.hspol())
            .field("int_line_num", &self.int_line_num())
            .field("clk_force_on", &self.clk_force_on())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpiIfConf5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DpiIfConf5 {{ pclk_div: {=u8:?}, pclkpol: {=bool:?}, depol: {=bool:?}, vspol: {=bool:?}, hspol: {=bool:?}, int_line_num: {=u16:?}, clk_force_on: {=bool:?} }}" , self . pclk_div () , self . pclkpol () , self . depol () , self . vspol () , self . hspol () , self . int_line_num () , self . clk_force_on ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpiStat(pub u32);
impl DpiStat {
    #[doc = "dpi horizontal position"]
    #[must_use]
    #[inline(always)]
    pub const fn hpos(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "dpi horizontal position"]
    #[inline(always)]
    pub const fn set_hpos(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "horizontal status 0: idle 1: prep 2: hsync 3: hbp 4: hact 5: hfp 6: wait"]
    #[must_use]
    #[inline(always)]
    pub const fn hstat(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "horizontal status 0: idle 1: prep 2: hsync 3: hbp 4: hact 5: hfp 6: wait"]
    #[inline(always)]
    pub const fn set_hstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[doc = "dpi vertical position"]
    #[must_use]
    #[inline(always)]
    pub const fn vpos(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "dpi vertical position"]
    #[inline(always)]
    pub const fn set_vpos(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DpiStat {
    #[inline(always)]
    fn default() -> DpiStat {
        DpiStat(0)
    }
}
impl core::fmt::Debug for DpiStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DpiStat")
            .field("hpos", &self.hpos())
            .field("hstat", &self.hstat())
            .field("vpos", &self.vpos())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DpiStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DpiStat {{ hpos: {=u16:?}, hstat: {=u8:?}, vpos: {=u16:?} }}",
            self.hpos(),
            self.hstat(),
            self.vpos()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub u32);
impl Irq {
    #[doc = "end of frame interrupt, masked by mask register"]
    #[must_use]
    #[inline(always)]
    pub const fn eof_stat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "end of frame interrupt, masked by mask register"]
    #[inline(always)]
    pub const fn set_eof_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "icb overflow interrupt, masked by mask register"]
    #[must_use]
    #[inline(always)]
    pub const fn icb_of_stat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "icb overflow interrupt, masked by mask register"]
    #[inline(always)]
    pub const fn set_icb_of_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "dpi line interrupt, masked by mask register"]
    #[must_use]
    #[inline(always)]
    pub const fn dpil_intr_stat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "dpi line interrupt, masked by mask register"]
    #[inline(always)]
    pub const fn set_dpil_intr_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "dpi under run interrupt, masked by mask register"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_udr_stat(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "dpi under run interrupt, masked by mask register"]
    #[inline(always)]
    pub const fn set_dpi_udr_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "jdi parallel interface line interrupt, masked by mask register"]
    #[must_use]
    #[inline(always)]
    pub const fn jdi_parl_intr_stat(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel interface line interrupt, masked by mask register"]
    #[inline(always)]
    pub const fn set_jdi_parl_intr_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "jdi parallel interface under run interrupt, masked by mask register"]
    #[must_use]
    #[inline(always)]
    pub const fn jdi_par_udr_stat(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel interface under run interrupt, masked by mask register"]
    #[inline(always)]
    pub const fn set_jdi_par_udr_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "line process done interrupt, masked by mask register"]
    #[must_use]
    #[inline(always)]
    pub const fn line_done_stat(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "line process done interrupt, masked by mask register"]
    #[inline(always)]
    pub const fn set_line_done_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "raw status of end of frame interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn eof_raw_stat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "raw status of end of frame interrupt"]
    #[inline(always)]
    pub const fn set_eof_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "raw status of icb overflow interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn icb_of_raw_stat(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "raw status of icb overflow interrupt"]
    #[inline(always)]
    pub const fn set_icb_of_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "raw status of dpi line interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dpil_intr_raw_stat(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "raw status of dpi line interrupt"]
    #[inline(always)]
    pub const fn set_dpil_intr_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "raw status of dpi under run interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_udr_raw_stat(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "raw status of dpi under run interrupt"]
    #[inline(always)]
    pub const fn set_dpi_udr_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "raw_status of jdi parallel interface line interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn jdi_parl_intr_raw_stat(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "raw_status of jdi parallel interface line interrupt"]
    #[inline(always)]
    pub const fn set_jdi_parl_intr_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "raw_status of jdi parallel interface under run interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn jdi_par_udr_raw_stat(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "raw_status of jdi parallel interface under run interrupt"]
    #[inline(always)]
    pub const fn set_jdi_par_udr_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "raw_status of line process done interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn line_done_raw_stat(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "raw_status of line process done interrupt"]
    #[inline(always)]
    pub const fn set_line_done_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Irq {
    #[inline(always)]
    fn default() -> Irq {
        Irq(0)
    }
}
impl core::fmt::Debug for Irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irq")
            .field("eof_stat", &self.eof_stat())
            .field("icb_of_stat", &self.icb_of_stat())
            .field("dpil_intr_stat", &self.dpil_intr_stat())
            .field("dpi_udr_stat", &self.dpi_udr_stat())
            .field("jdi_parl_intr_stat", &self.jdi_parl_intr_stat())
            .field("jdi_par_udr_stat", &self.jdi_par_udr_stat())
            .field("line_done_stat", &self.line_done_stat())
            .field("eof_raw_stat", &self.eof_raw_stat())
            .field("icb_of_raw_stat", &self.icb_of_raw_stat())
            .field("dpil_intr_raw_stat", &self.dpil_intr_raw_stat())
            .field("dpi_udr_raw_stat", &self.dpi_udr_raw_stat())
            .field("jdi_parl_intr_raw_stat", &self.jdi_parl_intr_raw_stat())
            .field("jdi_par_udr_raw_stat", &self.jdi_par_udr_raw_stat())
            .field("line_done_raw_stat", &self.line_done_raw_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Irq {{ eof_stat: {=bool:?}, icb_of_stat: {=bool:?}, dpil_intr_stat: {=bool:?}, dpi_udr_stat: {=bool:?}, jdi_parl_intr_stat: {=bool:?}, jdi_par_udr_stat: {=bool:?}, line_done_stat: {=bool:?}, eof_raw_stat: {=bool:?}, icb_of_raw_stat: {=bool:?}, dpil_intr_raw_stat: {=bool:?}, dpi_udr_raw_stat: {=bool:?}, jdi_parl_intr_raw_stat: {=bool:?}, jdi_par_udr_raw_stat: {=bool:?}, line_done_raw_stat: {=bool:?} }}" , self . eof_stat () , self . icb_of_stat () , self . dpil_intr_stat () , self . dpi_udr_stat () , self . jdi_parl_intr_stat () , self . jdi_par_udr_stat () , self . line_done_stat () , self . eof_raw_stat () , self . icb_of_raw_stat () , self . dpil_intr_raw_stat () , self . dpi_udr_raw_stat () , self . jdi_parl_intr_raw_stat () , self . jdi_par_udr_raw_stat () , self . line_done_raw_stat ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf1(pub u32);
impl JdiParConf1 {
    #[doc = "jdi parallel interface max column, column number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn max_col(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface max column, column number start from 0"]
    #[inline(always)]
    pub const fn set_max_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface max line, line number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn max_line(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface max line, line number start from 0"]
    #[inline(always)]
    pub const fn set_max_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf1 {
    #[inline(always)]
    fn default() -> JdiParConf1 {
        JdiParConf1(0)
    }
}
impl core::fmt::Debug for JdiParConf1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf1")
            .field("max_col", &self.max_col())
            .field("max_line", &self.max_line())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf1 {{ max_col: {=u16:?}, max_line: {=u16:?} }}",
            self.max_col(),
            self.max_line()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf10(pub u32);
impl JdiParConf10 {
    #[doc = "jdi parallel interface horizontal control end line, line number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hc_end_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface horizontal control end line, line number start from 0"]
    #[inline(always)]
    pub const fn set_hc_end_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface horizontal control start line, line number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hc_st_line(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface horizontal control start line, line number start from 0"]
    #[inline(always)]
    pub const fn set_hc_st_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf10 {
    #[inline(always)]
    fn default() -> JdiParConf10 {
        JdiParConf10(0)
    }
}
impl core::fmt::Debug for JdiParConf10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf10")
            .field("hc_end_line", &self.hc_end_line())
            .field("hc_st_line", &self.hc_st_line())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf10 {{ hc_end_line: {=u16:?}, hc_st_line: {=u16:?} }}",
            self.hc_end_line(),
            self.hc_st_line()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf2(pub u32);
impl JdiParConf2 {
    #[doc = "jdi parallel interface end line, line number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn end_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface end line, line number start from 0"]
    #[inline(always)]
    pub const fn set_end_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface start line, line number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn st_line(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface start line, line number start from 0"]
    #[inline(always)]
    pub const fn set_st_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf2 {
    #[inline(always)]
    fn default() -> JdiParConf2 {
        JdiParConf2(0)
    }
}
impl core::fmt::Debug for JdiParConf2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf2")
            .field("end_line", &self.end_line())
            .field("st_line", &self.st_line())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf2 {{ end_line: {=u16:?}, st_line: {=u16:?} }}",
            self.end_line(),
            self.st_line()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf3(pub u32);
impl JdiParConf3 {
    #[doc = "jdi parallel interface end column, column number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn end_col(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface end column, column number start from 0"]
    #[inline(always)]
    pub const fn set_end_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface start column, column number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn st_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface start column, column number start from 0"]
    #[inline(always)]
    pub const fn set_st_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf3 {
    #[inline(always)]
    fn default() -> JdiParConf3 {
        JdiParConf3(0)
    }
}
impl core::fmt::Debug for JdiParConf3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf3")
            .field("end_col", &self.end_col())
            .field("st_col", &self.st_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf3 {{ end_col: {=u16:?}, st_col: {=u16:?} }}",
            self.end_col(),
            self.st_col()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf4(pub u32);
impl JdiParConf4 {
    #[doc = "jdi parallel interface HST width, HST width = lcd_ck_cycle * HST_WIDTH"]
    #[must_use]
    #[inline(always)]
    pub const fn hst_width(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface HST width, HST width = lcd_ck_cycle * HST_WIDTH"]
    #[inline(always)]
    pub const fn set_hst_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface HCK width, HSK width = lcd_ck_cycle * HCK_WIDTH"]
    #[must_use]
    #[inline(always)]
    pub const fn hck_width(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface HCK width, HSK width = lcd_ck_cycle * HCK_WIDTH"]
    #[inline(always)]
    pub const fn set_hck_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf4 {
    #[inline(always)]
    fn default() -> JdiParConf4 {
        JdiParConf4(0)
    }
}
impl core::fmt::Debug for JdiParConf4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf4")
            .field("hst_width", &self.hst_width())
            .field("hck_width", &self.hck_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf4 {{ hst_width: {=u16:?}, hck_width: {=u16:?} }}",
            self.hst_width(),
            self.hck_width()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf5(pub u32);
impl JdiParConf5 {
    #[doc = "jdi parallel interface VST width, VST width = lcd_ck_cycle * VST_WIDTH"]
    #[must_use]
    #[inline(always)]
    pub const fn vst_width(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface VST width, VST width = lcd_ck_cycle * VST_WIDTH"]
    #[inline(always)]
    pub const fn set_vst_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface VCK width, VCK width = lcd_ck_cycle * VCK_WIDTH"]
    #[must_use]
    #[inline(always)]
    pub const fn vck_width(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface VCK width, VCK width = lcd_ck_cycle * VCK_WIDTH"]
    #[inline(always)]
    pub const fn set_vck_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf5 {
    #[inline(always)]
    fn default() -> JdiParConf5 {
        JdiParConf5(0)
    }
}
impl core::fmt::Debug for JdiParConf5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf5")
            .field("vst_width", &self.vst_width())
            .field("vck_width", &self.vck_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf5 {{ vst_width: {=u16:?}, vck_width: {=u16:?} }}",
            self.vst_width(),
            self.vck_width()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf6(pub u32);
impl JdiParConf6 {
    #[doc = "jdi parallel interface VCK to HST delay, VCK2HST delay = lcd_ck_cycle * HST_DLY"]
    #[must_use]
    #[inline(always)]
    pub const fn hst_dly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface VCK to HST delay, VCK2HST delay = lcd_ck_cycle * HST_DLY"]
    #[inline(always)]
    pub const fn set_hst_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface VST to VCK delay, VST2VCK delay = lcd_ck_cycle * VCK_DLY"]
    #[must_use]
    #[inline(always)]
    pub const fn vck_dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface VST to VCK delay, VST2VCK delay = lcd_ck_cycle * VCK_DLY"]
    #[inline(always)]
    pub const fn set_vck_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf6 {
    #[inline(always)]
    fn default() -> JdiParConf6 {
        JdiParConf6(0)
    }
}
impl core::fmt::Debug for JdiParConf6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf6")
            .field("hst_dly", &self.hst_dly())
            .field("vck_dly", &self.vck_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf6 {{ hst_dly: {=u16:?}, vck_dly: {=u16:?} }}",
            self.hst_dly(),
            self.vck_dly()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf7(pub u32);
impl JdiParConf7 {
    #[doc = "jdi parallel interface HST to HCK delay"]
    #[must_use]
    #[inline(always)]
    pub const fn hck_dly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface HST to HCK delay"]
    #[inline(always)]
    pub const fn set_hck_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "double pixel mode. Some jdi parallel screens use large pixel+small pixel structure. Set this bit to 1 to support this structure."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "double pixel mode. Some jdi parallel screens use large pixel+small pixel structure. Set this bit to 1 to support this structure."]
    #[inline(always)]
    pub const fn set_dp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for JdiParConf7 {
    #[inline(always)]
    fn default() -> JdiParConf7 {
        JdiParConf7(0)
    }
}
impl core::fmt::Debug for JdiParConf7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf7")
            .field("hck_dly", &self.hck_dly())
            .field("dp_mode", &self.dp_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf7 {{ hck_dly: {=u16:?}, dp_mode: {=bool:?} }}",
            self.hck_dly(),
            self.dp_mode()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf8(pub u32);
impl JdiParConf8 {
    #[doc = "jdi parallel interface enb end column, column number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn enb_end_col(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface enb end column, column number start from 0"]
    #[inline(always)]
    pub const fn set_enb_end_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface enb start column, column number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn enb_st_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface enb start column, column number start from 0"]
    #[inline(always)]
    pub const fn set_enb_st_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf8 {
    #[inline(always)]
    fn default() -> JdiParConf8 {
        JdiParConf8(0)
    }
}
impl core::fmt::Debug for JdiParConf8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf8")
            .field("enb_end_col", &self.enb_end_col())
            .field("enb_st_col", &self.enb_st_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf8 {{ enb_end_col: {=u16:?}, enb_st_col: {=u16:?} }}",
            self.enb_end_col(),
            self.enb_st_col()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParConf9(pub u32);
impl JdiParConf9 {
    #[doc = "jdi parallel interface enb end line, line number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn enb_end_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface enb end line, line number start from 0"]
    #[inline(always)]
    pub const fn set_enb_end_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel interface enb start line, line number start from 0"]
    #[must_use]
    #[inline(always)]
    pub const fn enb_st_line(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface enb start line, line number start from 0"]
    #[inline(always)]
    pub const fn set_enb_st_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParConf9 {
    #[inline(always)]
    fn default() -> JdiParConf9 {
        JdiParConf9(0)
    }
}
impl core::fmt::Debug for JdiParConf9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParConf9")
            .field("enb_end_line", &self.enb_end_line())
            .field("enb_st_line", &self.enb_st_line())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParConf9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParConf9 {{ enb_end_line: {=u16:?}, enb_st_line: {=u16:?} }}",
            self.enb_end_line(),
            self.enb_st_line()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParCtrl(pub u32);
impl JdiParCtrl {
    #[doc = "jdi parallel interface enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel interface enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "jdi parallel interface XRST"]
    #[must_use]
    #[inline(always)]
    pub const fn xrst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel interface XRST"]
    #[inline(always)]
    pub const fn set_xrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "jdi parallel enb polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn enbpol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel enb polarity"]
    #[inline(always)]
    pub const fn set_enbpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "jdi parallel hck polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn hckpol(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel hck polarity"]
    #[inline(always)]
    pub const fn set_hckpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "jdi parallel hst polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpol(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel hst polarity"]
    #[inline(always)]
    pub const fn set_hstpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "jdi parallel vck polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn vckpol(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel vck polarity"]
    #[inline(always)]
    pub const fn set_vckpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "jdi parallel vst polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn vstpol(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel vst polarity"]
    #[inline(always)]
    pub const fn set_vstpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "jdi parallel interface interrupt line number, line number start from 0."]
    #[must_use]
    #[inline(always)]
    pub const fn int_line_num(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel interface interrupt line number, line number start from 0."]
    #[inline(always)]
    pub const fn set_int_line_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParCtrl {
    #[inline(always)]
    fn default() -> JdiParCtrl {
        JdiParCtrl(0)
    }
}
impl core::fmt::Debug for JdiParCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParCtrl")
            .field("enable", &self.enable())
            .field("xrst", &self.xrst())
            .field("enbpol", &self.enbpol())
            .field("hckpol", &self.hckpol())
            .field("hstpol", &self.hstpol())
            .field("vckpol", &self.vckpol())
            .field("vstpol", &self.vstpol())
            .field("int_line_num", &self.int_line_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "JdiParCtrl {{ enable: {=bool:?}, xrst: {=bool:?}, enbpol: {=bool:?}, hckpol: {=bool:?}, hstpol: {=bool:?}, vckpol: {=bool:?}, vstpol: {=bool:?}, int_line_num: {=u16:?} }}" , self . enable () , self . xrst () , self . enbpol () , self . hckpol () , self . hstpol () , self . vckpol () , self . vstpol () , self . int_line_num ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParExCtrl(pub u32);
impl JdiParExCtrl {
    #[doc = "VCOM/FRP/XFRP max counter"]
    #[must_use]
    #[inline(always)]
    pub const fn max_cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "VCOM/FRP/XFRP max counter"]
    #[inline(always)]
    pub const fn set_max_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "VCOM/FRP/XFRP counter enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_en(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "VCOM/FRP/XFRP counter enable"]
    #[inline(always)]
    pub const fn set_cnt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "XFRP value"]
    #[must_use]
    #[inline(always)]
    pub const fn xfrp(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "XFRP value"]
    #[inline(always)]
    pub const fn set_xfrp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "FRP value"]
    #[must_use]
    #[inline(always)]
    pub const fn frp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "FRP value"]
    #[inline(always)]
    pub const fn set_frp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "VCOM value"]
    #[must_use]
    #[inline(always)]
    pub const fn vcom(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "VCOM value"]
    #[inline(always)]
    pub const fn set_vcom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for JdiParExCtrl {
    #[inline(always)]
    fn default() -> JdiParExCtrl {
        JdiParExCtrl(0)
    }
}
impl core::fmt::Debug for JdiParExCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParExCtrl")
            .field("max_cnt", &self.max_cnt())
            .field("cnt_en", &self.cnt_en())
            .field("xfrp", &self.xfrp())
            .field("frp", &self.frp())
            .field("vcom", &self.vcom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParExCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "JdiParExCtrl {{ max_cnt: {=u32:?}, cnt_en: {=bool:?}, xfrp: {=bool:?}, frp: {=bool:?}, vcom: {=bool:?} }}" , self . max_cnt () , self . cnt_en () , self . xfrp () , self . frp () , self . vcom ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiParStat(pub u32);
impl JdiParStat {
    #[doc = "jdi parallel horizontal position"]
    #[must_use]
    #[inline(always)]
    pub const fn hpos(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel horizontal position"]
    #[inline(always)]
    pub const fn set_hpos(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi parallel vertical position"]
    #[must_use]
    #[inline(always)]
    pub const fn vpos(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi parallel vertical position"]
    #[inline(always)]
    pub const fn set_vpos(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiParStat {
    #[inline(always)]
    fn default() -> JdiParStat {
        JdiParStat(0)
    }
}
impl core::fmt::Debug for JdiParStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiParStat")
            .field("hpos", &self.hpos())
            .field("vpos", &self.vpos())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiParStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiParStat {{ hpos: {=u16:?}, vpos: {=u16:?} }}",
            self.hpos(),
            self.vpos()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiSerConf1(pub u32);
impl JdiSerConf1 {
    #[doc = "jdi single write bit length"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "jdi single write bit length"]
    #[inline(always)]
    pub const fn set_wr_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "jdi serial clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "jdi serial clock divider"]
    #[inline(always)]
    pub const fn set_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for JdiSerConf1 {
    #[inline(always)]
    fn default() -> JdiSerConf1 {
        JdiSerConf1(0)
    }
}
impl core::fmt::Debug for JdiSerConf1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiSerConf1")
            .field("wr_len", &self.wr_len())
            .field("clk_div", &self.clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiSerConf1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiSerConf1 {{ wr_len: {=u8:?}, clk_div: {=u8:?} }}",
            self.wr_len(),
            self.clk_div()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiSerConf2(pub u32);
impl JdiSerConf2 {
    #[doc = "jdi serial data transfer write command"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_cmd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi serial data transfer write command"]
    #[inline(always)]
    pub const fn set_wr_cmd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "jdi serial init line counter"]
    #[must_use]
    #[inline(always)]
    pub const fn init_line_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "jdi serial init line counter"]
    #[inline(always)]
    pub const fn set_init_line_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for JdiSerConf2 {
    #[inline(always)]
    fn default() -> JdiSerConf2 {
        JdiSerConf2(0)
    }
}
impl core::fmt::Debug for JdiSerConf2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiSerConf2")
            .field("wr_cmd", &self.wr_cmd())
            .field("init_line_cnt", &self.init_line_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiSerConf2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiSerConf2 {{ wr_cmd: {=u16:?}, init_line_cnt: {=u16:?} }}",
            self.wr_cmd(),
            self.init_line_cnt()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JdiSerCtrl(pub u32);
impl JdiSerCtrl {
    #[doc = "jdi serial interface disp control"]
    #[must_use]
    #[inline(always)]
    pub const fn disp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "jdi serial interface disp control"]
    #[inline(always)]
    pub const fn set_disp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "jdi serial interface extcomin control"]
    #[must_use]
    #[inline(always)]
    pub const fn extcomin(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "jdi serial interface extcomin control"]
    #[inline(always)]
    pub const fn set_extcomin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for JdiSerCtrl {
    #[inline(always)]
    fn default() -> JdiSerCtrl {
        JdiSerCtrl(0)
    }
}
impl core::fmt::Debug for JdiSerCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JdiSerCtrl")
            .field("disp", &self.disp())
            .field("extcomin", &self.extcomin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JdiSerCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JdiSerCtrl {{ disp: {=bool:?}, extcomin: {=bool:?} }}",
            self.disp(),
            self.extcomin()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0BrPos(pub u32);
impl Layer0BrPos {
    #[doc = "Coordinate X-value"]
    #[must_use]
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[must_use]
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Layer0BrPos {
    #[inline(always)]
    fn default() -> Layer0BrPos {
        Layer0BrPos(0)
    }
}
impl core::fmt::Debug for Layer0BrPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0BrPos")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0BrPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Layer0BrPos {{ x1: {=u16:?}, y1: {=u16:?} }}",
            self.x1(),
            self.y1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0Config(pub u32);
impl Layer0Config {
    #[doc = "overlay layer input format 3'h0: RGB565 3'h1: RGB888 3'h2: ARGB8888 3'h3: ARGB8565 3'h4: RGB332 3'h5: A8 3'h6: L8 others: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "overlay layer input format 3'h0: RGB565 3'h1: RGB888 3'h2: ARGB8888 3'h3: ARGB8565 3'h4: RGB332 3'h5: A8 3'h6: L8 others: reserved"]
    #[inline(always)]
    pub const fn set_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_sel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub const fn set_alpha_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "layer alpha value"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub const fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "layer color filter enable"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub const fn set_filter_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> u16 {
        let val = (self.0 >> 13usize) & 0x1fff;
        val as u16
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub const fn set_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 13usize)) | (((val as u32) & 0x1fff) << 13usize);
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub const fn set_prefetch_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "line fetch mode 0: address skip every single line 1: address skip every two line"]
    #[must_use]
    #[inline(always)]
    pub const fn line_fetch_mode(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "line fetch mode 0: address skip every single line 1: address skip every two line"]
    #[inline(always)]
    pub const fn set_line_fetch_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "layer active flag"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_blend(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub const fn set_alpha_blend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "set 1 to do vertical mirror for the layer"]
    #[must_use]
    #[inline(always)]
    pub const fn v_mirror(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to do vertical mirror for the layer"]
    #[inline(always)]
    pub const fn set_v_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Layer0Config {
    #[inline(always)]
    fn default() -> Layer0Config {
        Layer0Config(0)
    }
}
impl core::fmt::Debug for Layer0Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0Config")
            .field("format", &self.format())
            .field("alpha_sel", &self.alpha_sel())
            .field("alpha", &self.alpha())
            .field("filter_en", &self.filter_en())
            .field("width", &self.width())
            .field("prefetch_en", &self.prefetch_en())
            .field("line_fetch_mode", &self.line_fetch_mode())
            .field("active", &self.active())
            .field("alpha_blend", &self.alpha_blend())
            .field("v_mirror", &self.v_mirror())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0Config {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Layer0Config {{ format: {=u8:?}, alpha_sel: {=bool:?}, alpha: {=u8:?}, filter_en: {=bool:?}, width: {=u16:?}, prefetch_en: {=bool:?}, line_fetch_mode: {=bool:?}, active: {=bool:?}, alpha_blend: {=bool:?}, v_mirror: {=bool:?} }}" , self . format () , self . alpha_sel () , self . alpha () , self . filter_en () , self . width () , self . prefetch_en () , self . line_fetch_mode () , self . active () , self . alpha_blend () , self . v_mirror ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0Decomp(pub u32);
impl Layer0Decomp {
    #[doc = "decompression enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "decompression enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "size of a single channel data before decompression. Unit is half word. Each line has 3 channels. So for each line, the compressed data size is target_words * 3 * 2 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn target_words(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x0fff;
        val as u16
    }
    #[doc = "size of a single channel data before decompression. Unit is half word. Each line has 3 channels. So for each line, the compressed data size is target_words * 3 * 2 bytes."]
    #[inline(always)]
    pub const fn set_target_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 1usize)) | (((val as u32) & 0x0fff) << 1usize);
    }
    #[doc = "number of colums in a line of original image, max column size is 1024"]
    #[must_use]
    #[inline(always)]
    pub const fn col_size(&self) -> u16 {
        let val = (self.0 >> 13usize) & 0x07ff;
        val as u16
    }
    #[doc = "number of colums in a line of original image, max column size is 1024"]
    #[inline(always)]
    pub const fn set_col_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 13usize)) | (((val as u32) & 0x07ff) << 13usize);
    }
}
impl Default for Layer0Decomp {
    #[inline(always)]
    fn default() -> Layer0Decomp {
        Layer0Decomp(0)
    }
}
impl core::fmt::Debug for Layer0Decomp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0Decomp")
            .field("enable", &self.enable())
            .field("target_words", &self.target_words())
            .field("col_size", &self.col_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0Decomp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Layer0Decomp {{ enable: {=bool:?}, target_words: {=u16:?}, col_size: {=u16:?} }}",
            self.enable(),
            self.target_words(),
            self.col_size()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0DecompCfg0(pub u32);
impl Layer0DecompCfg0 {
    #[doc = "extra bit for high quality bit"]
    #[must_use]
    #[inline(always)]
    pub const fn extra_high(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "extra bit for high quality bit"]
    #[inline(always)]
    pub const fn set_extra_high(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "the threshold to distinguish high/low quality block"]
    #[must_use]
    #[inline(always)]
    pub const fn extra_threshold(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "the threshold to distinguish high/low quality block"]
    #[inline(always)]
    pub const fn set_extra_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "condition to increase qidx"]
    #[must_use]
    #[inline(always)]
    pub const fn use_lossless_qidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "condition to increase qidx"]
    #[inline(always)]
    pub const fn set_use_lossless_qidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "up level for adjusted qidx value for low quality block"]
    #[must_use]
    #[inline(always)]
    pub const fn lossless_qidx1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "up level for adjusted qidx value for low quality block"]
    #[inline(always)]
    pub const fn set_lossless_qidx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "condition to decrease qidx"]
    #[must_use]
    #[inline(always)]
    pub const fn lossless_qidx2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "condition to decrease qidx"]
    #[inline(always)]
    pub const fn set_lossless_qidx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cfg0_reserved(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_cfg0_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Layer0DecompCfg0 {
    #[inline(always)]
    fn default() -> Layer0DecompCfg0 {
        Layer0DecompCfg0(0)
    }
}
impl core::fmt::Debug for Layer0DecompCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0DecompCfg0")
            .field("extra_high", &self.extra_high())
            .field("extra_threshold", &self.extra_threshold())
            .field("use_lossless_qidx", &self.use_lossless_qidx())
            .field("lossless_qidx1", &self.lossless_qidx1())
            .field("lossless_qidx2", &self.lossless_qidx2())
            .field("cfg0_reserved", &self.cfg0_reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0DecompCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Layer0DecompCfg0 {{ extra_high: {=u8:?}, extra_threshold: {=u8:?}, use_lossless_qidx: {=u8:?}, lossless_qidx1: {=u8:?}, lossless_qidx2: {=u8:?}, cfg0_reserved: {=u16:?} }}" , self . extra_high () , self . extra_threshold () , self . use_lossless_qidx () , self . lossless_qidx1 () , self . lossless_qidx2 () , self . cfg0_reserved ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0DecompCfg1(pub u32);
impl Layer0DecompCfg1 {
    #[doc = "block_size in pixel unit. 0: 16 pixels 1: 32 pixels Small block size will cause more blocks and more bits to store block information."]
    #[must_use]
    #[inline(always)]
    pub const fn block_width(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "block_size in pixel unit. 0: 16 pixels 1: 32 pixels Small block size will cause more blocks and more bits to store block information."]
    #[inline(always)]
    pub const fn set_block_width(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "dithering function 0: off 1: on"]
    #[must_use]
    #[inline(always)]
    pub const fn dither(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "dithering function 0: off 1: on"]
    #[inline(always)]
    pub const fn set_dither(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cfg1_reserved(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cfg1_reserved(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "failover compression mode target bits(Red)"]
    #[must_use]
    #[inline(always)]
    pub const fn failover_bits_r(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "failover compression mode target bits(Red)"]
    #[inline(always)]
    pub const fn set_failover_bits_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "failover compression mode target bits(Green)"]
    #[must_use]
    #[inline(always)]
    pub const fn failover_bits_g(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "failover compression mode target bits(Green)"]
    #[inline(always)]
    pub const fn set_failover_bits_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "failover compression mode target bits(Blue)"]
    #[must_use]
    #[inline(always)]
    pub const fn failover_bits_b(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "failover compression mode target bits(Blue)"]
    #[inline(always)]
    pub const fn set_failover_bits_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "minimum qidx for line mode"]
    #[must_use]
    #[inline(always)]
    pub const fn line_min_qidx(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "minimum qidx for line mode"]
    #[inline(always)]
    pub const fn set_line_min_qidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "minimum qidx for block mode"]
    #[must_use]
    #[inline(always)]
    pub const fn block_min_qidx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "minimum qidx for block mode"]
    #[inline(always)]
    pub const fn set_block_min_qidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "extra bit for low quality block"]
    #[must_use]
    #[inline(always)]
    pub const fn extra_low(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "extra bit for low quality block"]
    #[inline(always)]
    pub const fn set_extra_low(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Layer0DecompCfg1 {
    #[inline(always)]
    fn default() -> Layer0DecompCfg1 {
        Layer0DecompCfg1(0)
    }
}
impl core::fmt::Debug for Layer0DecompCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0DecompCfg1")
            .field("block_width", &self.block_width())
            .field("dither", &self.dither())
            .field("cfg1_reserved", &self.cfg1_reserved())
            .field("failover_bits_r", &self.failover_bits_r())
            .field("failover_bits_g", &self.failover_bits_g())
            .field("failover_bits_b", &self.failover_bits_b())
            .field("line_min_qidx", &self.line_min_qidx())
            .field("block_min_qidx", &self.block_min_qidx())
            .field("extra_low", &self.extra_low())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0DecompCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Layer0DecompCfg1 {{ block_width: {=bool:?}, dither: {=bool:?}, cfg1_reserved: {=u8:?}, failover_bits_r: {=u8:?}, failover_bits_g: {=u8:?}, failover_bits_b: {=u8:?}, line_min_qidx: {=u8:?}, block_min_qidx: {=u8:?}, extra_low: {=u8:?} }}" , self . block_width () , self . dither () , self . cfg1_reserved () , self . failover_bits_r () , self . failover_bits_g () , self . failover_bits_b () , self . line_min_qidx () , self . block_min_qidx () , self . extra_low ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0DecompStat(pub u32);
impl Layer0DecompStat {
    #[doc = "buf max usage"]
    #[must_use]
    #[inline(always)]
    pub const fn buf_max_depth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "buf max usage"]
    #[inline(always)]
    pub const fn set_buf_max_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Layer0DecompStat {
    #[inline(always)]
    fn default() -> Layer0DecompStat {
        Layer0DecompStat(0)
    }
}
impl core::fmt::Debug for Layer0DecompStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0DecompStat")
            .field("buf_max_depth", &self.buf_max_depth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0DecompStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Layer0DecompStat {{ buf_max_depth: {=u8:?} }}",
            self.buf_max_depth()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0Fill(pub u32);
impl Layer0Fill {
    #[doc = "background b color"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "background b color"]
    #[inline(always)]
    pub const fn set_bg_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "background g color"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub const fn set_bg_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "background r color"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub const fn set_bg_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "not used"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_mode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "not used"]
    #[inline(always)]
    pub const fn set_bg_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[must_use]
    #[inline(always)]
    pub const fn endian(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub const fn set_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Layer0Fill {
    #[inline(always)]
    fn default() -> Layer0Fill {
        Layer0Fill(0)
    }
}
impl core::fmt::Debug for Layer0Fill {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0Fill")
            .field("bg_b", &self.bg_b())
            .field("bg_g", &self.bg_g())
            .field("bg_r", &self.bg_r())
            .field("bg_mode", &self.bg_mode())
            .field("endian", &self.endian())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0Fill {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Layer0Fill {{ bg_b: {=u8:?}, bg_g: {=u8:?}, bg_r: {=u8:?}, bg_mode: {=bool:?}, endian: {=bool:?} }}" , self . bg_b () , self . bg_g () , self . bg_r () , self . bg_mode () , self . endian ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0Filter(pub u32);
impl Layer0Filter {
    #[doc = "filter b color"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "filter b color"]
    #[inline(always)]
    pub const fn set_filter_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "filter g color"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub const fn set_filter_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "filter r color"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub const fn set_filter_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "layer color filter mask"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_mask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub const fn set_filter_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Layer0Filter {
    #[inline(always)]
    fn default() -> Layer0Filter {
        Layer0Filter(0)
    }
}
impl core::fmt::Debug for Layer0Filter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0Filter")
            .field("filter_b", &self.filter_b())
            .field("filter_g", &self.filter_g())
            .field("filter_r", &self.filter_r())
            .field("filter_mask", &self.filter_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0Filter {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Layer0Filter {{ filter_b: {=u8:?}, filter_g: {=u8:?}, filter_r: {=u8:?}, filter_mask: {=u8:?} }}" , self . filter_b () , self . filter_g () , self . filter_r () , self . filter_mask ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0Src(pub u32);
impl Layer0Src {
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Layer0Src {
    #[inline(always)]
    fn default() -> Layer0Src {
        Layer0Src(0)
    }
}
impl core::fmt::Debug for Layer0Src {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0Src")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0Src {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Layer0Src {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer0TlPos(pub u32);
impl Layer0TlPos {
    #[doc = "Coordinate X-value"]
    #[must_use]
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[must_use]
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Layer0TlPos {
    #[inline(always)]
    fn default() -> Layer0TlPos {
        Layer0TlPos(0)
    }
}
impl core::fmt::Debug for Layer0TlPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer0TlPos")
            .field("x0", &self.x0())
            .field("y0", &self.y0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer0TlPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Layer0TlPos {{ x0: {=u16:?}, y0: {=u16:?} }}",
            self.x0(),
            self.y0()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer1BrPos(pub u32);
impl Layer1BrPos {
    #[doc = "Coordinate X-value"]
    #[must_use]
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[must_use]
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Layer1BrPos {
    #[inline(always)]
    fn default() -> Layer1BrPos {
        Layer1BrPos(0)
    }
}
impl core::fmt::Debug for Layer1BrPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer1BrPos")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer1BrPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Layer1BrPos {{ x1: {=u16:?}, y1: {=u16:?} }}",
            self.x1(),
            self.y1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer1Config(pub u32);
impl Layer1Config {
    #[doc = "overlay layer input format 3'h0: RGB565 3'h1: RGB888 3'h2: ARGB8888 3'h3: ARGB8565 3'h4: RGB332 3'h5: A8 3'h6: L8 others: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "overlay layer input format 3'h0: RGB565 3'h1: RGB888 3'h2: ARGB8888 3'h3: ARGB8565 3'h4: RGB332 3'h5: A8 3'h6: L8 others: reserved"]
    #[inline(always)]
    pub const fn set_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_sel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub const fn set_alpha_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "layer alpha value"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub const fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "layer color filter enable"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub const fn set_filter_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> u16 {
        let val = (self.0 >> 13usize) & 0x1fff;
        val as u16
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub const fn set_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 13usize)) | (((val as u32) & 0x1fff) << 13usize);
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub const fn set_prefetch_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "line fetch mode 0: address skip every single line 1: address skip every two line"]
    #[must_use]
    #[inline(always)]
    pub const fn line_fetch_mode(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "line fetch mode 0: address skip every single line 1: address skip every two line"]
    #[inline(always)]
    pub const fn set_line_fetch_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "layer active flag"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_blend(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub const fn set_alpha_blend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "set 1 to do vertical mirror for the layer"]
    #[must_use]
    #[inline(always)]
    pub const fn v_mirror(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to do vertical mirror for the layer"]
    #[inline(always)]
    pub const fn set_v_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Layer1Config {
    #[inline(always)]
    fn default() -> Layer1Config {
        Layer1Config(0)
    }
}
impl core::fmt::Debug for Layer1Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer1Config")
            .field("format", &self.format())
            .field("alpha_sel", &self.alpha_sel())
            .field("alpha", &self.alpha())
            .field("filter_en", &self.filter_en())
            .field("width", &self.width())
            .field("prefetch_en", &self.prefetch_en())
            .field("line_fetch_mode", &self.line_fetch_mode())
            .field("active", &self.active())
            .field("alpha_blend", &self.alpha_blend())
            .field("v_mirror", &self.v_mirror())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer1Config {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Layer1Config {{ format: {=u8:?}, alpha_sel: {=bool:?}, alpha: {=u8:?}, filter_en: {=bool:?}, width: {=u16:?}, prefetch_en: {=bool:?}, line_fetch_mode: {=bool:?}, active: {=bool:?}, alpha_blend: {=bool:?}, v_mirror: {=bool:?} }}" , self . format () , self . alpha_sel () , self . alpha () , self . filter_en () , self . width () , self . prefetch_en () , self . line_fetch_mode () , self . active () , self . alpha_blend () , self . v_mirror ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer1Fill(pub u32);
impl Layer1Fill {
    #[doc = "background b color"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "background b color"]
    #[inline(always)]
    pub const fn set_bg_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "background g color"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub const fn set_bg_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "background r color"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub const fn set_bg_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "not used"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_mode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "not used"]
    #[inline(always)]
    pub const fn set_bg_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[must_use]
    #[inline(always)]
    pub const fn endian(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub const fn set_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Layer1Fill {
    #[inline(always)]
    fn default() -> Layer1Fill {
        Layer1Fill(0)
    }
}
impl core::fmt::Debug for Layer1Fill {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer1Fill")
            .field("bg_b", &self.bg_b())
            .field("bg_g", &self.bg_g())
            .field("bg_r", &self.bg_r())
            .field("bg_mode", &self.bg_mode())
            .field("endian", &self.endian())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer1Fill {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Layer1Fill {{ bg_b: {=u8:?}, bg_g: {=u8:?}, bg_r: {=u8:?}, bg_mode: {=bool:?}, endian: {=bool:?} }}" , self . bg_b () , self . bg_g () , self . bg_r () , self . bg_mode () , self . endian ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer1Filter(pub u32);
impl Layer1Filter {
    #[doc = "filter b color"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "filter b color"]
    #[inline(always)]
    pub const fn set_filter_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "filter g color"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub const fn set_filter_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "filter r color"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub const fn set_filter_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "layer color filter mask"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_mask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub const fn set_filter_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Layer1Filter {
    #[inline(always)]
    fn default() -> Layer1Filter {
        Layer1Filter(0)
    }
}
impl core::fmt::Debug for Layer1Filter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer1Filter")
            .field("filter_b", &self.filter_b())
            .field("filter_g", &self.filter_g())
            .field("filter_r", &self.filter_r())
            .field("filter_mask", &self.filter_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer1Filter {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Layer1Filter {{ filter_b: {=u8:?}, filter_g: {=u8:?}, filter_r: {=u8:?}, filter_mask: {=u8:?} }}" , self . filter_b () , self . filter_g () , self . filter_r () , self . filter_mask ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer1Src(pub u32);
impl Layer1Src {
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Layer1Src {
    #[inline(always)]
    fn default() -> Layer1Src {
        Layer1Src(0)
    }
}
impl core::fmt::Debug for Layer1Src {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer1Src")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer1Src {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Layer1Src {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer1TlPos(pub u32);
impl Layer1TlPos {
    #[doc = "Coordinate X-value"]
    #[must_use]
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[must_use]
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Layer1TlPos {
    #[inline(always)]
    fn default() -> Layer1TlPos {
        Layer1TlPos(0)
    }
}
impl core::fmt::Debug for Layer1TlPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Layer1TlPos")
            .field("x0", &self.x0())
            .field("y0", &self.y0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Layer1TlPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Layer1TlPos {{ x0: {=u16:?}, y0: {=u16:?} }}",
            self.x0(),
            self.y0()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdConf(pub u32);
impl LcdConf {
    #[doc = "The Data can be sent to four destinations: 2'b00: LCD panel 0 2'b01: LCD panel 1 2'b10: AHB LCD 2'b11: AHB RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn target_lcd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "The Data can be sent to four destinations: 2'b00: LCD panel 0 2'b01: LCD panel 1 2'b10: AHB LCD 2'b11: AHB RAM"]
    #[inline(always)]
    pub const fn set_target_lcd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "3'b000: 8080 DBI Type B 3'b001: SPI interface 3'b010: DBI to DSI interface 3'b011: DPI interface 3'b100: JDI serial interface 3'b101: JDI parallel interface 3'b110: 8080 DBI Type A 3'b111: DPI to DSI interface"]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_intf_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "3'b000: 8080 DBI Type B 3'b001: SPI interface 3'b010: DBI to DSI interface 3'b011: DPI interface 3'b100: JDI serial interface 3'b101: JDI parallel interface 3'b110: 8080 DBI Type A 3'b111: DPI to DSI interface"]
    #[inline(always)]
    pub const fn set_lcd_intf_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "LCD output format: 3'b000: 8-bit RGB 3:3:2 3'b001: 16-bit RGB 5:6:5 over 8-bit bus, 2 cycles/pixel 3'b010: 12-bit RGB 4:4:4 3'b011: 16-bit RGB 5:6:5 3'b100: 18-bit RGB 6:6:6 3'b101: 24-bit RGB 8:8:8 3'b110: 24-bit RGB 8:8:8 over 16-bit bus, 1.5 cycles/pixel 3'b111: 24-bit RGB 8:8:8 over 8-bit bus, 3cycles/pixel others: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_format(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "LCD output format: 3'b000: 8-bit RGB 3:3:2 3'b001: 16-bit RGB 5:6:5 over 8-bit bus, 2 cycles/pixel 3'b010: 12-bit RGB 4:4:4 3'b011: 16-bit RGB 5:6:5 3'b100: 18-bit RGB 6:6:6 3'b101: 24-bit RGB 8:8:8 3'b110: 24-bit RGB 8:8:8 over 16-bit bus, 1.5 cycles/pixel 3'b111: 24-bit RGB 8:8:8 over 8-bit bus, 3cycles/pixel others: Reserved"]
    #[inline(always)]
    pub const fn set_lcd_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "AHB LCD/RAM output format: 0: RGB565 1: RGB888 2: ARGB8888 3: RGB332"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_format(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "AHB LCD/RAM output format: 0: RGB565 1: RGB888 2: ARGB8888 3: RGB332"]
    #[inline(always)]
    pub const fn set_ahb_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "SPI LCD format 2'b00: 8-bit RGB 3:3:2 2'b01: 16-bit RGB 5:6:5 2'b10: 24-bit RGB 8:8:8 2'b11: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_lcd_format(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "SPI LCD format 2'b00: 8-bit RGB 3:3:2 2'b01: 16-bit RGB 5:6:5 2'b10: 24-bit RGB 8:8:8 2'b11: Reserved"]
    #[inline(always)]
    pub const fn set_spi_lcd_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "DPI LCD format 3'b000: 16-bit conf1 3'b001: 16-bit conf2 3'b010: 16-bit conf3 3'b011: 18-bit conf1 3'b100: 18-bit conf2 3'b101: 24-bit others: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_lcd_format(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "DPI LCD format 3'b000: 16-bit conf1 3'b001: 16-bit conf2 3'b010: 16-bit conf3 3'b011: 18-bit conf1 3'b100: 18-bit conf2 3'b101: 24-bit others: Reserved"]
    #[inline(always)]
    pub const fn set_dpi_lcd_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "JDI serial format 2'b00: 3-bit mode 2'b01: 4-bit mode 2'b10: 1-bit mode 2'b11: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn jdi_ser_format(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[doc = "JDI serial format 2'b00: 3-bit mode 2'b01: 4-bit mode 2'b10: 1-bit mode 2'b11: reserved"]
    #[inline(always)]
    pub const fn set_jdi_ser_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
    }
    #[doc = "when the target LCD is AHB LCD, this bit enable the direct interface to DSI module. Direct interface has higher bandwidth and speed than AHB interface."]
    #[must_use]
    #[inline(always)]
    pub const fn direct_intf_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "when the target LCD is AHB LCD, this bit enable the direct interface to DSI module. Direct interface has higher bandwidth and speed than AHB interface."]
    #[inline(always)]
    pub const fn set_direct_intf_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LCD 565 data format endian, this bit would affect SPI, DPI, DBI and AHB interface 565 format 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[must_use]
    #[inline(always)]
    pub const fn endian(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LCD 565 data format endian, this bit would affect SPI, DPI, DBI and AHB interface 565 format 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub const fn set_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "spi read line select. 0: select line 0 1: select line 1 2: select line 2 3: select line 3"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_rd_sel(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "spi read line select. 0: select line 0 1: select line 1 2: select line 2 3: select line 3"]
    #[inline(always)]
    pub const fn set_spi_rd_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
}
impl Default for LcdConf {
    #[inline(always)]
    fn default() -> LcdConf {
        LcdConf(0)
    }
}
impl core::fmt::Debug for LcdConf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LcdConf")
            .field("target_lcd", &self.target_lcd())
            .field("lcd_intf_sel", &self.lcd_intf_sel())
            .field("lcd_format", &self.lcd_format())
            .field("ahb_format", &self.ahb_format())
            .field("spi_lcd_format", &self.spi_lcd_format())
            .field("dpi_lcd_format", &self.dpi_lcd_format())
            .field("jdi_ser_format", &self.jdi_ser_format())
            .field("direct_intf_en", &self.direct_intf_en())
            .field("endian", &self.endian())
            .field("spi_rd_sel", &self.spi_rd_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcdConf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "LcdConf {{ target_lcd: {=u8:?}, lcd_intf_sel: {=u8:?}, lcd_format: {=u8:?}, ahb_format: {=u8:?}, spi_lcd_format: {=u8:?}, dpi_lcd_format: {=u8:?}, jdi_ser_format: {=u8:?}, direct_intf_en: {=bool:?}, endian: {=bool:?}, spi_rd_sel: {=u8:?} }}" , self . target_lcd () , self . lcd_intf_sel () , self . lcd_format () , self . ahb_format () , self . spi_lcd_format () , self . dpi_lcd_format () , self . jdi_ser_format () , self . direct_intf_en () , self . endian () , self . spi_rd_sel ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdIfConf(pub u32);
impl LcdIfConf {
    #[doc = "setup cycles, delay from LCD_CS active to LCD_WR/LCD_RD active"]
    #[must_use]
    #[inline(always)]
    pub const fn tas(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "setup cycles, delay from LCD_CS active to LCD_WR/LCD_RD active"]
    #[inline(always)]
    pub const fn set_tas(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "hold cycles, delay from LCD_WR/LCD_RD inactive to LCD_CS inactive"]
    #[must_use]
    #[inline(always)]
    pub const fn tah(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "hold cycles, delay from LCD_WR/LCD_RD inactive to LCD_CS inactive"]
    #[inline(always)]
    pub const fn set_tah(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "active cycles of LCD_WR/LCD_RD"]
    #[must_use]
    #[inline(always)]
    pub const fn pwl(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "active cycles of LCD_WR/LCD_RD"]
    #[inline(always)]
    pub const fn set_pwl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "inactive cycles of LCD_WR/LCD_RD for consecutive write/read operation"]
    #[must_use]
    #[inline(always)]
    pub const fn pwh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "inactive cycles of LCD_WR/LCD_RD for consecutive write/read operation"]
    #[inline(always)]
    pub const fn set_pwh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "LCD1 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cs0_pol(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LCD1 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1."]
    #[inline(always)]
    pub const fn set_cs0_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LCD0 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cs1_pol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LCD0 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1."]
    #[inline(always)]
    pub const fn set_cs1_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LCD RS pin polarity. RS is 1 for data access, 0 for command access if polarity bit is set as 0. RS bit definition is opposite if polarity bit is set as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rs_pol(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LCD RS pin polarity. RS is 1 for data access, 0 for command access if polarity bit is set as 0. RS bit definition is opposite if polarity bit is set as 1."]
    #[inline(always)]
    pub const fn set_rs_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LCD WR pin polarity. WR is 0 for write operation, 1 for idle if polarity bit is set as 0. WR bit definition is opposite if polarity bit is set as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn wr_pol(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LCD WR pin polarity. WR is 0 for write operation, 1 for idle if polarity bit is set as 0. WR bit definition is opposite if polarity bit is set as 1."]
    #[inline(always)]
    pub const fn set_wr_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LCD RD pin polarity. RD is 0 for write operation, 1 for idle if polarity bit is set as 0. RD bit definition is opposite if polarity bit is set as 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_pol(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LCD RD pin polarity. RD is 0 for write operation, 1 for idle if polarity bit is set as 0. RD bit definition is opposite if polarity bit is set as 1."]
    #[inline(always)]
    pub const fn set_rd_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LCD RSTB pin, direct to output"]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_rstb(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LCD RSTB pin, direct to output"]
    #[inline(always)]
    pub const fn set_lcd_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "if this bit is set to 1, LCD data output will be delayed for 1 lcdc clock cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn do_dly_set(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "if this bit is set to 1, LCD data output will be delayed for 1 lcdc clock cycle"]
    #[inline(always)]
    pub const fn set_do_dly_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "if this bit is set to 1, LCD control output will be delayed for 1 lcdc clock cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl_dly_set(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "if this bit is set to 1, LCD control output will be delayed for 1 lcdc clock cycle"]
    #[inline(always)]
    pub const fn set_ctrl_dly_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for LcdIfConf {
    #[inline(always)]
    fn default() -> LcdIfConf {
        LcdIfConf(0)
    }
}
impl core::fmt::Debug for LcdIfConf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LcdIfConf")
            .field("tas", &self.tas())
            .field("tah", &self.tah())
            .field("pwl", &self.pwl())
            .field("pwh", &self.pwh())
            .field("cs0_pol", &self.cs0_pol())
            .field("cs1_pol", &self.cs1_pol())
            .field("rs_pol", &self.rs_pol())
            .field("wr_pol", &self.wr_pol())
            .field("rd_pol", &self.rd_pol())
            .field("lcd_rstb", &self.lcd_rstb())
            .field("do_dly_set", &self.do_dly_set())
            .field("ctrl_dly_set", &self.ctrl_dly_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcdIfConf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "LcdIfConf {{ tas: {=u8:?}, tah: {=u8:?}, pwl: {=u8:?}, pwh: {=u8:?}, cs0_pol: {=bool:?}, cs1_pol: {=bool:?}, rs_pol: {=bool:?}, wr_pol: {=bool:?}, rd_pol: {=bool:?}, lcd_rstb: {=bool:?}, do_dly_set: {=bool:?}, ctrl_dly_set: {=bool:?} }}" , self . tas () , self . tah () , self . pwl () , self . pwh () , self . cs0_pol () , self . cs1_pol () , self . rs_pol () , self . wr_pol () , self . rd_pol () , self . lcd_rstb () , self . do_dly_set () , self . ctrl_dly_set ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdMem(pub u32);
impl LcdMem {
    #[doc = "address for AHB LCD/AHB RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "address for AHB LCD/AHB RAM"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LcdMem {
    #[inline(always)]
    fn default() -> LcdMem {
        LcdMem(0)
    }
}
impl core::fmt::Debug for LcdMem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LcdMem")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcdMem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LcdMem {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdOWidth(pub u32);
impl LcdOWidth {
    #[doc = "AHB RAM address offset for each line"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AHB RAM address offset for each line"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LcdOWidth {
    #[inline(always)]
    fn default() -> LcdOWidth {
        LcdOWidth(0)
    }
}
impl core::fmt::Debug for LcdOWidth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LcdOWidth")
            .field("offset", &self.offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcdOWidth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LcdOWidth {{ offset: {=u16:?} }}", self.offset())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdRd(pub u32);
impl LcdRd {
    #[doc = "LCD read data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LCD read data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LcdRd {
    #[inline(always)]
    fn default() -> LcdRd {
        LcdRd(0)
    }
}
impl core::fmt::Debug for LcdRd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LcdRd").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcdRd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LcdRd {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdSingle(pub u32);
impl LcdSingle {
    #[doc = "LCD access type, this bit could affect all LCD interface including SPI, parellel and AHB 1'b0: command 1'b1: data"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LCD access type, this bit could affect all LCD interface including SPI, parellel and AHB 1'b0: command 1'b1: data"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Single write operation trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_trig(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Single write operation trigger"]
    #[inline(always)]
    pub const fn set_wr_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Single read operation trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Single read operation trigger"]
    #[inline(always)]
    pub const fn set_rd_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LCD/SPI LCD interface is busy for single access"]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_busy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LCD/SPI LCD interface is busy for single access"]
    #[inline(always)]
    pub const fn set_lcd_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for LcdSingle {
    #[inline(always)]
    fn default() -> LcdSingle {
        LcdSingle(0)
    }
}
impl core::fmt::Debug for LcdSingle {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LcdSingle")
            .field("type_", &self.type_())
            .field("wr_trig", &self.wr_trig())
            .field("rd_trig", &self.rd_trig())
            .field("lcd_busy", &self.lcd_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcdSingle {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "LcdSingle {{ type_: {=bool:?}, wr_trig: {=bool:?}, rd_trig: {=bool:?}, lcd_busy: {=bool:?} }}" , self . type_ () , self . wr_trig () , self . rd_trig () , self . lcd_busy ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdWr(pub u32);
impl LcdWr {
    #[doc = "LCD write data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LCD write data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LcdWr {
    #[inline(always)]
    fn default() -> LcdWr {
        LcdWr(0)
    }
}
impl core::fmt::Debug for LcdWr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LcdWr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcdWr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LcdWr {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemIfStat(pub u32);
impl MemIfStat {
    #[must_use]
    #[inline(always)]
    pub const fn ahb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ahb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn arb_read_port(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_arb_read_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn arb_main(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_arb_main(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
}
impl Default for MemIfStat {
    #[inline(always)]
    fn default() -> MemIfStat {
        MemIfStat(0)
    }
}
impl core::fmt::Debug for MemIfStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MemIfStat")
            .field("ahb", &self.ahb())
            .field("arb_read_port", &self.arb_read_port())
            .field("arb_main", &self.arb_main())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MemIfStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MemIfStat {{ ahb: {=u8:?}, arb_read_port: {=u8:?}, arb_main: {=u8:?} }}",
            self.ahb(),
            self.arb_read_port(),
            self.arb_main()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ol0Stat(pub u32);
impl Ol0Stat {
    #[must_use]
    #[inline(always)]
    pub const fn done_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_done_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_out(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_prefetch_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_read(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_prefetch_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn data_conv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_data_conv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pf_df(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pf_df(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pf_pr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pf_pr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sc_out(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sc_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sc_be(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sc_be(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sc_fe(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sc_fe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sc_lb1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sc_lb1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sc_lb0(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sc_lb0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
}
impl Default for Ol0Stat {
    #[inline(always)]
    fn default() -> Ol0Stat {
        Ol0Stat(0)
    }
}
impl core::fmt::Debug for Ol0Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ol0Stat")
            .field("done_req", &self.done_req())
            .field("prefetch_out", &self.prefetch_out())
            .field("prefetch_read", &self.prefetch_read())
            .field("data_conv", &self.data_conv())
            .field("pf_df", &self.pf_df())
            .field("pf_pr", &self.pf_pr())
            .field("sc_out", &self.sc_out())
            .field("sc_be", &self.sc_be())
            .field("sc_fe", &self.sc_fe())
            .field("sc_lb1", &self.sc_lb1())
            .field("sc_lb0", &self.sc_lb0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ol0Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ol0Stat {{ done_req: {=bool:?}, prefetch_out: {=bool:?}, prefetch_read: {=u8:?}, data_conv: {=u8:?}, pf_df: {=u8:?}, pf_pr: {=u8:?}, sc_out: {=u8:?}, sc_be: {=u8:?}, sc_fe: {=u8:?}, sc_lb1: {=u8:?}, sc_lb0: {=u8:?} }}" , self . done_req () , self . prefetch_out () , self . prefetch_read () , self . data_conv () , self . pf_df () , self . pf_pr () , self . sc_out () , self . sc_be () , self . sc_fe () , self . sc_lb1 () , self . sc_lb0 ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ol1Stat(pub u32);
impl Ol1Stat {
    #[must_use]
    #[inline(always)]
    pub const fn done_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_done_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_out(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_prefetch_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn prefetch_read(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_prefetch_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn data_conv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_data_conv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pf_df(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pf_df(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pf_pr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pf_pr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Ol1Stat {
    #[inline(always)]
    fn default() -> Ol1Stat {
        Ol1Stat(0)
    }
}
impl core::fmt::Debug for Ol1Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ol1Stat")
            .field("done_req", &self.done_req())
            .field("prefetch_out", &self.prefetch_out())
            .field("prefetch_read", &self.prefetch_read())
            .field("data_conv", &self.data_conv())
            .field("pf_df", &self.pf_df())
            .field("pf_pr", &self.pf_pr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ol1Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ol1Stat {{ done_req: {=bool:?}, prefetch_out: {=bool:?}, prefetch_read: {=u8:?}, data_conv: {=u8:?}, pf_df: {=u8:?}, pf_pr: {=u8:?} }}" , self . done_req () , self . prefetch_out () , self . prefetch_read () , self . data_conv () , self . pf_df () , self . pf_pr ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfCnt(pub u32);
impl PerfCnt {
    #[doc = "lcdc performance counter"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "lcdc performance counter"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PerfCnt {
    #[inline(always)]
    fn default() -> PerfCnt {
        PerfCnt(0)
    }
}
impl core::fmt::Debug for PerfCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PerfCnt").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PerfCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PerfCnt {{ val: {=u32:?} }}", self.val())
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd2(pub u32);
impl Rsvd2 {}
impl Default for Rsvd2 {
    #[inline(always)]
    fn default() -> Rsvd2 {
        Rsvd2(0)
    }
}
impl core::fmt::Debug for Rsvd2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd2").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd2 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setting(pub u32);
impl Setting {
    #[doc = "end of frame interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn eof_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "end of frame interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_eof_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "icb overflow interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn icb_of_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "icb overflow interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_icb_of_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "dpi line interrupt, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dpil_intr_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "dpi line interrupt, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_dpil_intr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "dpi under run interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_udr_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "dpi under run interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_dpi_udr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "jdi parallel interface line interrupt, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn jdi_parl_intr_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel interface line interrupt, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_jdi_parl_intr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "jdi parallel interface under run interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn jdi_par_udr_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "jdi parallel interface under run interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_jdi_par_udr_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "line process done interrupt, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn line_done_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "line process done interrupt, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_line_done_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn auto_gate_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_auto_gate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "line number of line process done interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn line_done_num(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "line number of line process done interrupt"]
    #[inline(always)]
    pub const fn set_line_done_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Setting {
    #[inline(always)]
    fn default() -> Setting {
        Setting(0)
    }
}
impl core::fmt::Debug for Setting {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setting")
            .field("eof_mask", &self.eof_mask())
            .field("icb_of_mask", &self.icb_of_mask())
            .field("dpil_intr_mask", &self.dpil_intr_mask())
            .field("dpi_udr_mask", &self.dpi_udr_mask())
            .field("jdi_parl_intr_mask", &self.jdi_parl_intr_mask())
            .field("jdi_par_udr_mask", &self.jdi_par_udr_mask())
            .field("line_done_mask", &self.line_done_mask())
            .field("auto_gate_en", &self.auto_gate_en())
            .field("line_done_num", &self.line_done_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setting {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Setting {{ eof_mask: {=bool:?}, icb_of_mask: {=bool:?}, dpil_intr_mask: {=bool:?}, dpi_udr_mask: {=bool:?}, jdi_parl_intr_mask: {=bool:?}, jdi_par_udr_mask: {=bool:?}, line_done_mask: {=bool:?}, auto_gate_en: {=bool:?}, line_done_num: {=u16:?} }}" , self . eof_mask () , self . icb_of_mask () , self . dpil_intr_mask () , self . dpi_udr_mask () , self . jdi_parl_intr_mask () , self . jdi_par_udr_mask () , self . line_done_mask () , self . auto_gate_en () , self . line_done_num ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiIfConf(pub u32);
impl SpiIfConf {
    #[doc = "SPI line wait cycle, wait cycle is after each line and is according to SPI clock. 0 refers to no wait cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn wait_cycle(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "SPI line wait cycle, wait cycle is after each line and is according to SPI clock. 0 refers to no wait cycle."]
    #[inline(always)]
    pub const fn set_wait_cycle(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "SPI clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_div(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0xff;
        val as u8
    }
    #[doc = "SPI clock divider"]
    #[inline(always)]
    pub const fn set_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
    }
    #[doc = "SPI transaction dummy cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn dummy_cycle(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "SPI transaction dummy cycle"]
    #[inline(always)]
    pub const fn set_dummy_cycle(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[doc = "SPI line mode 0: 4-line 1: 4-line with 2 data line(support RGB565 and RGB888) 2: 4-line with 4 data line(support RGB565 and RGB888) 3: reserved 4: 3-line 5: 3-line with 2 data line(support RGB565 and RGB888) 6: 3-line with 4 data line(support RGB565 and RGB888) 7: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn line(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "SPI line mode 0: 4-line 1: 4-line with 2 data line(support RGB565 and RGB888) 2: 4-line with 4 data line(support RGB565 and RGB888) 3: reserved 4: 3-line 5: 3-line with 2 data line(support RGB565 and RGB888) 6: 3-line with 4 data line(support RGB565 and RGB888) 7: reserved"]
    #[inline(always)]
    pub const fn set_line(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "SPI read data length(single access)"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_len(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "SPI read data length(single access)"]
    #[inline(always)]
    pub const fn set_rd_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "SPI write data length(single access)"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_len(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "SPI write data length(single access)"]
    #[inline(always)]
    pub const fn set_wr_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "SPI read mode: 1'b0: normal read. Send write request before read. 1'b1: direct read. Read data without write request."]
    #[must_use]
    #[inline(always)]
    pub const fn spi_rd_mode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SPI read mode: 1'b0: normal read. Send write request before read. 1'b1: direct read. Read data without write request."]
    #[inline(always)]
    pub const fn set_spi_rd_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "1: SPI clock auto disable in wait state during data transaction 0: SPI clock is always on in wait state during data transaction"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_clk_auto_dis(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "1: SPI clock auto disable in wait state during data transaction 0: SPI clock is always on in wait state during data transaction"]
    #[inline(always)]
    pub const fn set_spi_clk_auto_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "1: SPI CS is always active during data transaction 0: SPI CS is IDLE in wait state during data transaction"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_cs_no_idle(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "1: SPI CS is always active during data transaction 0: SPI CS is IDLE in wait state during data transaction"]
    #[inline(always)]
    pub const fn set_spi_cs_no_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "1: SPI CS is automatically disabled after data transaction 0: SPI CS is not disabled after data transaction"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_cs_auto_dis(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "1: SPI CS is automatically disabled after data transaction 0: SPI CS is not disabled after data transaction"]
    #[inline(always)]
    pub const fn set_spi_cs_auto_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "SPI CS polarity 0: low active 1: high active"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_cs_pol(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "SPI CS polarity 0: low active 1: high active"]
    #[inline(always)]
    pub const fn set_spi_cs_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "SPI CLK polarity 1'h0: normal 1'h1: inverted"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_clk_pol(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SPI CLK polarity 1'h0: normal 1'h1: inverted"]
    #[inline(always)]
    pub const fn set_spi_clk_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "SPI CLK idle state value 1'h0: high 1'h1: low"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_clk_init(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SPI CLK idle state value 1'h0: high 1'h1: low"]
    #[inline(always)]
    pub const fn set_spi_clk_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for SpiIfConf {
    #[inline(always)]
    fn default() -> SpiIfConf {
        SpiIfConf(0)
    }
}
impl core::fmt::Debug for SpiIfConf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpiIfConf")
            .field("wait_cycle", &self.wait_cycle())
            .field("clk_div", &self.clk_div())
            .field("dummy_cycle", &self.dummy_cycle())
            .field("line", &self.line())
            .field("rd_len", &self.rd_len())
            .field("wr_len", &self.wr_len())
            .field("spi_rd_mode", &self.spi_rd_mode())
            .field("spi_clk_auto_dis", &self.spi_clk_auto_dis())
            .field("spi_cs_no_idle", &self.spi_cs_no_idle())
            .field("spi_cs_auto_dis", &self.spi_cs_auto_dis())
            .field("spi_cs_pol", &self.spi_cs_pol())
            .field("spi_clk_pol", &self.spi_clk_pol())
            .field("spi_clk_init", &self.spi_clk_init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpiIfConf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "SpiIfConf {{ wait_cycle: {=u8:?}, clk_div: {=u8:?}, dummy_cycle: {=u8:?}, line: {=u8:?}, rd_len: {=u8:?}, wr_len: {=u8:?}, spi_rd_mode: {=bool:?}, spi_clk_auto_dis: {=bool:?}, spi_cs_no_idle: {=bool:?}, spi_cs_auto_dis: {=bool:?}, spi_cs_pol: {=bool:?}, spi_clk_pol: {=bool:?}, spi_clk_init: {=bool:?} }}" , self . wait_cycle () , self . clk_div () , self . dummy_cycle () , self . line () , self . rd_len () , self . wr_len () , self . spi_rd_mode () , self . spi_clk_auto_dis () , self . spi_cs_no_idle () , self . spi_cs_auto_dis () , self . spi_cs_pol () , self . spi_clk_pol () , self . spi_clk_init ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "LCS controll busy flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lcd_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LCS controll busy flag"]
    #[inline(always)]
    pub const fn set_lcd_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DPI interface is running"]
    #[must_use]
    #[inline(always)]
    pub const fn dpi_run(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DPI interface is running"]
    #[inline(always)]
    pub const fn set_dpi_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "JDI parallel interface is running"]
    #[must_use]
    #[inline(always)]
    pub const fn jdi_par_run(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "JDI parallel interface is running"]
    #[inline(always)]
    pub const fn set_jdi_par_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("lcd_busy", &self.lcd_busy())
            .field("dpi_run", &self.dpi_run())
            .field("jdi_par_run", &self.jdi_par_run())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ lcd_busy: {=bool:?}, dpi_run: {=bool:?}, jdi_par_run: {=bool:?} }}",
            self.lcd_busy(),
            self.dpi_run(),
            self.jdi_par_run()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TeConf(pub u32);
impl TeConf {
    #[doc = "TE enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TE enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TE signal polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn fmark_pol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TE signal polarity"]
    #[inline(always)]
    pub const fn set_fmark_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0: vsync only TE mode 1: vsync+hsync TE mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0: vsync only TE mode 1: vsync+hsync TE mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "vsync signal detect counter, used for mode 1 to detect vsync signal"]
    #[must_use]
    #[inline(always)]
    pub const fn vsync_det_cnt(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0xffff;
        val as u16
    }
    #[doc = "vsync signal detect counter, used for mode 1 to detect vsync signal"]
    #[inline(always)]
    pub const fn set_vsync_det_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 3usize)) | (((val as u32) & 0xffff) << 3usize);
    }
    #[doc = "TE signal trigger mode 1: edge trigger 0: pulse trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn fmark_mode(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TE signal trigger mode 1: edge trigger 0: pulse trigger"]
    #[inline(always)]
    pub const fn set_fmark_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "TE signal source 1: use TE signal from DSI 0: use TE signal from external pin"]
    #[must_use]
    #[inline(always)]
    pub const fn fmark_source(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "TE signal source 1: use TE signal from DSI 0: use TE signal from external pin"]
    #[inline(always)]
    pub const fn set_fmark_source(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for TeConf {
    #[inline(always)]
    fn default() -> TeConf {
        TeConf(0)
    }
}
impl core::fmt::Debug for TeConf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TeConf")
            .field("enable", &self.enable())
            .field("fmark_pol", &self.fmark_pol())
            .field("mode", &self.mode())
            .field("vsync_det_cnt", &self.vsync_det_cnt())
            .field("fmark_mode", &self.fmark_mode())
            .field("fmark_source", &self.fmark_source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TeConf {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TeConf {{ enable: {=bool:?}, fmark_pol: {=bool:?}, mode: {=bool:?}, vsync_det_cnt: {=u16:?}, fmark_mode: {=bool:?}, fmark_source: {=bool:?} }}" , self . enable () , self . fmark_pol () , self . mode () , self . vsync_det_cnt () , self . fmark_mode () , self . fmark_source ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TeConf2(pub u32);
impl TeConf2 {
    #[doc = "TE delay counter"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TE delay counter"]
    #[inline(always)]
    pub const fn set_dly_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TeConf2 {
    #[inline(always)]
    fn default() -> TeConf2 {
        TeConf2(0)
    }
}
impl core::fmt::Debug for TeConf2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TeConf2")
            .field("dly_cnt", &self.dly_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TeConf2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TeConf2 {{ dly_cnt: {=u32:?} }}", self.dly_cnt())
    }
}
