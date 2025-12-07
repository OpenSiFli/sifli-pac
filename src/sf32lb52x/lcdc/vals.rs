#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbFormat {
    RGB565 = 0x0,
    RGB888 = 0x01,
    ARGB8888 = 0x02,
    RGB332 = 0x03,
}
impl AhbFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbFormat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbFormat {
    #[inline(always)]
    fn from(val: u8) -> AhbFormat {
        AhbFormat::from_bits(val)
    }
}
impl From<AhbFormat> for u8 {
    #[inline(always)]
    fn from(val: AhbFormat) -> u8 {
        AhbFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlphaSel {
    #[doc = "Select alpha according to image format"]
    Image = 0x0,
    #[doc = "Select layer alpha"]
    Layer = 0x01,
}
impl AlphaSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlphaSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlphaSel {
    #[inline(always)]
    fn from(val: u8) -> AlphaSel {
        AlphaSel::from_bits(val)
    }
}
impl From<AlphaSel> for u8 {
    #[inline(always)]
    fn from(val: AlphaSel) -> u8 {
        AlphaSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DecompBlockWidth {
    #[doc = "Block size is 16 pixels"]
    Pixels16 = 0x0,
    #[doc = "Block size is 32 pixels"]
    Pixels32 = 0x01,
}
impl DecompBlockWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DecompBlockWidth {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DecompBlockWidth {
    #[inline(always)]
    fn from(val: u8) -> DecompBlockWidth {
        DecompBlockWidth::from_bits(val)
    }
}
impl From<DecompBlockWidth> for u8 {
    #[inline(always)]
    fn from(val: DecompBlockWidth) -> u8 {
        DecompBlockWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpiHstat {
    Idle = 0x0,
    Prep = 0x01,
    Hsync = 0x02,
    Hbp = 0x03,
    Hact = 0x04,
    Hfp = 0x05,
    Wait = 0x06,
    _RESERVED_7 = 0x07,
}
impl DpiHstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpiHstat {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpiHstat {
    #[inline(always)]
    fn from(val: u8) -> DpiHstat {
        DpiHstat::from_bits(val)
    }
}
impl From<DpiHstat> for u8 {
    #[inline(always)]
    fn from(val: DpiHstat) -> u8 {
        DpiHstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpiLcdFormat {
    Conf16bit1 = 0x0,
    Conf16bit2 = 0x01,
    Conf16bit3 = 0x02,
    Conf18bit1 = 0x03,
    Conf18bit2 = 0x04,
    Conf24bit = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl DpiLcdFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpiLcdFormat {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpiLcdFormat {
    #[inline(always)]
    fn from(val: u8) -> DpiLcdFormat {
        DpiLcdFormat::from_bits(val)
    }
}
impl From<DpiLcdFormat> for u8 {
    #[inline(always)]
    fn from(val: DpiLcdFormat) -> u8 {
        DpiLcdFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmarkMode {
    #[doc = "Pulse trigger"]
    Pulse = 0x0,
    #[doc = "Edge trigger"]
    Edge = 0x01,
}
impl FmarkMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmarkMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmarkMode {
    #[inline(always)]
    fn from(val: u8) -> FmarkMode {
        FmarkMode::from_bits(val)
    }
}
impl From<FmarkMode> for u8 {
    #[inline(always)]
    fn from(val: FmarkMode) -> u8 {
        FmarkMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmarkSource {
    #[doc = "Use TE signal from external pin"]
    External = 0x0,
    #[doc = "Use TE signal from DSI"]
    Dsi = 0x01,
}
impl FmarkSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmarkSource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmarkSource {
    #[inline(always)]
    fn from(val: u8) -> FmarkSource {
        FmarkSource::from_bits(val)
    }
}
impl From<FmarkSource> for u8 {
    #[inline(always)]
    fn from(val: FmarkSource) -> u8 {
        FmarkSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum JdiSerFormat {
    Mode3bit = 0x0,
    Mode4bit = 0x01,
    Mode1bit = 0x02,
    _RESERVED_3 = 0x03,
}
impl JdiSerFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> JdiSerFormat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for JdiSerFormat {
    #[inline(always)]
    fn from(val: u8) -> JdiSerFormat {
        JdiSerFormat::from_bits(val)
    }
}
impl From<JdiSerFormat> for u8 {
    #[inline(always)]
    fn from(val: JdiSerFormat) -> u8 {
        JdiSerFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LayerFormat {
    RGB565 = 0x0,
    RGB888 = 0x01,
    ARGB8888 = 0x02,
    ARGB8565 = 0x03,
    RGB332 = 0x04,
    A8 = 0x05,
    L8 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LayerFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LayerFormat {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LayerFormat {
    #[inline(always)]
    fn from(val: u8) -> LayerFormat {
        LayerFormat::from_bits(val)
    }
}
impl From<LayerFormat> for u8 {
    #[inline(always)]
    fn from(val: LayerFormat) -> u8 {
        LayerFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdFormat {
    #[doc = "8-bit RGB 3:3:2"]
    Rgb332 = 0x0,
    #[doc = "16-bit RGB 5:6:5 over 8-bit bus, 2 cycles/pixel"]
    Rgb565Over8bit = 0x01,
    #[doc = "12-bit RGB 4:4:4"]
    Rgb444 = 0x02,
    #[doc = "16-bit RGB 5:6:5"]
    Rgb565 = 0x03,
    #[doc = "18-bit RGB 6:6:6"]
    Rgb666 = 0x04,
    #[doc = "24-bit RGB 8:8:8"]
    Rgb888 = 0x05,
    #[doc = "24-bit RGB 8:8:8 over 16-bit bus, 1.5 cycles/pixel"]
    Rgb888Over16bit = 0x06,
    #[doc = "24-bit RGB 8:8:8 over 8-bit bus, 3 cycles/pixel"]
    Rgb888Over8bit = 0x07,
}
impl LcdFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdFormat {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdFormat {
    #[inline(always)]
    fn from(val: u8) -> LcdFormat {
        LcdFormat::from_bits(val)
    }
}
impl From<LcdFormat> for u8 {
    #[inline(always)]
    fn from(val: LcdFormat) -> u8 {
        LcdFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LcdIntfSel {
    #[doc = "8080 DBI Type B"]
    DbiTypeB = 0x0,
    #[doc = "SPI interface"]
    Spi = 0x01,
    #[doc = "DBI to DSI interface"]
    DbiToDsi = 0x02,
    #[doc = "DPI interface"]
    Dpi = 0x03,
    #[doc = "JDI serial interface"]
    JdiSerial = 0x04,
    #[doc = "JDI parallel interface"]
    JdiParallel = 0x05,
    #[doc = "8080 DBI Type A"]
    DbiTypeA = 0x06,
    #[doc = "DPI to DSI interface"]
    DpiToDsi = 0x07,
}
impl LcdIntfSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdIntfSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdIntfSel {
    #[inline(always)]
    fn from(val: u8) -> LcdIntfSel {
        LcdIntfSel::from_bits(val)
    }
}
impl From<LcdIntfSel> for u8 {
    #[inline(always)]
    fn from(val: LcdIntfSel) -> u8 {
        LcdIntfSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LfsrLoadSel {
    None = 0x0,
    Red = 0x01,
    Green = 0x02,
    Blue = 0x03,
}
impl LfsrLoadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfsrLoadSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfsrLoadSel {
    #[inline(always)]
    fn from(val: u8) -> LfsrLoadSel {
        LfsrLoadSel::from_bits(val)
    }
}
impl From<LfsrLoadSel> for u8 {
    #[inline(always)]
    fn from(val: LfsrLoadSel) -> u8 {
        LfsrLoadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LineFetchMode {
    #[doc = "Address skip every single line"]
    SingleLine = 0x0,
    #[doc = "Address skip every two lines"]
    TwoLines = 0x01,
}
impl LineFetchMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LineFetchMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LineFetchMode {
    #[inline(always)]
    fn from(val: u8) -> LineFetchMode {
        LineFetchMode::from_bits(val)
    }
}
impl From<LineFetchMode> for u8 {
    #[inline(always)]
    fn from(val: LineFetchMode) -> u8 {
        LineFetchMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Polarity {
    #[doc = "Active low"]
    ActiveLow = 0x0,
    #[doc = "Active high"]
    ActiveHigh = 0x01,
}
impl Polarity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Polarity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Polarity {
    #[inline(always)]
    fn from(val: u8) -> Polarity {
        Polarity::from_bits(val)
    }
}
impl From<Polarity> for u8 {
    #[inline(always)]
    fn from(val: Polarity) -> u8 {
        Polarity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rgb565Endian {
    #[doc = "{R\\[4:0\\], G\\[5:3\\], G\\[2:0\\], B\\[4:0\\]}"]
    R4_0_G5_3_G2_0_B4_0 = 0x0,
    #[doc = "{G\\[2:0\\], R\\[4:0\\], B\\[4:0\\], G\\[5:3\\]}"]
    G2_0_R4_0_B4_0_G5_3 = 0x01,
}
impl Rgb565Endian {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rgb565Endian {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rgb565Endian {
    #[inline(always)]
    fn from(val: u8) -> Rgb565Endian {
        Rgb565Endian::from_bits(val)
    }
}
impl From<Rgb565Endian> for u8 {
    #[inline(always)]
    fn from(val: Rgb565Endian) -> u8 {
        Rgb565Endian::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleAccessType {
    Command = 0x0,
    Data = 0x01,
}
impl SingleAccessType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleAccessType {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleAccessType {
    #[inline(always)]
    fn from(val: u8) -> SingleAccessType {
        SingleAccessType::from_bits(val)
    }
}
impl From<SingleAccessType> for u8 {
    #[inline(always)]
    fn from(val: SingleAccessType) -> u8 {
        SingleAccessType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiAccessLen {
    #[doc = "1 byte"]
    Bytes1 = 0x0,
    #[doc = "2 bytes"]
    Bytes2 = 0x01,
    #[doc = "3 bytes"]
    Bytes3 = 0x02,
    #[doc = "4 bytes"]
    Bytes4 = 0x03,
}
impl SpiAccessLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiAccessLen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiAccessLen {
    #[inline(always)]
    fn from(val: u8) -> SpiAccessLen {
        SpiAccessLen::from_bits(val)
    }
}
impl From<SpiAccessLen> for u8 {
    #[inline(always)]
    fn from(val: SpiAccessLen) -> u8 {
        SpiAccessLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiClkInit {
    #[doc = "SPI CLK idle state is high"]
    High = 0x0,
    #[doc = "SPI CLK idle state is low"]
    Low = 0x01,
}
impl SpiClkInit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiClkInit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiClkInit {
    #[inline(always)]
    fn from(val: u8) -> SpiClkInit {
        SpiClkInit::from_bits(val)
    }
}
impl From<SpiClkInit> for u8 {
    #[inline(always)]
    fn from(val: SpiClkInit) -> u8 {
        SpiClkInit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiClkPol {
    Normal = 0x0,
    Inverted = 0x01,
}
impl SpiClkPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiClkPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiClkPol {
    #[inline(always)]
    fn from(val: u8) -> SpiClkPol {
        SpiClkPol::from_bits(val)
    }
}
impl From<SpiClkPol> for u8 {
    #[inline(always)]
    fn from(val: SpiClkPol) -> u8 {
        SpiClkPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiLcdFormat {
    #[doc = "8-bit RGB 3:3:2"]
    Rgb332 = 0x0,
    #[doc = "16-bit RGB 5:6:5"]
    Rgb565 = 0x01,
    #[doc = "24-bit RGB 8:8:8"]
    Rgb888 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SpiLcdFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiLcdFormat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiLcdFormat {
    #[inline(always)]
    fn from(val: u8) -> SpiLcdFormat {
        SpiLcdFormat::from_bits(val)
    }
}
impl From<SpiLcdFormat> for u8 {
    #[inline(always)]
    fn from(val: SpiLcdFormat) -> u8 {
        SpiLcdFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiLineMode {
    #[doc = "4-line"]
    FourLine = 0x0,
    #[doc = "4-line with 2 data line"]
    FourLine2Data = 0x01,
    #[doc = "4-line with 4 data line"]
    FourLine4Data = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "3-line"]
    ThreeLine = 0x04,
    #[doc = "3-line with 2 data line"]
    ThreeLine2Data = 0x05,
    #[doc = "3-line with 4 data line"]
    ThreeLine4Data = 0x06,
    _RESERVED_7 = 0x07,
}
impl SpiLineMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiLineMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiLineMode {
    #[inline(always)]
    fn from(val: u8) -> SpiLineMode {
        SpiLineMode::from_bits(val)
    }
}
impl From<SpiLineMode> for u8 {
    #[inline(always)]
    fn from(val: SpiLineMode) -> u8 {
        SpiLineMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiRdMode {
    #[doc = "Normal read. Send write request before read."]
    Normal = 0x0,
    #[doc = "Direct read. Read data without write request."]
    Direct = 0x01,
}
impl SpiRdMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiRdMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiRdMode {
    #[inline(always)]
    fn from(val: u8) -> SpiRdMode {
        SpiRdMode::from_bits(val)
    }
}
impl From<SpiRdMode> for u8 {
    #[inline(always)]
    fn from(val: SpiRdMode) -> u8 {
        SpiRdMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiRdSel {
    Line0 = 0x0,
    Line1 = 0x01,
    Line2 = 0x02,
    Line3 = 0x03,
}
impl SpiRdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiRdSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiRdSel {
    #[inline(always)]
    fn from(val: u8) -> SpiRdSel {
        SpiRdSel::from_bits(val)
    }
}
impl From<SpiRdSel> for u8 {
    #[inline(always)]
    fn from(val: SpiRdSel) -> u8 {
        SpiRdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TargetLcd {
    LcdPanel0 = 0x0,
    LcdPanel1 = 0x01,
    AhbLcd = 0x02,
    AhbRam = 0x03,
}
impl TargetLcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TargetLcd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TargetLcd {
    #[inline(always)]
    fn from(val: u8) -> TargetLcd {
        TargetLcd::from_bits(val)
    }
}
impl From<TargetLcd> for u8 {
    #[inline(always)]
    fn from(val: TargetLcd) -> u8 {
        TargetLcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TeMode {
    VsyncOnly = 0x0,
    VsyncHsync = 0x01,
}
impl TeMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TeMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TeMode {
    #[inline(always)]
    fn from(val: u8) -> TeMode {
        TeMode::from_bits(val)
    }
}
impl From<TeMode> for u8 {
    #[inline(always)]
    fn from(val: TeMode) -> u8 {
        TeMode::to_bits(val)
    }
}
