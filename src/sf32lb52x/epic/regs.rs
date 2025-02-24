#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbCtrl(pub u32);
impl AhbCtrl {
    #[doc = "The Data can be sent to two destinations: 2'b0: AHB RAM 2'b1: AHB LCD"]
    #[inline(always)]
    pub const fn destination(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "The Data can be sent to two destinations: 2'b0: AHB RAM 2'b1: AHB LCD"]
    #[inline(always)]
    pub fn set_destination(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB output format: 2'h0: RGB565 2'h1: RGB888 2'h2: ARGB8888 2'h3: ARGB8565"]
    #[inline(always)]
    pub const fn o_format(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "AHB output format: 2'h0: RGB565 2'h1: RGB888 2'h2: ARGB8888 2'h3: ARGB8565"]
    #[inline(always)]
    pub fn set_o_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
}
impl Default for AhbCtrl {
    #[inline(always)]
    fn default() -> AhbCtrl {
        AhbCtrl(0)
    }
}
impl core::fmt::Debug for AhbCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbCtrl")
            .field("destination", &self.destination())
            .field("o_format", &self.o_format())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbCtrl {{ destination: {=bool:?}, o_format: {=u8:?} }}",
            self.destination(),
            self.o_format()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbMem(pub u32);
impl AhbMem {
    #[doc = "AHB RAM/AHB LCD target address"]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AHB RAM/AHB LCD target address"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AhbMem {
    #[inline(always)]
    fn default() -> AhbMem {
        AhbMem(0)
    }
}
impl core::fmt::Debug for AhbMem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbMem")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbMem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AhbMem {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbStride(pub u32);
impl AhbStride {
    #[inline(always)]
    pub const fn offset(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for AhbStride {
    #[inline(always)]
    fn default() -> AhbStride {
        AhbStride(0)
    }
}
impl core::fmt::Debug for AhbStride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbStride")
            .field("offset", &self.offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbStride {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AhbStride {{ offset: {=u16:?} }}", self.offset())
    }
}
#[doc = "Background color"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanvasBg(pub u32);
impl CanvasBg {
    #[doc = "blue color"]
    #[inline(always)]
    pub const fn blue(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "blue color"]
    #[inline(always)]
    pub fn set_blue(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "green color"]
    #[inline(always)]
    pub const fn green(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "green color"]
    #[inline(always)]
    pub fn set_green(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Red color"]
    #[inline(always)]
    pub const fn red(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Red color"]
    #[inline(always)]
    pub fn set_red(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "if this bit is set, the layer is not blending with background. The alpha value will be reserved to output."]
    #[inline(always)]
    pub const fn bg_blending_bypass(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "if this bit is set, the layer is not blending with background. The alpha value will be reserved to output."]
    #[inline(always)]
    pub fn set_bg_blending_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "if this bit is set, epic is in pure dma mode. No blending operation."]
    #[inline(always)]
    pub const fn all_blending_bypass(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "if this bit is set, epic is in pure dma mode. No blending operation."]
    #[inline(always)]
    pub fn set_all_blending_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CanvasBg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CanvasBg {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?}, bg_blending_bypass: {=bool:?}, all_blending_bypass: {=bool:?} }}" , self . blue () , self . green () , self . red () , self . bg_blending_bypass () , self . all_blending_bypass ())
    }
}
#[doc = "Bottom-Right pixel coordinate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanvasBrPos(pub u32);
impl CanvasBrPos {
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
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
pub struct CanvasStat(pub u32);
impl CanvasStat {
    #[doc = "canvas x cordinate"]
    #[inline(always)]
    pub const fn x_cor(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "canvas x cordinate"]
    #[inline(always)]
    pub fn set_x_cor(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "canvas y cordinate"]
    #[inline(always)]
    pub const fn y_cor(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "canvas y cordinate"]
    #[inline(always)]
    pub fn set_y_cor(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[doc = "pre calc fifo count"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "pre calc fifo count"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "postc_status"]
    #[inline(always)]
    pub const fn postc_stat(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[doc = "postc_status"]
    #[inline(always)]
    pub fn set_postc_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
    #[doc = "prec status"]
    #[inline(always)]
    pub const fn prec_stat(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "prec status"]
    #[inline(always)]
    pub fn set_prec_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "fetch status"]
    #[inline(always)]
    pub const fn fetch_stat(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "fetch status"]
    #[inline(always)]
    pub fn set_fetch_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for CanvasStat {
    #[inline(always)]
    fn default() -> CanvasStat {
        CanvasStat(0)
    }
}
impl core::fmt::Debug for CanvasStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CanvasStat")
            .field("x_cor", &self.x_cor())
            .field("y_cor", &self.y_cor())
            .field("fifo_cnt", &self.fifo_cnt())
            .field("postc_stat", &self.postc_stat())
            .field("prec_stat", &self.prec_stat())
            .field("fetch_stat", &self.fetch_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CanvasStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CanvasStat {{ x_cor: {=u16:?}, y_cor: {=u16:?}, fifo_cnt: {=u8:?}, postc_stat: {=u8:?}, prec_stat: {=u8:?}, fetch_stat: {=u8:?} }}" , self . x_cor () , self . y_cor () , self . fifo_cnt () , self . postc_stat () , self . prec_stat () , self . fetch_stat ())
    }
}
#[doc = "Top-Left pixel coordinate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanvasTlPos(pub u32);
impl CanvasTlPos {
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
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
pub struct Coef0(pub u32);
impl Coef0 {
    #[doc = "YUV Fub coef"]
    #[inline(always)]
    pub const fn fub(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "YUV Fub coef"]
    #[inline(always)]
    pub fn set_fub(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "YUV Fug coef"]
    #[inline(always)]
    pub const fn fug(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "YUV Fug coef"]
    #[inline(always)]
    pub fn set_fug(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[doc = "YUV Fy coef"]
    #[inline(always)]
    pub const fn fy(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "YUV Fy coef"]
    #[inline(always)]
    pub fn set_fy(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for Coef0 {
    #[inline(always)]
    fn default() -> Coef0 {
        Coef0(0)
    }
}
impl core::fmt::Debug for Coef0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Coef0")
            .field("fub", &self.fub())
            .field("fug", &self.fug())
            .field("fy", &self.fy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Coef0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Coef0 {{ fub: {=u16:?}, fug: {=u16:?}, fy: {=u16:?} }}",
            self.fub(),
            self.fug(),
            self.fy()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coef1(pub u32);
impl Coef1 {
    #[doc = "YUV Fvg coef"]
    #[inline(always)]
    pub const fn fvg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "YUV Fvg coef"]
    #[inline(always)]
    pub fn set_fvg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "YUV Fvr coef"]
    #[inline(always)]
    pub const fn fvr(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "YUV Fvr coef"]
    #[inline(always)]
    pub fn set_fvr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
}
impl Default for Coef1 {
    #[inline(always)]
    fn default() -> Coef1 {
        Coef1(0)
    }
}
impl core::fmt::Debug for Coef1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Coef1")
            .field("fvg", &self.fvg())
            .field("fvr", &self.fvr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Coef1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Coef1 {{ fvg: {=u16:?}, fvr: {=u16:?} }}",
            self.fvg(),
            self.fvr()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoengCfg(pub u32);
impl CoengCfg {
    #[doc = "ezip enable"]
    #[inline(always)]
    pub const fn ezip_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ezip enable"]
    #[inline(always)]
    pub fn set_ezip_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ezip channel select"]
    #[inline(always)]
    pub const fn ezip_ch_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "ezip channel select"]
    #[inline(always)]
    pub fn set_ezip_ch_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "yuv enable"]
    #[inline(always)]
    pub const fn yuv_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "yuv enable"]
    #[inline(always)]
    pub fn set_yuv_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "yuv engine channel select"]
    #[inline(always)]
    pub const fn yuv_ch_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "yuv engine channel select"]
    #[inline(always)]
    pub fn set_yuv_ch_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for CoengCfg {
    #[inline(always)]
    fn default() -> CoengCfg {
        CoengCfg(0)
    }
}
impl core::fmt::Debug for CoengCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CoengCfg")
            .field("ezip_en", &self.ezip_en())
            .field("ezip_ch_sel", &self.ezip_ch_sel())
            .field("yuv_en", &self.yuv_en())
            .field("yuv_ch_sel", &self.yuv_ch_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CoengCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CoengCfg {{ ezip_en: {=bool:?}, ezip_ch_sel: {=u8:?}, yuv_en: {=bool:?}, yuv_ch_sel: {=u8:?} }}" , self . ezip_en () , self . ezip_ch_sel () , self . yuv_en () , self . yuv_ch_sel ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Command(pub u32);
impl Command {
    #[doc = "write 1 to trigger the lcd interface block"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to trigger the lcd interface block"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: reset the whole graphics 0: release the reset"]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: reset the whole graphics 0: release the reset"]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
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
pub struct Debug(pub u32);
impl Debug {
    #[inline(always)]
    pub const fn debug_out_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_debug_out_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn debug_int_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_debug_int_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "4'h0: RSVD 4'h1: OL0 debug info 4'h2: OL1 debug info 4'h3: OL2 debug info 4'h4: VL debug info1 4'h5: VL debug info2 4'h6: ROI debug out 4'h7: mem intfa debug out 4'h8: mem intfb debug out 4'h9: ahb ctrl debug out 4'ha: ROI XX 4'hb: ROI YY 4'hc: EPIC_EZIP debug out others: RSVD"]
    #[inline(always)]
    pub const fn debug_int_data(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "4'h0: RSVD 4'h1: OL0 debug info 4'h2: OL1 debug info 4'h3: OL2 debug info 4'h4: VL debug info1 4'h5: VL debug info2 4'h6: ROI debug out 4'h7: mem intfa debug out 4'h8: mem intfb debug out 4'h9: ahb ctrl debug out 4'ha: ROI XX 4'hb: ROI YY 4'hc: EPIC_EZIP debug out others: RSVD"]
    #[inline(always)]
    pub fn set_debug_int_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Debug {
    #[inline(always)]
    fn default() -> Debug {
        Debug(0)
    }
}
impl core::fmt::Debug for Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug")
            .field("debug_out_sel", &self.debug_out_sel())
            .field("debug_int_sel", &self.debug_int_sel())
            .field("debug_int_data", &self.debug_int_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug {{ debug_out_sel: {=u8:?}, debug_int_sel: {=u8:?}, debug_int_data: {=u16:?} }}",
            self.debug_out_sel(),
            self.debug_int_sel(),
            self.debug_int_data()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DitherConf(pub u32);
impl DitherConf {
    #[doc = "dither enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "dither enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "blue dither width"]
    #[inline(always)]
    pub const fn w_b(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "blue dither width"]
    #[inline(always)]
    pub fn set_w_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "green dither width"]
    #[inline(always)]
    pub const fn w_g(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "green dither width"]
    #[inline(always)]
    pub fn set_w_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "red dither width"]
    #[inline(always)]
    pub const fn w_r(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "red dither width"]
    #[inline(always)]
    pub fn set_w_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "select lfsr 0: none 1: red 2: green 3: blue"]
    #[inline(always)]
    pub const fn lfsr_load_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "select lfsr 0: none 1: red 2: green 3: blue"]
    #[inline(always)]
    pub fn set_lfsr_load_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "load lfsr init value"]
    #[inline(always)]
    pub const fn lfsr_load(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "load lfsr init value"]
    #[inline(always)]
    pub fn set_lfsr_load(&mut self, val: bool) {
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
    #[inline(always)]
    pub const fn init_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "lfsr init load value"]
    #[inline(always)]
    pub fn set_init_val(&mut self, val: u32) {
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
pub struct EofIrq(pub u32);
impl EofIrq {
    #[doc = "end of frame interrupt, can be masked by EOF_IRQ_MASK"]
    #[inline(always)]
    pub const fn irq_cause(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "end of frame interrupt, can be masked by EOF_IRQ_MASK"]
    #[inline(always)]
    pub fn set_irq_cause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "line hit interrupt, can be masked by LINE_IRQ_MASK"]
    #[inline(always)]
    pub const fn line_hit_cause(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "line hit interrupt, can be masked by LINE_IRQ_MASK"]
    #[inline(always)]
    pub fn set_line_hit_cause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "raw status of end of frame interrupt"]
    #[inline(always)]
    pub const fn irq_status(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "raw status of end of frame interrupt"]
    #[inline(always)]
    pub fn set_irq_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "raw status of line hit interrupt"]
    #[inline(always)]
    pub const fn line_hit_status(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "raw status of line hit interrupt"]
    #[inline(always)]
    pub fn set_line_hit_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for EofIrq {
    #[inline(always)]
    fn default() -> EofIrq {
        EofIrq(0)
    }
}
impl core::fmt::Debug for EofIrq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EofIrq")
            .field("irq_cause", &self.irq_cause())
            .field("line_hit_cause", &self.line_hit_cause())
            .field("irq_status", &self.irq_status())
            .field("line_hit_status", &self.line_hit_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EofIrq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "EofIrq {{ irq_cause: {=bool:?}, line_hit_cause: {=bool:?}, irq_status: {=bool:?}, line_hit_status: {=bool:?} }}" , self . irq_cause () , self . line_hit_cause () , self . irq_status () , self . line_hit_status ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EzipStat(pub u32);
impl EzipStat {
    #[doc = "ezip engine 0 line count"]
    #[inline(always)]
    pub const fn line_cnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "ezip engine 0 line count"]
    #[inline(always)]
    pub fn set_line_cnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "ezip engine 0 buffer count"]
    #[inline(always)]
    pub const fn buf_cnt0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "ezip engine 0 buffer count"]
    #[inline(always)]
    pub fn set_buf_cnt0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "ezip engine 0 status"]
    #[inline(always)]
    pub const fn run_stat0(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "ezip engine 0 status"]
    #[inline(always)]
    pub fn set_run_stat0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "ezip engine 1 line count"]
    #[inline(always)]
    pub const fn line_cnt1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "ezip engine 1 line count"]
    #[inline(always)]
    pub fn set_line_cnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "ezip engine 1 buffer count"]
    #[inline(always)]
    pub const fn buf_cnt1(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "ezip engine 1 buffer count"]
    #[inline(always)]
    pub fn set_buf_cnt1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "ezip engine 1 status"]
    #[inline(always)]
    pub const fn run_stat1(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "ezip engine 1 status"]
    #[inline(always)]
    pub fn set_run_stat1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for EzipStat {
    #[inline(always)]
    fn default() -> EzipStat {
        EzipStat(0)
    }
}
impl core::fmt::Debug for EzipStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EzipStat")
            .field("line_cnt0", &self.line_cnt0())
            .field("buf_cnt0", &self.buf_cnt0())
            .field("run_stat0", &self.run_stat0())
            .field("line_cnt1", &self.line_cnt1())
            .field("buf_cnt1", &self.buf_cnt1())
            .field("run_stat1", &self.run_stat1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EzipStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "EzipStat {{ line_cnt0: {=u16:?}, buf_cnt0: {=u8:?}, run_stat0: {=u8:?}, line_cnt1: {=u16:?}, buf_cnt1: {=u8:?}, run_stat1: {=u8:?} }}" , self . line_cnt0 () , self . buf_cnt0 () , self . run_stat0 () , self . line_cnt1 () , self . buf_cnt1 () , self . run_stat1 ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0BrPos(pub u32);
impl L0BrPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for L0BrPos {
    #[inline(always)]
    fn default() -> L0BrPos {
        L0BrPos(0)
    }
}
impl core::fmt::Debug for L0BrPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0BrPos")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0BrPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L0BrPos {{ x1: {=u16:?}, y1: {=u16:?} }}",
            self.x1(),
            self.y1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0Cfg(pub u32);
impl L0Cfg {
    #[doc = "layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2"]
    #[inline(always)]
    pub const fn format(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2"]
    #[inline(always)]
    pub fn set_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub const fn alpha_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub fn set_alpha_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub const fn filter_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub fn set_filter_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub const fn width(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub fn set_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub const fn prefetch_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub fn set_prefetch_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub const fn alpha_blend(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub fn set_alpha_blend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L0Cfg {
    #[inline(always)]
    fn default() -> L0Cfg {
        L0Cfg(0)
    }
}
impl core::fmt::Debug for L0Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0Cfg")
            .field("format", &self.format())
            .field("alpha_sel", &self.alpha_sel())
            .field("alpha", &self.alpha())
            .field("filter_en", &self.filter_en())
            .field("width", &self.width())
            .field("prefetch_en", &self.prefetch_en())
            .field("active", &self.active())
            .field("alpha_blend", &self.alpha_blend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L0Cfg {{ format: {=u8:?}, alpha_sel: {=bool:?}, alpha: {=u8:?}, filter_en: {=bool:?}, width: {=u16:?}, prefetch_en: {=bool:?}, active: {=bool:?}, alpha_blend: {=bool:?} }}" , self . format () , self . alpha_sel () , self . alpha () , self . filter_en () , self . width () , self . prefetch_en () , self . active () , self . alpha_blend ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0Fill(pub u32);
impl L0Fill {
    #[doc = "background b color"]
    #[inline(always)]
    pub const fn bg_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "background b color"]
    #[inline(always)]
    pub fn set_bg_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub const fn bg_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub fn set_bg_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub const fn bg_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub fn set_bg_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Not used."]
    #[inline(always)]
    pub const fn bg_mode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Not used."]
    #[inline(always)]
    pub fn set_bg_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub const fn endian(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub fn set_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for L0Fill {
    #[inline(always)]
    fn default() -> L0Fill {
        L0Fill(0)
    }
}
impl core::fmt::Debug for L0Fill {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0Fill")
            .field("bg_b", &self.bg_b())
            .field("bg_g", &self.bg_g())
            .field("bg_r", &self.bg_r())
            .field("bg_mode", &self.bg_mode())
            .field("endian", &self.endian())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0Fill {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L0Fill {{ bg_b: {=u8:?}, bg_g: {=u8:?}, bg_r: {=u8:?}, bg_mode: {=bool:?}, endian: {=bool:?} }}" , self . bg_b () , self . bg_g () , self . bg_r () , self . bg_mode () , self . endian ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0Filter(pub u32);
impl L0Filter {
    #[doc = "filter b color"]
    #[inline(always)]
    pub const fn filter_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "filter b color"]
    #[inline(always)]
    pub fn set_filter_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub const fn filter_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub fn set_filter_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub const fn filter_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub fn set_filter_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub const fn filter_mask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub fn set_filter_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for L0Filter {
    #[inline(always)]
    fn default() -> L0Filter {
        L0Filter(0)
    }
}
impl core::fmt::Debug for L0Filter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0Filter")
            .field("filter_b", &self.filter_b())
            .field("filter_g", &self.filter_g())
            .field("filter_r", &self.filter_r())
            .field("filter_mask", &self.filter_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0Filter {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L0Filter {{ filter_b: {=u8:?}, filter_g: {=u8:?}, filter_r: {=u8:?}, filter_mask: {=u8:?} }}" , self . filter_b () , self . filter_g () , self . filter_r () , self . filter_mask ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0MiscCfg(pub u32);
impl L0MiscCfg {
    #[doc = "L0 CLUT select: 1'h1: select pallette1 1'h0: select pallette0"]
    #[inline(always)]
    pub const fn clut_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "L0 CLUT select: 1'h1: select pallette1 1'h0: select pallette0"]
    #[inline(always)]
    pub fn set_clut_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "vertical mirror enable"]
    #[inline(always)]
    pub const fn v_mirror(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "vertical mirror enable"]
    #[inline(always)]
    pub fn set_v_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for L0MiscCfg {
    #[inline(always)]
    fn default() -> L0MiscCfg {
        L0MiscCfg(0)
    }
}
impl core::fmt::Debug for L0MiscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0MiscCfg")
            .field("clut_sel", &self.clut_sel())
            .field("v_mirror", &self.v_mirror())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0MiscCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L0MiscCfg {{ clut_sel: {=bool:?}, v_mirror: {=bool:?} }}",
            self.clut_sel(),
            self.v_mirror()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0Src(pub u32);
impl L0Src {
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for L0Src {
    #[inline(always)]
    fn default() -> L0Src {
        L0Src(0)
    }
}
impl core::fmt::Debug for L0Src {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0Src").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0Src {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "L0Src {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0TlPos(pub u32);
impl L0TlPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for L0TlPos {
    #[inline(always)]
    fn default() -> L0TlPos {
        L0TlPos(0)
    }
}
impl core::fmt::Debug for L0TlPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0TlPos")
            .field("x0", &self.x0())
            .field("y0", &self.y0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0TlPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L0TlPos {{ x0: {=u16:?}, y0: {=u16:?} }}",
            self.x0(),
            self.y0()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1BrPos(pub u32);
impl L1BrPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for L1BrPos {
    #[inline(always)]
    fn default() -> L1BrPos {
        L1BrPos(0)
    }
}
impl core::fmt::Debug for L1BrPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1BrPos")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1BrPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L1BrPos {{ x1: {=u16:?}, y1: {=u16:?} }}",
            self.x1(),
            self.y1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1Cfg(pub u32);
impl L1Cfg {
    #[doc = "layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2"]
    #[inline(always)]
    pub const fn format(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2"]
    #[inline(always)]
    pub fn set_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub const fn alpha_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub fn set_alpha_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub const fn filter_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub fn set_filter_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub const fn width(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub fn set_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub const fn prefetch_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub fn set_prefetch_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub const fn alpha_blend(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub fn set_alpha_blend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L1Cfg {
    #[inline(always)]
    fn default() -> L1Cfg {
        L1Cfg(0)
    }
}
impl core::fmt::Debug for L1Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1Cfg")
            .field("format", &self.format())
            .field("alpha_sel", &self.alpha_sel())
            .field("alpha", &self.alpha())
            .field("filter_en", &self.filter_en())
            .field("width", &self.width())
            .field("prefetch_en", &self.prefetch_en())
            .field("active", &self.active())
            .field("alpha_blend", &self.alpha_blend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L1Cfg {{ format: {=u8:?}, alpha_sel: {=bool:?}, alpha: {=u8:?}, filter_en: {=bool:?}, width: {=u16:?}, prefetch_en: {=bool:?}, active: {=bool:?}, alpha_blend: {=bool:?} }}" , self . format () , self . alpha_sel () , self . alpha () , self . filter_en () , self . width () , self . prefetch_en () , self . active () , self . alpha_blend ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1Fill(pub u32);
impl L1Fill {
    #[doc = "background b color"]
    #[inline(always)]
    pub const fn bg_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "background b color"]
    #[inline(always)]
    pub fn set_bg_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub const fn bg_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub fn set_bg_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub const fn bg_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub fn set_bg_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Not used."]
    #[inline(always)]
    pub const fn bg_mode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Not used."]
    #[inline(always)]
    pub fn set_bg_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub const fn endian(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub fn set_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for L1Fill {
    #[inline(always)]
    fn default() -> L1Fill {
        L1Fill(0)
    }
}
impl core::fmt::Debug for L1Fill {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1Fill")
            .field("bg_b", &self.bg_b())
            .field("bg_g", &self.bg_g())
            .field("bg_r", &self.bg_r())
            .field("bg_mode", &self.bg_mode())
            .field("endian", &self.endian())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1Fill {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L1Fill {{ bg_b: {=u8:?}, bg_g: {=u8:?}, bg_r: {=u8:?}, bg_mode: {=bool:?}, endian: {=bool:?} }}" , self . bg_b () , self . bg_g () , self . bg_r () , self . bg_mode () , self . endian ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1Filter(pub u32);
impl L1Filter {
    #[doc = "filter b color"]
    #[inline(always)]
    pub const fn filter_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "filter b color"]
    #[inline(always)]
    pub fn set_filter_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub const fn filter_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub fn set_filter_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub const fn filter_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub fn set_filter_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub const fn filter_mask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub fn set_filter_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for L1Filter {
    #[inline(always)]
    fn default() -> L1Filter {
        L1Filter(0)
    }
}
impl core::fmt::Debug for L1Filter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1Filter")
            .field("filter_b", &self.filter_b())
            .field("filter_g", &self.filter_g())
            .field("filter_r", &self.filter_r())
            .field("filter_mask", &self.filter_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1Filter {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L1Filter {{ filter_b: {=u8:?}, filter_g: {=u8:?}, filter_r: {=u8:?}, filter_mask: {=u8:?} }}" , self . filter_b () , self . filter_g () , self . filter_r () , self . filter_mask ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1MiscCfg(pub u32);
impl L1MiscCfg {
    #[doc = "L1 CLUT select: 1'h1: select pallette1 1'h0: select pallette0"]
    #[inline(always)]
    pub const fn clut_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "L1 CLUT select: 1'h1: select pallette1 1'h0: select pallette0"]
    #[inline(always)]
    pub fn set_clut_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "vertical mirror enable"]
    #[inline(always)]
    pub const fn v_mirror(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "vertical mirror enable"]
    #[inline(always)]
    pub fn set_v_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for L1MiscCfg {
    #[inline(always)]
    fn default() -> L1MiscCfg {
        L1MiscCfg(0)
    }
}
impl core::fmt::Debug for L1MiscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1MiscCfg")
            .field("clut_sel", &self.clut_sel())
            .field("v_mirror", &self.v_mirror())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1MiscCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L1MiscCfg {{ clut_sel: {=bool:?}, v_mirror: {=bool:?} }}",
            self.clut_sel(),
            self.v_mirror()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1Src(pub u32);
impl L1Src {
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for L1Src {
    #[inline(always)]
    fn default() -> L1Src {
        L1Src(0)
    }
}
impl core::fmt::Debug for L1Src {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1Src").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1Src {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "L1Src {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1TlPos(pub u32);
impl L1TlPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for L1TlPos {
    #[inline(always)]
    fn default() -> L1TlPos {
        L1TlPos(0)
    }
}
impl core::fmt::Debug for L1TlPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1TlPos")
            .field("x0", &self.x0())
            .field("y0", &self.y0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1TlPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L1TlPos {{ x0: {=u16:?}, y0: {=u16:?} }}",
            self.x0(),
            self.y0()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2BrPos(pub u32);
impl L2BrPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for L2BrPos {
    #[inline(always)]
    fn default() -> L2BrPos {
        L2BrPos(0)
    }
}
impl core::fmt::Debug for L2BrPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2BrPos")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2BrPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L2BrPos {{ x1: {=u16:?}, y1: {=u16:?} }}",
            self.x1(),
            self.y1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2Cfg(pub u32);
impl L2Cfg {
    #[doc = "layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2"]
    #[inline(always)]
    pub const fn format(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2"]
    #[inline(always)]
    pub fn set_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub const fn alpha_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub fn set_alpha_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub const fn filter_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub fn set_filter_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub const fn width(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub fn set_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub const fn prefetch_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub fn set_prefetch_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub const fn alpha_blend(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub fn set_alpha_blend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L2Cfg {
    #[inline(always)]
    fn default() -> L2Cfg {
        L2Cfg(0)
    }
}
impl core::fmt::Debug for L2Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2Cfg")
            .field("format", &self.format())
            .field("alpha_sel", &self.alpha_sel())
            .field("alpha", &self.alpha())
            .field("filter_en", &self.filter_en())
            .field("width", &self.width())
            .field("prefetch_en", &self.prefetch_en())
            .field("active", &self.active())
            .field("alpha_blend", &self.alpha_blend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L2Cfg {{ format: {=u8:?}, alpha_sel: {=bool:?}, alpha: {=u8:?}, filter_en: {=bool:?}, width: {=u16:?}, prefetch_en: {=bool:?}, active: {=bool:?}, alpha_blend: {=bool:?} }}" , self . format () , self . alpha_sel () , self . alpha () , self . filter_en () , self . width () , self . prefetch_en () , self . active () , self . alpha_blend ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2Extents(pub u32);
impl L2Extents {
    #[doc = "number of pixels of each column of source image(not including padding)"]
    #[inline(always)]
    pub const fn max_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "number of pixels of each column of source image(not including padding)"]
    #[inline(always)]
    pub fn set_max_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "number of pixels of each line of source image(not including padding)"]
    #[inline(always)]
    pub const fn max_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "number of pixels of each line of source image(not including padding)"]
    #[inline(always)]
    pub fn set_max_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for L2Extents {
    #[inline(always)]
    fn default() -> L2Extents {
        L2Extents(0)
    }
}
impl core::fmt::Debug for L2Extents {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2Extents")
            .field("max_line", &self.max_line())
            .field("max_col", &self.max_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2Extents {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L2Extents {{ max_line: {=u16:?}, max_col: {=u16:?} }}",
            self.max_line(),
            self.max_col()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2Fill(pub u32);
impl L2Fill {
    #[doc = "background b color"]
    #[inline(always)]
    pub const fn bg_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "background b color"]
    #[inline(always)]
    pub fn set_bg_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub const fn bg_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub fn set_bg_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub const fn bg_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub fn set_bg_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Not used."]
    #[inline(always)]
    pub const fn bg_mode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Not used."]
    #[inline(always)]
    pub fn set_bg_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub const fn endian(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub fn set_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for L2Fill {
    #[inline(always)]
    fn default() -> L2Fill {
        L2Fill(0)
    }
}
impl core::fmt::Debug for L2Fill {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2Fill")
            .field("bg_b", &self.bg_b())
            .field("bg_g", &self.bg_g())
            .field("bg_r", &self.bg_r())
            .field("bg_mode", &self.bg_mode())
            .field("endian", &self.endian())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2Fill {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L2Fill {{ bg_b: {=u8:?}, bg_g: {=u8:?}, bg_r: {=u8:?}, bg_mode: {=bool:?}, endian: {=bool:?} }}" , self . bg_b () , self . bg_g () , self . bg_r () , self . bg_mode () , self . endian ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2Filter(pub u32);
impl L2Filter {
    #[doc = "filter b color"]
    #[inline(always)]
    pub const fn filter_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "filter b color"]
    #[inline(always)]
    pub fn set_filter_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub const fn filter_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub fn set_filter_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub const fn filter_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub fn set_filter_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub const fn filter_mask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub fn set_filter_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for L2Filter {
    #[inline(always)]
    fn default() -> L2Filter {
        L2Filter(0)
    }
}
impl core::fmt::Debug for L2Filter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2Filter")
            .field("filter_b", &self.filter_b())
            .field("filter_g", &self.filter_g())
            .field("filter_r", &self.filter_r())
            .field("filter_mask", &self.filter_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2Filter {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L2Filter {{ filter_b: {=u8:?}, filter_g: {=u8:?}, filter_r: {=u8:?}, filter_mask: {=u8:?} }}" , self . filter_b () , self . filter_g () , self . filter_r () , self . filter_mask ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2MiscCfg(pub u32);
impl L2MiscCfg {
    #[doc = "L2 CLUT select: 1'h1: select pallette1 1'h0: select pallette0"]
    #[inline(always)]
    pub const fn clut_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "L2 CLUT select: 1'h1: select pallette1 1'h0: select pallette0"]
    #[inline(always)]
    pub fn set_clut_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "vertical mirror enable"]
    #[inline(always)]
    pub const fn v_mirror(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "vertical mirror enable"]
    #[inline(always)]
    pub fn set_v_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "horizontal mirror enable"]
    #[inline(always)]
    pub const fn h_mirror(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "horizontal mirror enable"]
    #[inline(always)]
    pub fn set_h_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "external absolute value of cos. U13.12 format."]
    #[inline(always)]
    pub const fn cos_force_value(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x1fff;
        val as u16
    }
    #[doc = "external absolute value of cos. U13.12 format."]
    #[inline(always)]
    pub fn set_cos_force_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
    }
    #[doc = "external absolute value of sin. U13.12 format."]
    #[inline(always)]
    pub const fn sin_force_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "external absolute value of sin. U13.12 format."]
    #[inline(always)]
    pub fn set_sin_force_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
    #[doc = "force epic use external sin and cos value, quadrant is still calculated from ROT_DEG."]
    #[inline(always)]
    pub const fn deg_force(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "force epic use external sin and cos value, quadrant is still calculated from ROT_DEG."]
    #[inline(always)]
    pub fn set_deg_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for L2MiscCfg {
    #[inline(always)]
    fn default() -> L2MiscCfg {
        L2MiscCfg(0)
    }
}
impl core::fmt::Debug for L2MiscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2MiscCfg")
            .field("clut_sel", &self.clut_sel())
            .field("v_mirror", &self.v_mirror())
            .field("h_mirror", &self.h_mirror())
            .field("cos_force_value", &self.cos_force_value())
            .field("sin_force_value", &self.sin_force_value())
            .field("deg_force", &self.deg_force())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2MiscCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L2MiscCfg {{ clut_sel: {=bool:?}, v_mirror: {=bool:?}, h_mirror: {=bool:?}, cos_force_value: {=u16:?}, sin_force_value: {=u16:?}, deg_force: {=bool:?} }}" , self . clut_sel () , self . v_mirror () , self . h_mirror () , self . cos_force_value () , self . sin_force_value () , self . deg_force ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2Rot(pub u32);
impl L2Rot {
    #[doc = "rot_max_col and rot_max_line calculation request. Write 1 to trigger the calculation."]
    #[inline(always)]
    pub const fn calc_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "rot_max_col and rot_max_line calculation request. Write 1 to trigger the calculation."]
    #[inline(always)]
    pub fn set_calc_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "rot_max_col and rot_max_line calculation clear request. Write 1 to clear the result."]
    #[inline(always)]
    pub const fn calc_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "rot_max_col and rot_max_line calculation clear request. Write 1 to clear the result."]
    #[inline(always)]
    pub fn set_calc_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "rotation degree, rotation is clockwise."]
    #[inline(always)]
    pub const fn rot_deg(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x01ff;
        val as u16
    }
    #[doc = "rotation degree, rotation is clockwise."]
    #[inline(always)]
    pub fn set_rot_deg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
    }
    #[doc = "calculation done indicator"]
    #[inline(always)]
    pub const fn calc_done(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "calculation done indicator"]
    #[inline(always)]
    pub fn set_calc_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for L2Rot {
    #[inline(always)]
    fn default() -> L2Rot {
        L2Rot(0)
    }
}
impl core::fmt::Debug for L2Rot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2Rot")
            .field("calc_req", &self.calc_req())
            .field("calc_clr", &self.calc_clr())
            .field("rot_deg", &self.rot_deg())
            .field("calc_done", &self.calc_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2Rot {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "L2Rot {{ calc_req: {=bool:?}, calc_clr: {=bool:?}, rot_deg: {=u16:?}, calc_done: {=bool:?} }}" , self . calc_req () , self . calc_clr () , self . rot_deg () , self . calc_done ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2RotMCfg1(pub u32);
impl L2RotMCfg1 {
    #[doc = "manual mode rotation max line, unsigned value"]
    #[inline(always)]
    pub const fn m_rot_max_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode rotation max line, unsigned value"]
    #[inline(always)]
    pub fn set_m_rot_max_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "manual mode rotation max column, unsigned value"]
    #[inline(always)]
    pub const fn m_rot_max_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode rotation max column, unsigned value"]
    #[inline(always)]
    pub fn set_m_rot_max_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[doc = "rotation mode setting 1'b0: auto mode 1'b1: manual mode"]
    #[inline(always)]
    pub const fn m_mode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "rotation mode setting 1'b0: auto mode 1'b1: manual mode"]
    #[inline(always)]
    pub fn set_m_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L2RotMCfg1 {
    #[inline(always)]
    fn default() -> L2RotMCfg1 {
        L2RotMCfg1(0)
    }
}
impl core::fmt::Debug for L2RotMCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2RotMCfg1")
            .field("m_rot_max_line", &self.m_rot_max_line())
            .field("m_rot_max_col", &self.m_rot_max_col())
            .field("m_mode", &self.m_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2RotMCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L2RotMCfg1 {{ m_rot_max_line: {=u16:?}, m_rot_max_col: {=u16:?}, m_mode: {=bool:?} }}",
            self.m_rot_max_line(),
            self.m_rot_max_col(),
            self.m_mode()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2RotMCfg2(pub u32);
impl L2RotMCfg2 {
    #[doc = "manual mode pivot x, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub const fn m_pivot_x(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode pivot x, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub fn set_m_pivot_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "manual mode pivot y, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub const fn m_pivot_y(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode pivot y, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub fn set_m_pivot_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for L2RotMCfg2 {
    #[inline(always)]
    fn default() -> L2RotMCfg2 {
        L2RotMCfg2(0)
    }
}
impl core::fmt::Debug for L2RotMCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2RotMCfg2")
            .field("m_pivot_x", &self.m_pivot_x())
            .field("m_pivot_y", &self.m_pivot_y())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2RotMCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L2RotMCfg2 {{ m_pivot_x: {=u16:?}, m_pivot_y: {=u16:?} }}",
            self.m_pivot_x(),
            self.m_pivot_y()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2RotMCfg3(pub u32);
impl L2RotMCfg3 {
    #[doc = "manual mode top left x cordinate, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub const fn m_xtl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode top left x cordinate, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub fn set_m_xtl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "manual mode top left y cordinate, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub const fn m_ytl(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode top left y cordinate, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub fn set_m_ytl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for L2RotMCfg3 {
    #[inline(always)]
    fn default() -> L2RotMCfg3 {
        L2RotMCfg3(0)
    }
}
impl core::fmt::Debug for L2RotMCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2RotMCfg3")
            .field("m_xtl", &self.m_xtl())
            .field("m_ytl", &self.m_ytl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2RotMCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L2RotMCfg3 {{ m_xtl: {=u16:?}, m_ytl: {=u16:?} }}",
            self.m_xtl(),
            self.m_ytl()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2RotStat(pub u32);
impl L2RotStat {
    #[doc = "max line of rotated image"]
    #[inline(always)]
    pub const fn rot_max_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "max line of rotated image"]
    #[inline(always)]
    pub fn set_rot_max_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "max column of rotated image"]
    #[inline(always)]
    pub const fn rot_max_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "max column of rotated image"]
    #[inline(always)]
    pub fn set_rot_max_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for L2RotStat {
    #[inline(always)]
    fn default() -> L2RotStat {
        L2RotStat(0)
    }
}
impl core::fmt::Debug for L2RotStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2RotStat")
            .field("rot_max_line", &self.rot_max_line())
            .field("rot_max_col", &self.rot_max_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2RotStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L2RotStat {{ rot_max_line: {=u16:?}, rot_max_col: {=u16:?} }}",
            self.rot_max_line(),
            self.rot_max_col()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2ScaleInitCfg1(pub u32);
impl L2ScaleInitCfg1 {
    #[doc = "x-axis scale initial value, 10.16 format"]
    #[inline(always)]
    pub const fn x_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "x-axis scale initial value, 10.16 format"]
    #[inline(always)]
    pub fn set_x_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for L2ScaleInitCfg1 {
    #[inline(always)]
    fn default() -> L2ScaleInitCfg1 {
        L2ScaleInitCfg1(0)
    }
}
impl core::fmt::Debug for L2ScaleInitCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2ScaleInitCfg1")
            .field("x_val", &self.x_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2ScaleInitCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "L2ScaleInitCfg1 {{ x_val: {=u32:?} }}", self.x_val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2ScaleInitCfg2(pub u32);
impl L2ScaleInitCfg2 {
    #[doc = "y-axis scale initial value, 10.16 format"]
    #[inline(always)]
    pub const fn y_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "y-axis scale initial value, 10.16 format"]
    #[inline(always)]
    pub fn set_y_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for L2ScaleInitCfg2 {
    #[inline(always)]
    fn default() -> L2ScaleInitCfg2 {
        L2ScaleInitCfg2(0)
    }
}
impl core::fmt::Debug for L2ScaleInitCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2ScaleInitCfg2")
            .field("y_val", &self.y_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2ScaleInitCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "L2ScaleInitCfg2 {{ y_val: {=u32:?} }}", self.y_val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2ScaleRatioH(pub u32);
impl L2ScaleRatioH {
    #[doc = "x-axis rescaling ration, 10.16 fixed point number, XPITCH lt MAX_COL/(X1-X0)"]
    #[inline(always)]
    pub const fn xpitch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "x-axis rescaling ration, 10.16 fixed point number, XPITCH lt MAX_COL/(X1-X0)"]
    #[inline(always)]
    pub fn set_xpitch(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for L2ScaleRatioH {
    #[inline(always)]
    fn default() -> L2ScaleRatioH {
        L2ScaleRatioH(0)
    }
}
impl core::fmt::Debug for L2ScaleRatioH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2ScaleRatioH")
            .field("xpitch", &self.xpitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2ScaleRatioH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "L2ScaleRatioH {{ xpitch: {=u32:?} }}", self.xpitch())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2ScaleRatioV(pub u32);
impl L2ScaleRatioV {
    #[doc = "y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)"]
    #[inline(always)]
    pub const fn ypitch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)"]
    #[inline(always)]
    pub fn set_ypitch(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for L2ScaleRatioV {
    #[inline(always)]
    fn default() -> L2ScaleRatioV {
        L2ScaleRatioV(0)
    }
}
impl core::fmt::Debug for L2ScaleRatioV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2ScaleRatioV")
            .field("ypitch", &self.ypitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2ScaleRatioV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "L2ScaleRatioV {{ ypitch: {=u32:?} }}", self.ypitch())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2Src(pub u32);
impl L2Src {
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for L2Src {
    #[inline(always)]
    fn default() -> L2Src {
        L2Src(0)
    }
}
impl core::fmt::Debug for L2Src {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2Src").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2Src {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "L2Src {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2TlPos(pub u32);
impl L2TlPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for L2TlPos {
    #[inline(always)]
    fn default() -> L2TlPos {
        L2TlPos(0)
    }
}
impl core::fmt::Debug for L2TlPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2TlPos")
            .field("x0", &self.x0())
            .field("y0", &self.y0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2TlPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "L2TlPos {{ x0: {=u16:?}, y0: {=u16:?} }}",
            self.x0(),
            self.y0()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MaskBrPos(pub u32);
impl MaskBrPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for MaskBrPos {
    #[inline(always)]
    fn default() -> MaskBrPos {
        MaskBrPos(0)
    }
}
impl core::fmt::Debug for MaskBrPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MaskBrPos")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MaskBrPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MaskBrPos {{ x1: {=u16:?}, y1: {=u16:?} }}",
            self.x1(),
            self.y1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MaskCfg(pub u32);
impl MaskCfg {
    #[doc = "mask input format 1'h0: A8 1'h1: A4"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "mask input format 1'h0: A8 1'h1: A4"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "mask mix mode 1'h0: mult mode 1'h1: overwrite mode"]
    #[inline(always)]
    pub const fn mix_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "mask mix mode 1'h0: mult mode 1'h1: overwrite mode"]
    #[inline(always)]
    pub fn set_mix_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "layer0 mask enable"]
    #[inline(always)]
    pub const fn l0_mask_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "layer0 mask enable"]
    #[inline(always)]
    pub fn set_l0_mask_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "layer1 mask enable"]
    #[inline(always)]
    pub const fn l1_mask_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "layer1 mask enable"]
    #[inline(always)]
    pub fn set_l1_mask_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "layer2 mask enable"]
    #[inline(always)]
    pub const fn l2_mask_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "layer2 mask enable"]
    #[inline(always)]
    pub fn set_l2_mask_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "video layer mask enable"]
    #[inline(always)]
    pub const fn vl_mask_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "video layer mask enable"]
    #[inline(always)]
    pub fn set_vl_mask_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub const fn width(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x1fff;
        val as u16
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub fn set_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 14usize)) | (((val as u32) & 0x1fff) << 14usize);
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub const fn prefetch_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub fn set_prefetch_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for MaskCfg {
    #[inline(always)]
    fn default() -> MaskCfg {
        MaskCfg(0)
    }
}
impl core::fmt::Debug for MaskCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MaskCfg")
            .field("format", &self.format())
            .field("mix_mode", &self.mix_mode())
            .field("l0_mask_en", &self.l0_mask_en())
            .field("l1_mask_en", &self.l1_mask_en())
            .field("l2_mask_en", &self.l2_mask_en())
            .field("vl_mask_en", &self.vl_mask_en())
            .field("width", &self.width())
            .field("prefetch_en", &self.prefetch_en())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MaskCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "MaskCfg {{ format: {=bool:?}, mix_mode: {=bool:?}, l0_mask_en: {=bool:?}, l1_mask_en: {=bool:?}, l2_mask_en: {=bool:?}, vl_mask_en: {=bool:?}, width: {=u16:?}, prefetch_en: {=bool:?}, active: {=bool:?} }}" , self . format () , self . mix_mode () , self . l0_mask_en () , self . l1_mask_en () , self . l2_mask_en () , self . vl_mask_en () , self . width () , self . prefetch_en () , self . active ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MaskSrc(pub u32);
impl MaskSrc {
    #[doc = "mask data address\\[31:0\\]. This is byte address, even for A4, this address is byte aligned."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "mask data address\\[31:0\\]. This is byte address, even for A4, this address is byte aligned."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MaskSrc {
    #[inline(always)]
    fn default() -> MaskSrc {
        MaskSrc(0)
    }
}
impl core::fmt::Debug for MaskSrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MaskSrc")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MaskSrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MaskSrc {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MaskTlPos(pub u32);
impl MaskTlPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for MaskTlPos {
    #[inline(always)]
    fn default() -> MaskTlPos {
        MaskTlPos(0)
    }
}
impl core::fmt::Debug for MaskTlPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MaskTlPos")
            .field("x0", &self.x0())
            .field("y0", &self.y0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MaskTlPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MaskTlPos {{ x0: {=u16:?}, y0: {=u16:?} }}",
            self.x0(),
            self.y0()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemIfStat(pub u32);
impl MemIfStat {
    #[inline(always)]
    pub const fn ahb0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_ahb0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn arb_read_port0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_arb_read_port0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[inline(always)]
    pub const fn arb_main0(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_arb_main0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[inline(always)]
    pub const fn ahb1(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_ahb1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[inline(always)]
    pub const fn arb_read_port1(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_arb_read_port1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[inline(always)]
    pub const fn arb_main1(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_arb_main1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[inline(always)]
    pub const fn ahb_ctrl(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_ahb_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[inline(always)]
    pub const fn ahb_ctrl_eol(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ahb_ctrl_eol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[inline(always)]
    pub const fn ahb_ctrl_fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_ahb_ctrl_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
            .field("ahb0", &self.ahb0())
            .field("arb_read_port0", &self.arb_read_port0())
            .field("arb_main0", &self.arb_main0())
            .field("ahb1", &self.ahb1())
            .field("arb_read_port1", &self.arb_read_port1())
            .field("arb_main1", &self.arb_main1())
            .field("ahb_ctrl", &self.ahb_ctrl())
            .field("ahb_ctrl_eol", &self.ahb_ctrl_eol())
            .field("ahb_ctrl_fifo_cnt", &self.ahb_ctrl_fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MemIfStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "MemIfStat {{ ahb0: {=u8:?}, arb_read_port0: {=u8:?}, arb_main0: {=u8:?}, ahb1: {=u8:?}, arb_read_port1: {=u8:?}, arb_main1: {=u8:?}, ahb_ctrl: {=u8:?}, ahb_ctrl_eol: {=bool:?}, ahb_ctrl_fifo_cnt: {=u8:?} }}" , self . ahb0 () , self . arb_read_port0 () , self . arb_main0 () , self . ahb1 () , self . arb_read_port1 () , self . arb_main1 () , self . ahb_ctrl () , self . ahb_ctrl_eol () , self . ahb_ctrl_fifo_cnt ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MlStat(pub u32);
impl MlStat {
    #[inline(always)]
    pub const fn done_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_done_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn prefetch_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prefetch_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn prefetch_out(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prefetch_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn prefetch_read(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_prefetch_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[inline(always)]
    pub const fn mf_df(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_mf_df(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[inline(always)]
    pub const fn mf_pr(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_mf_pr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
}
impl Default for MlStat {
    #[inline(always)]
    fn default() -> MlStat {
        MlStat(0)
    }
}
impl core::fmt::Debug for MlStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MlStat")
            .field("done_req", &self.done_req())
            .field("prefetch_hold", &self.prefetch_hold())
            .field("prefetch_out", &self.prefetch_out())
            .field("prefetch_read", &self.prefetch_read())
            .field("mf_df", &self.mf_df())
            .field("mf_pr", &self.mf_pr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MlStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "MlStat {{ done_req: {=bool:?}, prefetch_hold: {=bool:?}, prefetch_out: {=bool:?}, prefetch_read: {=u8:?}, mf_df: {=u8:?}, mf_pr: {=u8:?} }}" , self . done_req () , self . prefetch_hold () , self . prefetch_out () , self . prefetch_read () , self . mf_df () , self . mf_pr ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ol2Stat(pub u32);
impl Ol2Stat {
    #[inline(always)]
    pub const fn prefetch_out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prefetch_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn prefetch_read(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_prefetch_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[inline(always)]
    pub const fn rf_rot(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_rf_rot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[inline(always)]
    pub const fn nf_pr(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nf_pr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[inline(always)]
    pub const fn nf_df(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_nf_df(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[inline(always)]
    pub const fn nf_data_conv(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_nf_data_conv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[inline(always)]
    pub const fn sc_out(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[inline(always)]
    pub const fn sc_be(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_be(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[inline(always)]
    pub const fn sc_fe(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_fe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
    }
    #[inline(always)]
    pub const fn sc_lb1(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_lb1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[inline(always)]
    pub const fn sc_lb0(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_lb0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
}
impl Default for Ol2Stat {
    #[inline(always)]
    fn default() -> Ol2Stat {
        Ol2Stat(0)
    }
}
impl core::fmt::Debug for Ol2Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ol2Stat")
            .field("prefetch_out", &self.prefetch_out())
            .field("prefetch_read", &self.prefetch_read())
            .field("rf_rot", &self.rf_rot())
            .field("nf_pr", &self.nf_pr())
            .field("nf_df", &self.nf_df())
            .field("nf_data_conv", &self.nf_data_conv())
            .field("sc_out", &self.sc_out())
            .field("sc_be", &self.sc_be())
            .field("sc_fe", &self.sc_fe())
            .field("sc_lb1", &self.sc_lb1())
            .field("sc_lb0", &self.sc_lb0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ol2Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ol2Stat {{ prefetch_out: {=bool:?}, prefetch_read: {=u8:?}, rf_rot: {=u8:?}, nf_pr: {=u8:?}, nf_df: {=u8:?}, nf_data_conv: {=u8:?}, sc_out: {=u8:?}, sc_be: {=u8:?}, sc_fe: {=u8:?}, sc_lb1: {=u8:?}, sc_lb0: {=u8:?} }}" , self . prefetch_out () , self . prefetch_read () , self . rf_rot () , self . nf_pr () , self . nf_df () , self . nf_data_conv () , self . sc_out () , self . sc_be () , self . sc_fe () , self . sc_lb1 () , self . sc_lb0 ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OlStat(pub u32);
impl OlStat {
    #[inline(always)]
    pub const fn done_req0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_done_req0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn prefetch_hold0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prefetch_hold0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn prefetch_out0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prefetch_out0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn prefetch_read0(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_prefetch_read0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[inline(always)]
    pub const fn data_conv0(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_data_conv0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[inline(always)]
    pub const fn pf_df0(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_pf_df0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[inline(always)]
    pub const fn pf_pr0(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pf_pr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[inline(always)]
    pub const fn done_req1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_done_req1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn prefetch_hold1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prefetch_hold1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn prefetch_out1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prefetch_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn prefetch_read1(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_prefetch_read1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
    #[inline(always)]
    pub const fn data_conv1(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_data_conv1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[inline(always)]
    pub const fn pf_df1(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_pf_df1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[inline(always)]
    pub const fn pf_pr1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pf_pr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
}
impl Default for OlStat {
    #[inline(always)]
    fn default() -> OlStat {
        OlStat(0)
    }
}
impl core::fmt::Debug for OlStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OlStat")
            .field("done_req0", &self.done_req0())
            .field("prefetch_hold0", &self.prefetch_hold0())
            .field("prefetch_out0", &self.prefetch_out0())
            .field("prefetch_read0", &self.prefetch_read0())
            .field("data_conv0", &self.data_conv0())
            .field("pf_df0", &self.pf_df0())
            .field("pf_pr0", &self.pf_pr0())
            .field("done_req1", &self.done_req1())
            .field("prefetch_hold1", &self.prefetch_hold1())
            .field("prefetch_out1", &self.prefetch_out1())
            .field("prefetch_read1", &self.prefetch_read1())
            .field("data_conv1", &self.data_conv1())
            .field("pf_df1", &self.pf_df1())
            .field("pf_pr1", &self.pf_pr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OlStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "OlStat {{ done_req0: {=bool:?}, prefetch_hold0: {=bool:?}, prefetch_out0: {=bool:?}, prefetch_read0: {=u8:?}, data_conv0: {=u8:?}, pf_df0: {=u8:?}, pf_pr0: {=u8:?}, done_req1: {=bool:?}, prefetch_hold1: {=bool:?}, prefetch_out1: {=bool:?}, prefetch_read1: {=u8:?}, data_conv1: {=u8:?}, pf_df1: {=u8:?}, pf_pr1: {=u8:?} }}" , self . done_req0 () , self . prefetch_hold0 () , self . prefetch_out0 () , self . prefetch_read0 () , self . data_conv0 () , self . pf_df0 () , self . pf_pr0 () , self . done_req1 () , self . prefetch_hold1 () , self . prefetch_out1 () , self . prefetch_read1 () , self . data_conv1 () , self . pf_df1 () , self . pf_pr1 ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfCnt(pub u32);
impl PerfCnt {
    #[doc = "epic performance counter"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "epic performance counter"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
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
pub struct Rsvd3(pub u32);
impl Rsvd3 {}
impl Default for Rsvd3 {
    #[inline(always)]
    fn default() -> Rsvd3 {
        Rsvd3(0)
    }
}
impl core::fmt::Debug for Rsvd3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd3").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd3 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd4(pub u32);
impl Rsvd4 {}
impl Default for Rsvd4 {
    #[inline(always)]
    fn default() -> Rsvd4 {
        Rsvd4(0)
    }
}
impl core::fmt::Debug for Rsvd4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd4").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd4 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setting(pub u32);
impl Setting {
    #[doc = "end of frame interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn eof_irq_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "end of frame interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_eof_irq_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "canvas line hit interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn line_irq_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "canvas line hit interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_line_irq_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn auto_gate_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub fn set_auto_gate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "canvas line hit interrupt line number"]
    #[inline(always)]
    pub const fn line_irq_num(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "canvas line hit interrupt line number"]
    #[inline(always)]
    pub fn set_line_irq_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
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
            .field("eof_irq_mask", &self.eof_irq_mask())
            .field("line_irq_mask", &self.line_irq_mask())
            .field("auto_gate_en", &self.auto_gate_en())
            .field("line_irq_num", &self.line_irq_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setting {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Setting {{ eof_irq_mask: {=bool:?}, line_irq_mask: {=bool:?}, auto_gate_en: {=bool:?}, line_irq_num: {=u16:?} }}" , self . eof_irq_mask () , self . line_irq_mask () , self . auto_gate_en () , self . line_irq_num ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Graphics accelerator busy flag"]
    #[inline(always)]
    pub const fn ia_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Graphics accelerator busy flag"]
    #[inline(always)]
    pub fn set_ia_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LCD controll busy flag"]
    #[inline(always)]
    pub const fn lcd_busy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LCD controll busy flag"]
    #[inline(always)]
    pub fn set_lcd_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("ia_busy", &self.ia_busy())
            .field("lcd_busy", &self.lcd_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ ia_busy: {=bool:?}, lcd_busy: {=bool:?} }}",
            self.ia_busy(),
            self.lcd_busy()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USrc(pub u32);
impl USrc {
    #[doc = "u vector address"]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "u vector address"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USrc {
    #[inline(always)]
    fn default() -> USrc {
        USrc(0)
    }
}
impl core::fmt::Debug for USrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USrc").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USrc {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VSrc(pub u32);
impl VSrc {
    #[doc = "v vector address"]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "v vector address"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for VSrc {
    #[inline(always)]
    fn default() -> VSrc {
        VSrc(0)
    }
}
impl core::fmt::Debug for VSrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VSrc").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VSrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VSrc {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlBrPos(pub u32);
impl VlBrPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for VlBrPos {
    #[inline(always)]
    fn default() -> VlBrPos {
        VlBrPos(0)
    }
}
impl core::fmt::Debug for VlBrPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlBrPos")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlBrPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VlBrPos {{ x1: {=u16:?}, y1: {=u16:?} }}",
            self.x1(),
            self.y1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlCfg(pub u32);
impl VlCfg {
    #[doc = "video layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2"]
    #[inline(always)]
    pub const fn format(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "video layer input format 4'h0: RGB565 4'h1: RGB888 4'h2: ARGB8888 4'h3: ARGB8565 4'h4: A8 4'h5: A4 4'h6: L8 4'h7: A2"]
    #[inline(always)]
    pub fn set_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub const fn alpha_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "alpha selection 1'b0: select alpha according to image format 1'b1: select layer alpha"]
    #[inline(always)]
    pub fn set_alpha_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "layer alpha value"]
    #[inline(always)]
    pub fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "video layer blending depth"]
    #[inline(always)]
    pub const fn blend_depth(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "video layer blending depth"]
    #[inline(always)]
    pub fn set_blend_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub const fn filter_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "layer color filter enable"]
    #[inline(always)]
    pub fn set_filter_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub const fn width(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "source image width(including padding), unit is bytes"]
    #[inline(always)]
    pub fn set_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub const fn prefetch_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "preload 64 bytes extra data when reading pixel from memory"]
    #[inline(always)]
    pub fn set_prefetch_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "layer active flag"]
    #[inline(always)]
    pub fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub const fn alpha_blend(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to enable alpha blending mode. Use layer alpha as blending factor for image with Alpha. Alpha_out = Layer_alpha * Image_alpha"]
    #[inline(always)]
    pub fn set_alpha_blend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VlCfg {
    #[inline(always)]
    fn default() -> VlCfg {
        VlCfg(0)
    }
}
impl core::fmt::Debug for VlCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlCfg")
            .field("format", &self.format())
            .field("alpha_sel", &self.alpha_sel())
            .field("alpha", &self.alpha())
            .field("blend_depth", &self.blend_depth())
            .field("filter_en", &self.filter_en())
            .field("width", &self.width())
            .field("prefetch_en", &self.prefetch_en())
            .field("active", &self.active())
            .field("alpha_blend", &self.alpha_blend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VlCfg {{ format: {=u8:?}, alpha_sel: {=bool:?}, alpha: {=u8:?}, blend_depth: {=u8:?}, filter_en: {=bool:?}, width: {=u16:?}, prefetch_en: {=bool:?}, active: {=bool:?}, alpha_blend: {=bool:?} }}" , self . format () , self . alpha_sel () , self . alpha () , self . blend_depth () , self . filter_en () , self . width () , self . prefetch_en () , self . active () , self . alpha_blend ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlExtents(pub u32);
impl VlExtents {
    #[doc = "number of pixels of each column of source image(not including padding)"]
    #[inline(always)]
    pub const fn max_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "number of pixels of each column of source image(not including padding)"]
    #[inline(always)]
    pub fn set_max_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "number of pixels of each line of source image(not including padding)"]
    #[inline(always)]
    pub const fn max_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "number of pixels of each line of source image(not including padding)"]
    #[inline(always)]
    pub fn set_max_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for VlExtents {
    #[inline(always)]
    fn default() -> VlExtents {
        VlExtents(0)
    }
}
impl core::fmt::Debug for VlExtents {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlExtents")
            .field("max_line", &self.max_line())
            .field("max_col", &self.max_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlExtents {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VlExtents {{ max_line: {=u16:?}, max_col: {=u16:?} }}",
            self.max_line(),
            self.max_col()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlFill(pub u32);
impl VlFill {
    #[doc = "background b color"]
    #[inline(always)]
    pub const fn bg_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "background b color"]
    #[inline(always)]
    pub fn set_bg_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub const fn bg_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "background g color"]
    #[inline(always)]
    pub fn set_bg_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub const fn bg_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "background r color"]
    #[inline(always)]
    pub fn set_bg_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Not used."]
    #[inline(always)]
    pub const fn bg_mode(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Not used."]
    #[inline(always)]
    pub fn set_bg_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub const fn endian(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "input 565 data format endian 0: {R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]} 1: {G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    #[inline(always)]
    pub fn set_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for VlFill {
    #[inline(always)]
    fn default() -> VlFill {
        VlFill(0)
    }
}
impl core::fmt::Debug for VlFill {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlFill")
            .field("bg_b", &self.bg_b())
            .field("bg_g", &self.bg_g())
            .field("bg_r", &self.bg_r())
            .field("bg_mode", &self.bg_mode())
            .field("endian", &self.endian())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlFill {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VlFill {{ bg_b: {=u8:?}, bg_g: {=u8:?}, bg_r: {=u8:?}, bg_mode: {=bool:?}, endian: {=bool:?} }}" , self . bg_b () , self . bg_g () , self . bg_r () , self . bg_mode () , self . endian ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlFilter(pub u32);
impl VlFilter {
    #[doc = "filter b color"]
    #[inline(always)]
    pub const fn filter_b(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "filter b color"]
    #[inline(always)]
    pub fn set_filter_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub const fn filter_g(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "filter g color"]
    #[inline(always)]
    pub fn set_filter_g(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub const fn filter_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "filter r color"]
    #[inline(always)]
    pub fn set_filter_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub const fn filter_mask(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "layer color filter mask"]
    #[inline(always)]
    pub fn set_filter_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for VlFilter {
    #[inline(always)]
    fn default() -> VlFilter {
        VlFilter(0)
    }
}
impl core::fmt::Debug for VlFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlFilter")
            .field("filter_b", &self.filter_b())
            .field("filter_g", &self.filter_g())
            .field("filter_r", &self.filter_r())
            .field("filter_mask", &self.filter_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlFilter {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VlFilter {{ filter_b: {=u8:?}, filter_g: {=u8:?}, filter_r: {=u8:?}, filter_mask: {=u8:?} }}" , self . filter_b () , self . filter_g () , self . filter_r () , self . filter_mask ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlMiscCfg(pub u32);
impl VlMiscCfg {
    #[doc = "VL CLUT select: 1'h1: select pallette1 1'h0: select pallette0"]
    #[inline(always)]
    pub const fn clut_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VL CLUT select: 1'h1: select pallette1 1'h0: select pallette0"]
    #[inline(always)]
    pub fn set_clut_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "vertical mirror enable"]
    #[inline(always)]
    pub const fn v_mirror(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "vertical mirror enable"]
    #[inline(always)]
    pub fn set_v_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "horizontal mirror enable"]
    #[inline(always)]
    pub const fn h_mirror(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "horizontal mirror enable"]
    #[inline(always)]
    pub fn set_h_mirror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "external absolute value of cos. U13.12 format."]
    #[inline(always)]
    pub const fn cos_force_value(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x1fff;
        val as u16
    }
    #[doc = "external absolute value of cos. U13.12 format."]
    #[inline(always)]
    pub fn set_cos_force_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
    }
    #[doc = "external absolute value of sin. U13.12 format."]
    #[inline(always)]
    pub const fn sin_force_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "external absolute value of sin. U13.12 format."]
    #[inline(always)]
    pub fn set_sin_force_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
    #[doc = "force epic use external sin and cos value, quadrant is still calculated from ROT_DEG."]
    #[inline(always)]
    pub const fn deg_force(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "force epic use external sin and cos value, quadrant is still calculated from ROT_DEG."]
    #[inline(always)]
    pub fn set_deg_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for VlMiscCfg {
    #[inline(always)]
    fn default() -> VlMiscCfg {
        VlMiscCfg(0)
    }
}
impl core::fmt::Debug for VlMiscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlMiscCfg")
            .field("clut_sel", &self.clut_sel())
            .field("v_mirror", &self.v_mirror())
            .field("h_mirror", &self.h_mirror())
            .field("cos_force_value", &self.cos_force_value())
            .field("sin_force_value", &self.sin_force_value())
            .field("deg_force", &self.deg_force())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlMiscCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VlMiscCfg {{ clut_sel: {=bool:?}, v_mirror: {=bool:?}, h_mirror: {=bool:?}, cos_force_value: {=u16:?}, sin_force_value: {=u16:?}, deg_force: {=bool:?} }}" , self . clut_sel () , self . v_mirror () , self . h_mirror () , self . cos_force_value () , self . sin_force_value () , self . deg_force ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlRot(pub u32);
impl VlRot {
    #[doc = "rot_max_col and rot_max_line calculation request. Write 1 to trigger the calculation."]
    #[inline(always)]
    pub const fn calc_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "rot_max_col and rot_max_line calculation request. Write 1 to trigger the calculation."]
    #[inline(always)]
    pub fn set_calc_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "rot_max_col and rot_max_line calculation clear request. Write 1 to clear the result."]
    #[inline(always)]
    pub const fn calc_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "rot_max_col and rot_max_line calculation clear request. Write 1 to clear the result."]
    #[inline(always)]
    pub fn set_calc_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "rotation degree, rotation is clockwise."]
    #[inline(always)]
    pub const fn rot_deg(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x01ff;
        val as u16
    }
    #[doc = "rotation degree, rotation is clockwise."]
    #[inline(always)]
    pub fn set_rot_deg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
    }
    #[doc = "calculation done indicator"]
    #[inline(always)]
    pub const fn calc_done(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "calculation done indicator"]
    #[inline(always)]
    pub fn set_calc_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for VlRot {
    #[inline(always)]
    fn default() -> VlRot {
        VlRot(0)
    }
}
impl core::fmt::Debug for VlRot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlRot")
            .field("calc_req", &self.calc_req())
            .field("calc_clr", &self.calc_clr())
            .field("rot_deg", &self.rot_deg())
            .field("calc_done", &self.calc_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlRot {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VlRot {{ calc_req: {=bool:?}, calc_clr: {=bool:?}, rot_deg: {=u16:?}, calc_done: {=bool:?} }}" , self . calc_req () , self . calc_clr () , self . rot_deg () , self . calc_done ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlRotMCfg1(pub u32);
impl VlRotMCfg1 {
    #[doc = "manual mode rotation max line, unsigned value"]
    #[inline(always)]
    pub const fn m_rot_max_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode rotation max line, unsigned value"]
    #[inline(always)]
    pub fn set_m_rot_max_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "manual mode rotation max column, unsigned value"]
    #[inline(always)]
    pub const fn m_rot_max_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode rotation max column, unsigned value"]
    #[inline(always)]
    pub fn set_m_rot_max_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[doc = "rotation mode setting 1'b0: auto mode 1'b1: manual mode"]
    #[inline(always)]
    pub const fn m_mode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "rotation mode setting 1'b0: auto mode 1'b1: manual mode"]
    #[inline(always)]
    pub fn set_m_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VlRotMCfg1 {
    #[inline(always)]
    fn default() -> VlRotMCfg1 {
        VlRotMCfg1(0)
    }
}
impl core::fmt::Debug for VlRotMCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlRotMCfg1")
            .field("m_rot_max_line", &self.m_rot_max_line())
            .field("m_rot_max_col", &self.m_rot_max_col())
            .field("m_mode", &self.m_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlRotMCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VlRotMCfg1 {{ m_rot_max_line: {=u16:?}, m_rot_max_col: {=u16:?}, m_mode: {=bool:?} }}",
            self.m_rot_max_line(),
            self.m_rot_max_col(),
            self.m_mode()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlRotMCfg2(pub u32);
impl VlRotMCfg2 {
    #[doc = "manual mode pivot x, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub const fn m_pivot_x(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode pivot x, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub fn set_m_pivot_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "manual mode pivot y, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub const fn m_pivot_y(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode pivot y, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub fn set_m_pivot_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for VlRotMCfg2 {
    #[inline(always)]
    fn default() -> VlRotMCfg2 {
        VlRotMCfg2(0)
    }
}
impl core::fmt::Debug for VlRotMCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlRotMCfg2")
            .field("m_pivot_x", &self.m_pivot_x())
            .field("m_pivot_y", &self.m_pivot_y())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlRotMCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VlRotMCfg2 {{ m_pivot_x: {=u16:?}, m_pivot_y: {=u16:?} }}",
            self.m_pivot_x(),
            self.m_pivot_y()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlRotMCfg3(pub u32);
impl VlRotMCfg3 {
    #[doc = "manual mode top left x cordinate, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub const fn m_xtl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode top left x cordinate, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub fn set_m_xtl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "manual mode top left y cordinate, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub const fn m_ytl(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "manual mode top left y cordinate, signed value, -1023~1023, -1024 is not supported"]
    #[inline(always)]
    pub fn set_m_ytl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for VlRotMCfg3 {
    #[inline(always)]
    fn default() -> VlRotMCfg3 {
        VlRotMCfg3(0)
    }
}
impl core::fmt::Debug for VlRotMCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlRotMCfg3")
            .field("m_xtl", &self.m_xtl())
            .field("m_ytl", &self.m_ytl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlRotMCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VlRotMCfg3 {{ m_xtl: {=u16:?}, m_ytl: {=u16:?} }}",
            self.m_xtl(),
            self.m_ytl()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlRotStat(pub u32);
impl VlRotStat {
    #[doc = "max line of rotated image"]
    #[inline(always)]
    pub const fn rot_max_line(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "max line of rotated image"]
    #[inline(always)]
    pub fn set_rot_max_line(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "max column of rotated image"]
    #[inline(always)]
    pub const fn rot_max_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "max column of rotated image"]
    #[inline(always)]
    pub fn set_rot_max_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for VlRotStat {
    #[inline(always)]
    fn default() -> VlRotStat {
        VlRotStat(0)
    }
}
impl core::fmt::Debug for VlRotStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlRotStat")
            .field("rot_max_line", &self.rot_max_line())
            .field("rot_max_col", &self.rot_max_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlRotStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VlRotStat {{ rot_max_line: {=u16:?}, rot_max_col: {=u16:?} }}",
            self.rot_max_line(),
            self.rot_max_col()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlScaleInitCfg1(pub u32);
impl VlScaleInitCfg1 {
    #[doc = "x-axis scale initial value, 10.16 format"]
    #[inline(always)]
    pub const fn x_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "x-axis scale initial value, 10.16 format"]
    #[inline(always)]
    pub fn set_x_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for VlScaleInitCfg1 {
    #[inline(always)]
    fn default() -> VlScaleInitCfg1 {
        VlScaleInitCfg1(0)
    }
}
impl core::fmt::Debug for VlScaleInitCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlScaleInitCfg1")
            .field("x_val", &self.x_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlScaleInitCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VlScaleInitCfg1 {{ x_val: {=u32:?} }}", self.x_val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlScaleInitCfg2(pub u32);
impl VlScaleInitCfg2 {
    #[doc = "y-axis scale initial value, 10.16 format"]
    #[inline(always)]
    pub const fn y_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "y-axis scale initial value, 10.16 format"]
    #[inline(always)]
    pub fn set_y_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for VlScaleInitCfg2 {
    #[inline(always)]
    fn default() -> VlScaleInitCfg2 {
        VlScaleInitCfg2(0)
    }
}
impl core::fmt::Debug for VlScaleInitCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlScaleInitCfg2")
            .field("y_val", &self.y_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlScaleInitCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VlScaleInitCfg2 {{ y_val: {=u32:?} }}", self.y_val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlScaleRatioH(pub u32);
impl VlScaleRatioH {
    #[doc = "x-axis rescaling ration, 10.16 fixed point number, XPITCH lt MAX_COL/(X1-X0)"]
    #[inline(always)]
    pub const fn xpitch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "x-axis rescaling ration, 10.16 fixed point number, XPITCH lt MAX_COL/(X1-X0)"]
    #[inline(always)]
    pub fn set_xpitch(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for VlScaleRatioH {
    #[inline(always)]
    fn default() -> VlScaleRatioH {
        VlScaleRatioH(0)
    }
}
impl core::fmt::Debug for VlScaleRatioH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlScaleRatioH")
            .field("xpitch", &self.xpitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlScaleRatioH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VlScaleRatioH {{ xpitch: {=u32:?} }}", self.xpitch())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlScaleRatioV(pub u32);
impl VlScaleRatioV {
    #[doc = "y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)"]
    #[inline(always)]
    pub const fn ypitch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "y-axis rescaling ratio, 10.16 fixed point number, YPITCH lt MAX_LINE/(Y1-Y0)"]
    #[inline(always)]
    pub fn set_ypitch(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for VlScaleRatioV {
    #[inline(always)]
    fn default() -> VlScaleRatioV {
        VlScaleRatioV(0)
    }
}
impl core::fmt::Debug for VlScaleRatioV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlScaleRatioV")
            .field("ypitch", &self.ypitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlScaleRatioV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VlScaleRatioV {{ ypitch: {=u32:?} }}", self.ypitch())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlSrc(pub u32);
impl VlSrc {
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "source image RGB data address\\[31:0\\]. For RGB565 format, address should be aligned to halfword. For ARGB8888 format, address should be aligned to word."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for VlSrc {
    #[inline(always)]
    fn default() -> VlSrc {
        VlSrc(0)
    }
}
impl core::fmt::Debug for VlSrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlSrc").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlSrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VlSrc {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlStat(pub u32);
impl VlStat {
    #[inline(always)]
    pub const fn prefetch_out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_prefetch_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn prefetch_read(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_prefetch_read(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[inline(always)]
    pub const fn rf_rot(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_rf_rot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[inline(always)]
    pub const fn nf_pr(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nf_pr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[inline(always)]
    pub const fn nf_df(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_nf_df(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[inline(always)]
    pub const fn nf_data_conv(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_nf_data_conv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[inline(always)]
    pub const fn sc_out(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[inline(always)]
    pub const fn sc_be(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_be(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[inline(always)]
    pub const fn sc_fe(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_fe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
    }
    #[inline(always)]
    pub const fn sc_lb1(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_lb1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[inline(always)]
    pub const fn sc_lb0(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_sc_lb0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
}
impl Default for VlStat {
    #[inline(always)]
    fn default() -> VlStat {
        VlStat(0)
    }
}
impl core::fmt::Debug for VlStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlStat")
            .field("prefetch_out", &self.prefetch_out())
            .field("prefetch_read", &self.prefetch_read())
            .field("rf_rot", &self.rf_rot())
            .field("nf_pr", &self.nf_pr())
            .field("nf_df", &self.nf_df())
            .field("nf_data_conv", &self.nf_data_conv())
            .field("sc_out", &self.sc_out())
            .field("sc_be", &self.sc_be())
            .field("sc_fe", &self.sc_fe())
            .field("sc_lb1", &self.sc_lb1())
            .field("sc_lb0", &self.sc_lb0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VlStat {{ prefetch_out: {=bool:?}, prefetch_read: {=u8:?}, rf_rot: {=u8:?}, nf_pr: {=u8:?}, nf_df: {=u8:?}, nf_data_conv: {=u8:?}, sc_out: {=u8:?}, sc_be: {=u8:?}, sc_fe: {=u8:?}, sc_lb1: {=u8:?}, sc_lb0: {=u8:?} }}" , self . prefetch_out () , self . prefetch_read () , self . rf_rot () , self . nf_pr () , self . nf_df () , self . nf_data_conv () , self . sc_out () , self . sc_be () , self . sc_fe () , self . sc_lb1 () , self . sc_lb0 ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VlTlPos(pub u32);
impl VlTlPos {
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub const fn x0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordinate X-value"]
    #[inline(always)]
    pub fn set_x0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub const fn y0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Coordingate Y-value"]
    #[inline(always)]
    pub fn set_y0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for VlTlPos {
    #[inline(always)]
    fn default() -> VlTlPos {
        VlTlPos(0)
    }
}
impl core::fmt::Debug for VlTlPos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VlTlPos")
            .field("x0", &self.x0())
            .field("y0", &self.y0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VlTlPos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VlTlPos {{ x0: {=u16:?}, y0: {=u16:?} }}",
            self.x0(),
            self.y0()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct YSrc(pub u32);
impl YSrc {
    #[doc = "y vector address"]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "y vector address"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for YSrc {
    #[inline(always)]
    fn default() -> YSrc {
        YSrc(0)
    }
}
impl core::fmt::Debug for YSrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YSrc").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for YSrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "YSrc {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct YuvEngCfg0(pub u32);
impl YuvEngCfg0 {
    #[doc = "yuv u vector line width, unit is bytes"]
    #[inline(always)]
    pub const fn width_u(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "yuv u vector line width, unit is bytes"]
    #[inline(always)]
    pub fn set_width_u(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "yuv y vector line width, unit is bytes"]
    #[inline(always)]
    pub const fn width_y(&self) -> u16 {
        let val = (self.0 >> 13usize) & 0x1fff;
        val as u16
    }
    #[doc = "yuv y vector line width, unit is bytes"]
    #[inline(always)]
    pub fn set_width_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 13usize)) | (((val as u32) & 0x1fff) << 13usize);
    }
    #[doc = "yuv format"]
    #[inline(always)]
    pub const fn format(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "yuv format"]
    #[inline(always)]
    pub fn set_format(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "yuv input range 1'h0: tv range 1'h1: pc range"]
    #[inline(always)]
    pub const fn range_sel(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "yuv input range 1'h0: tv range 1'h1: pc range"]
    #[inline(always)]
    pub fn set_range_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for YuvEngCfg0 {
    #[inline(always)]
    fn default() -> YuvEngCfg0 {
        YuvEngCfg0(0)
    }
}
impl core::fmt::Debug for YuvEngCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YuvEngCfg0")
            .field("width_u", &self.width_u())
            .field("width_y", &self.width_y())
            .field("format", &self.format())
            .field("range_sel", &self.range_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for YuvEngCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "YuvEngCfg0 {{ width_u: {=u16:?}, width_y: {=u16:?}, format: {=u8:?}, range_sel: {=bool:?} }}" , self . width_u () , self . width_y () , self . format () , self . range_sel ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct YuvEngCfg1(pub u32);
impl YuvEngCfg1 {
    #[doc = "yuv v vector line width, unit is bytes"]
    #[inline(always)]
    pub const fn width_v(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "yuv v vector line width, unit is bytes"]
    #[inline(always)]
    pub fn set_width_v(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for YuvEngCfg1 {
    #[inline(always)]
    fn default() -> YuvEngCfg1 {
        YuvEngCfg1(0)
    }
}
impl core::fmt::Debug for YuvEngCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YuvEngCfg1")
            .field("width_v", &self.width_v())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for YuvEngCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "YuvEngCfg1 {{ width_v: {=u16:?} }}", self.width_v())
    }
}
