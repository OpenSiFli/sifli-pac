#[doc = "CLK Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCtrl(pub u32);
impl ClkCtrl {
    #[doc = "div ratio from clk_sys"]
    #[inline(always)]
    pub const fn clk_div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "div ratio from clk_sys"]
    #[inline(always)]
    pub fn set_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "0: select clk_div as clk for SPI controller 1: select clk_sys as clk for SPI controller"]
    #[inline(always)]
    pub const fn clk_sel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0: select clk_div as clk for SPI controller 1: select clk_sys as clk for SPI controller"]
    #[inline(always)]
    pub fn set_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "enable clk for internal logic"]
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "enable clk for internal logic"]
    #[inline(always)]
    pub fn set_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Select spi_di source. 0: from port SPI_DI. 1: from port SPI_DIO."]
    #[inline(always)]
    pub const fn spi_di_sel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Select spi_di source. 0: from port SPI_DI. 1: from port SPI_DIO."]
    #[inline(always)]
    pub fn set_spi_di_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for ClkCtrl {
    #[inline(always)]
    fn default() -> ClkCtrl {
        ClkCtrl(0)
    }
}
impl core::fmt::Debug for ClkCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkCtrl")
            .field("clk_div", &self.clk_div())
            .field("clk_sel", &self.clk_sel())
            .field("clk_en", &self.clk_en())
            .field("spi_di_sel", &self.spi_di_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ClkCtrl {{ clk_div: {=u8:?}, clk_sel: {=bool:?}, clk_en: {=bool:?}, spi_di_sel: {=bool:?} }}" , self . clk_div () , self . clk_sel () , self . clk_en () , self . spi_di_sel ())
    }
}
#[doc = "SPI DATA Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "DATA This field is used for data to be written to the TXFIFO read from the RXFIFO."]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DATA This field is used for data to be written to the TXFIFO read from the RXFIFO."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Data {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoCtrl(pub u32);
impl FifoCtrl {
    #[doc = "TXFIFO Trigger Threshold This field sets the threshold level at which TXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1."]
    #[inline(always)]
    pub const fn tft(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "TXFIFO Trigger Threshold This field sets the threshold level at which TXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1."]
    #[inline(always)]
    pub fn set_tft(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "RXFIFO Trigger Threshold This field sets the threshold level at which RXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1."]
    #[inline(always)]
    pub const fn rft(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "RXFIFO Trigger Threshold This field sets the threshold level at which RXFIFO asserts interrupt. The level should be set to the preferred threshold value minus 1."]
    #[inline(always)]
    pub fn set_rft(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Transmit Service Request Enable 0: TxFIFO DMA service request is disabled 1: TxFIFO DMA service request is enabled"]
    #[inline(always)]
    pub const fn tsre(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Service Request Enable 0: TxFIFO DMA service request is disabled 1: TxFIFO DMA service request is enabled"]
    #[inline(always)]
    pub fn set_tsre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive Service Request Enable 0: RxFIFO DMA service request is disabled 1: RxFIFO DMA service request is enabled"]
    #[inline(always)]
    pub const fn rsre(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Service Request Enable 0: RxFIFO DMA service request is disabled 1: RxFIFO DMA service request is enabled"]
    #[inline(always)]
    pub fn set_rsre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "apb_prdata Read from Rx FIFO Endian 0x0 = apb_prdata\\[31:0\\] = rxfifo_wdata\\[31:0\\] 0x1 = apb_prdata\\[31:0\\] = {rxfifo_wdata\\[15:0\\], rxfifo_wdata\\[31:16\\]} 0x2 = apb_prdata\\[31:0\\]= {rxfifo_wdata\\[7:0\\], rxfifo_wdata\\[15:8\\], rxfifo_wdata\\[23:16\\], rxfifo_wdata\\[31:24\\]} 0x3 = apb_prdata\\[31:0\\]= {rxfifo_wdata\\[23:16\\], rxfifo_wdata\\[31:24\\], rxfifo_wdata\\[7:0\\], rxfifo_wdata\\[15:8\\]}"]
    #[inline(always)]
    pub const fn rxfifo_rd_endian(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "apb_prdata Read from Rx FIFO Endian 0x0 = apb_prdata\\[31:0\\] = rxfifo_wdata\\[31:0\\] 0x1 = apb_prdata\\[31:0\\] = {rxfifo_wdata\\[15:0\\], rxfifo_wdata\\[31:16\\]} 0x2 = apb_prdata\\[31:0\\]= {rxfifo_wdata\\[7:0\\], rxfifo_wdata\\[15:8\\], rxfifo_wdata\\[23:16\\], rxfifo_wdata\\[31:24\\]} 0x3 = apb_prdata\\[31:0\\]= {rxfifo_wdata\\[23:16\\], rxfifo_wdata\\[31:24\\], rxfifo_wdata\\[7:0\\], rxfifo_wdata\\[15:8\\]}"]
    #[inline(always)]
    pub fn set_rxfifo_rd_endian(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "apb_pwdata Write to TxFIFO Endian 0x0: txfifo_wdata\\[31:0\\] = apb_pwdata\\[31:0\\] 0x1: fifo_wdata\\[31:0\\] = {apb_pwdata\\[15:0\\], apb_pwdata\\[31:16\\]} 0x2: txfifo_wdata\\[31:0\\] = {apb_pwdata\\[7:0\\], apb_pwdata\\[15:8\\], apb_pwdata\\[23:16\\], apb_pwdata\\[31:24\\]} 0x3: txfifo_wdata\\[31:0\\] = {apb_pwdata\\[23:16\\], apb_pwdata\\[31:24\\], apb_pwdata\\[7:0\\], apb_pwdata\\[15:8\\]}"]
    #[inline(always)]
    pub const fn txfifo_wr_endian(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "apb_pwdata Write to TxFIFO Endian 0x0: txfifo_wdata\\[31:0\\] = apb_pwdata\\[31:0\\] 0x1: fifo_wdata\\[31:0\\] = {apb_pwdata\\[15:0\\], apb_pwdata\\[31:16\\]} 0x2: txfifo_wdata\\[31:0\\] = {apb_pwdata\\[7:0\\], apb_pwdata\\[15:8\\], apb_pwdata\\[23:16\\], apb_pwdata\\[31:24\\]} 0x3: txfifo_wdata\\[31:0\\] = {apb_pwdata\\[23:16\\], apb_pwdata\\[31:24\\], apb_pwdata\\[7:0\\], apb_pwdata\\[15:8\\]}"]
    #[inline(always)]
    pub fn set_txfifo_wr_endian(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "FIFO Packing Enable 0: FIFO packing mode disabled 1: FIFO packing mode enabled"]
    #[inline(always)]
    pub const fn fpcke(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Packing Enable 0: FIFO packing mode disabled 1: FIFO packing mode enabled"]
    #[inline(always)]
    pub fn set_fpcke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Rx FIFO Auto Full Control: After this field is set to 1 and the SPI controller is operating in master mode, the controller FSM returns to IDLE state and stops the SPI_CLK. When Rx FIFO is full, the controller FSM continues transferring data after the RxFIFO is not full. This field is used to avoid an RxFIFO overrun issue. 1: Enable Rx FIFO auto full control"]
    #[inline(always)]
    pub const fn rxfifo_auto_full_ctrl(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO Auto Full Control: After this field is set to 1 and the SPI controller is operating in master mode, the controller FSM returns to IDLE state and stops the SPI_CLK. When Rx FIFO is full, the controller FSM continues transferring data after the RxFIFO is not full. This field is used to avoid an RxFIFO overrun issue. 1: Enable Rx FIFO auto full control"]
    #[inline(always)]
    pub fn set_rxfifo_auto_full_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for FifoCtrl {
    #[inline(always)]
    fn default() -> FifoCtrl {
        FifoCtrl(0)
    }
}
impl core::fmt::Debug for FifoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FifoCtrl")
            .field("tft", &self.tft())
            .field("rft", &self.rft())
            .field("tsre", &self.tsre())
            .field("rsre", &self.rsre())
            .field("rxfifo_rd_endian", &self.rxfifo_rd_endian())
            .field("txfifo_wr_endian", &self.txfifo_wr_endian())
            .field("fpcke", &self.fpcke())
            .field("rxfifo_auto_full_ctrl", &self.rxfifo_auto_full_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FifoCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "FifoCtrl {{ tft: {=u8:?}, rft: {=u8:?}, tsre: {=bool:?}, rsre: {=bool:?}, rxfifo_rd_endian: {=u8:?}, txfifo_wr_endian: {=u8:?}, fpcke: {=bool:?}, rxfifo_auto_full_ctrl: {=bool:?} }}" , self . tft () , self . rft () , self . tsre () , self . rsre () , self . rxfifo_rd_endian () , self . txfifo_wr_endian () , self . fpcke () , self . rxfifo_auto_full_ctrl ())
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte(pub u32);
impl Inte {
    #[doc = "Receiver Time-out Interrupt Enable 0: Receiver time-out interrupt is disabled 1: Receiver time-out interrupt is enabled"]
    #[inline(always)]
    pub const fn tinte(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Time-out Interrupt Enable 0: Receiver time-out interrupt is disabled 1: Receiver time-out interrupt is enabled"]
    #[inline(always)]
    pub fn set_tinte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Interrupt Enable 0: RxFIFO threshold-level-reached interrupt is disabled 1: RxFIFO threshold-level-reached interrupt is enabled"]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Interrupt Enable 0: RxFIFO threshold-level-reached interrupt is disabled 1: RxFIFO threshold-level-reached interrupt is enabled"]
    #[inline(always)]
    pub fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit FIFO Interrupt Enable 0: TxFIFO threshold-level-reached interrupt is disabled 1: TxFIFO threshold-level-reached interrupt is enabled"]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Interrupt Enable 0: TxFIFO threshold-level-reached interrupt is disabled 1: TxFIFO threshold-level-reached interrupt is enabled"]
    #[inline(always)]
    pub fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO Overrun Interrupt Mask 0: ROR events generate an SPI interrupt 1: ROR events do NOT generate an SPI interrupt"]
    #[inline(always)]
    pub const fn rim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Overrun Interrupt Mask 0: ROR events generate an SPI interrupt 1: ROR events do NOT generate an SPI interrupt"]
    #[inline(always)]
    pub fn set_rim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit FIFO Underrun Interrupt Mask 0 : TUR events generate an SPI interrupt 1 : TUR events do NOT generate an SPI interrupt"]
    #[inline(always)]
    pub const fn tim(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Underrun Interrupt Mask 0 : TUR events generate an SPI interrupt 1 : TUR events do NOT generate an SPI interrupt"]
    #[inline(always)]
    pub fn set_tim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Inte {
    #[inline(always)]
    fn default() -> Inte {
        Inte(0)
    }
}
impl core::fmt::Debug for Inte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inte")
            .field("tinte", &self.tinte())
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .field("rim", &self.rim())
            .field("tim", &self.tim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inte {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inte {{ tinte: {=bool:?}, rie: {=bool:?}, tie: {=bool:?}, rim: {=bool:?}, tim: {=bool:?} }}" , self . tinte () , self . rie () , self . tie () , self . rim () , self . tim ())
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
#[doc = "RWOT Counter Cycles Match Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RwotCcm(pub u32);
impl RwotCcm {
    #[doc = "It's just total SPI_CLK Cycles. The value of this register defines the total number of SPI_CLK cycles when SPI controller works in master and RWOT mode. When the rwot_counter matches this value, SPI controller returns to IDLE state and does not output SPI_CLK anymore."]
    #[inline(always)]
    pub const fn rwotccm(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "It's just total SPI_CLK Cycles. The value of this register defines the total number of SPI_CLK cycles when SPI controller works in master and RWOT mode. When the rwot_counter matches this value, SPI controller returns to IDLE state and does not output SPI_CLK anymore."]
    #[inline(always)]
    pub fn set_rwotccm(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RwotCcm {
    #[inline(always)]
    fn default() -> RwotCcm {
        RwotCcm(0)
    }
}
impl core::fmt::Debug for RwotCcm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RwotCcm")
            .field("rwotccm", &self.rwotccm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RwotCcm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RwotCcm {{ rwotccm: {=u32:?} }}", self.rwotccm())
    }
}
#[doc = "RWOT Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RwotCtrl(pub u32);
impl RwotCtrl {
    #[doc = "Receive Without Transmit 0: Transmit/receive mode 1: Receive without transmit mode"]
    #[inline(always)]
    pub const fn rwot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Without Transmit 0: Transmit/receive mode 1: Receive without transmit mode"]
    #[inline(always)]
    pub fn set_rwot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable RWOT Cycle Counter Mode 1: Enable"]
    #[inline(always)]
    pub const fn cycle_rwot_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RWOT Cycle Counter Mode 1: Enable"]
    #[inline(always)]
    pub fn set_cycle_rwot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set RWOT Cycle This field is used to set the value of the RWOT_CCM register to the internal rwot_counter. This field is self-cleared after SSE = 1. 1: Set rwot_counter"]
    #[inline(always)]
    pub const fn set_rwot_cycle(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set RWOT Cycle This field is used to set the value of the RWOT_CCM register to the internal rwot_counter. This field is self-cleared after SSE = 1. 1: Set rwot_counter"]
    #[inline(always)]
    pub fn set_set_rwot_cycle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Internal rwot_counter This field clears the rwot_counter to 0. This field is self cleared after SSE = 1. 1: Clear rwot_counter"]
    #[inline(always)]
    pub const fn clr_rwot_cycle(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Internal rwot_counter This field clears the rwot_counter to 0. This field is self cleared after SSE = 1. 1: Clear rwot_counter"]
    #[inline(always)]
    pub fn set_clr_rwot_cycle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Mask last_sample_flag in RWOT Mode 1: Mask 0: Unmask"]
    #[inline(always)]
    pub const fn mask_rwot_last_sample(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mask last_sample_flag in RWOT Mode 1: Mask 0: Unmask"]
    #[inline(always)]
    pub fn set_mask_rwot_last_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for RwotCtrl {
    #[inline(always)]
    fn default() -> RwotCtrl {
        RwotCtrl(0)
    }
}
impl core::fmt::Debug for RwotCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RwotCtrl")
            .field("rwot", &self.rwot())
            .field("cycle_rwot_en", &self.cycle_rwot_en())
            .field("set_rwot_cycle", &self.set_rwot_cycle())
            .field("clr_rwot_cycle", &self.clr_rwot_cycle())
            .field("mask_rwot_last_sample", &self.mask_rwot_last_sample())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RwotCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RwotCtrl {{ rwot: {=bool:?}, cycle_rwot_en: {=bool:?}, set_rwot_cycle: {=bool:?}, clr_rwot_cycle: {=bool:?}, mask_rwot_last_sample: {=bool:?} }}" , self . rwot () , self . cycle_rwot_en () , self . set_rwot_cycle () , self . clr_rwot_cycle () , self . mask_rwot_last_sample ())
    }
}
#[doc = "RWOT Counter Value Write for Red Request Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RwotCvwrn(pub u32);
impl RwotCvwrn {
    #[doc = "RWOTCVWR This register prevents the risk of instability on rwot_counter value reading, it's only valid after SPI controller has been enabled Write 0 = No effect Write 1 = Capture value of rwot_counter Read: Returns the captured value of rwot_counter"]
    #[inline(always)]
    pub const fn rwotcvwr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RWOTCVWR This register prevents the risk of instability on rwot_counter value reading, it's only valid after SPI controller has been enabled Write 0 = No effect Write 1 = Capture value of rwot_counter Read: Returns the captured value of rwot_counter"]
    #[inline(always)]
    pub fn set_rwotcvwr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RwotCvwrn {
    #[inline(always)]
    fn default() -> RwotCvwrn {
        RwotCvwrn(0)
    }
}
impl core::fmt::Debug for RwotCvwrn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RwotCvwrn")
            .field("rwotcvwr", &self.rwotcvwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RwotCvwrn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RwotCvwrn {{ rwotcvwr: {=u32:?} }}", self.rwotcvwr())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "SPI controller Busy 0: SPI controller is idle or disabled 1: SPI controller is currently transmitting or receiving framed data"]
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI controller Busy 0: SPI controller is idle or disabled 1: SPI controller is currently transmitting or receiving framed data"]
    #[inline(always)]
    pub fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock Synchronization Status 0: SPI controller is ready for slave clock operations 1: SPI controller is currently busy synchronizing slave mode signals"]
    #[inline(always)]
    pub const fn css(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Synchronization Status 0: SPI controller is ready for slave clock operations 1: SPI controller is currently busy synchronizing slave mode signals"]
    #[inline(always)]
    pub fn set_css(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receiver Time-out Interrupt 0: No receiver time-out is pending 1: Receiver time-out pending, causes an interrupt request"]
    #[inline(always)]
    pub const fn tint(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Time-out Interrupt 0: No receiver time-out is pending 1: Receiver time-out pending, causes an interrupt request"]
    #[inline(always)]
    pub fn set_tint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO Service Request 0: TX FIFO level exceeds the TFT threshold (TFT + 1) or SPI controller is disabled 1: TXFIFO level is at or below TFT threshold (TFT + 1), causes an interrupt request"]
    #[inline(always)]
    pub const fn tfs(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Service Request 0: TX FIFO level exceeds the TFT threshold (TFT + 1) or SPI controller is disabled 1: TXFIFO level is at or below TFT threshold (TFT + 1), causes an interrupt request"]
    #[inline(always)]
    pub fn set_tfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit FIFO Not Full 0: TXFIFO is full 1: TXFIFO is not full"]
    #[inline(always)]
    pub const fn tnf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Not Full 0: TXFIFO is full 1: TXFIFO is not full"]
    #[inline(always)]
    pub fn set_tnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO Level This field is the number of entries in TXFIFO.When the value 0x0 is read, the TXFIFO is either empty or full, and software should read the \\[Transmit FIFO Not Full\\] field."]
    #[inline(always)]
    pub const fn tfl(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO Level This field is the number of entries in TXFIFO.When the value 0x0 is read, the TXFIFO is either empty or full, and software should read the \\[Transmit FIFO Not Full\\] field."]
    #[inline(always)]
    pub fn set_tfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
    }
    #[doc = "Transmit FIFO Underrun 0: The TXFIFO has not experienced an underrun 1: A read from the TXFIFO was attempted when the TXFIFO was empty, causes an interrupt if it is enabled (\\[Transmit FIFO Underrun Interrupt Mask\\] in the INT EN Register is 0)"]
    #[inline(always)]
    pub const fn tur(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Underrun 0: The TXFIFO has not experienced an underrun 1: A read from the TXFIFO was attempted when the TXFIFO was empty, causes an interrupt if it is enabled (\\[Transmit FIFO Underrun Interrupt Mask\\] in the INT EN Register is 0)"]
    #[inline(always)]
    pub fn set_tur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Receive FIFO Service Request 0: RXFIFO level is at or below RFT threshold (RFT) or SPI controller is disabled 1: RXFIFO level exceeds RFT threshold (RFT), causes an interrupt request"]
    #[inline(always)]
    pub const fn rfs(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Service Request 0: RXFIFO level is at or below RFT threshold (RFT) or SPI controller is disabled 1: RXFIFO level exceeds RFT threshold (RFT), causes an interrupt request"]
    #[inline(always)]
    pub fn set_rfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive FIFO Not Empty 0: RXFIFO is empty 1: RXFIFO is not empty"]
    #[inline(always)]
    pub const fn rne(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Not Empty 0: RXFIFO is empty 1: RXFIFO is not empty"]
    #[inline(always)]
    pub fn set_rne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Receive FIFO Level This field is the number of entries minus one in RXFIFO. When the value 0xF is read, the RXFIFO is either empty or full, and software should read the \\[Receive FIFO Not Empty\\] field."]
    #[inline(always)]
    pub const fn rfl(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive FIFO Level This field is the number of entries minus one in RXFIFO. When the value 0xF is read, the RXFIFO is either empty or full, and software should read the \\[Receive FIFO Not Empty\\] field."]
    #[inline(always)]
    pub fn set_rfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
    }
    #[doc = "Receive FIFO Overrun 0: RXFIFO has not experienced an overrun 1: Attempted data write to full RXFIFO, causes an interrupt request"]
    #[inline(always)]
    pub const fn ror(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Overrun 0: RXFIFO has not experienced an overrun 1: Attempted data write to full RXFIFO, causes an interrupt request"]
    #[inline(always)]
    pub fn set_ror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TX FIFO Odd Sample Status When SPI controller is in packed mode, the number of samples in the TX FIFO is: (\\[Transmit FIFO Level\\]*2 + this field), when \\[Transmit FIFO Not Full\\] = 1 32, when \\[Transmit FIFO Not Full\\] = 0. The TX FIFO cannot accept new data when \\[Transmit FIFO Not Full\\] = 1 and \\[Transmit FIFO Level\\] = 15 and this field = 1. (The TX FIFO has 31 samples). 0: TxFIFO entry has an even number of samples 1: TxFIFO entry has an odd number of samples Note that this bit needs to be read only when FIFO Packing is enabled (\\[FIFO Packing Enable\\] in the FIFO Control Register is set). Otherwise, this bit is zero."]
    #[inline(always)]
    pub const fn tx_oss(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO Odd Sample Status When SPI controller is in packed mode, the number of samples in the TX FIFO is: (\\[Transmit FIFO Level\\]*2 + this field), when \\[Transmit FIFO Not Full\\] = 1 32, when \\[Transmit FIFO Not Full\\] = 0. The TX FIFO cannot accept new data when \\[Transmit FIFO Not Full\\] = 1 and \\[Transmit FIFO Level\\] = 15 and this field = 1. (The TX FIFO has 31 samples). 0: TxFIFO entry has an even number of samples 1: TxFIFO entry has an odd number of samples Note that this bit needs to be read only when FIFO Packing is enabled (\\[FIFO Packing Enable\\] in the FIFO Control Register is set). Otherwise, this bit is zero."]
    #[inline(always)]
    pub fn set_tx_oss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Odd Sample Status 0: RxFIFO entry has two samples 1: RxFIFO entry has one sample Note that this bit needs to be looked at only when FIFO Packing is enabled (FPCKE field in FIFO Control Register is set). Otherwise, this bit is zero. When SPI controller is in Packed mode and the CPU is used instead of DMA to read the RxFIFO, the CPU should make sure that \\[Receive FIFO Not Empty\\] = 1 AND this field = 0 before it attempts to read the RxFIFO."]
    #[inline(always)]
    pub const fn oss(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Odd Sample Status 0: RxFIFO entry has two samples 1: RxFIFO entry has one sample Note that this bit needs to be looked at only when FIFO Packing is enabled (FPCKE field in FIFO Control Register is set). Otherwise, this bit is zero. When SPI controller is in Packed mode and the CPU is used instead of DMA to read the RxFIFO, the CPU should make sure that \\[Receive FIFO Not Empty\\] = 1 AND this field = 0 before it attempts to read the RxFIFO."]
    #[inline(always)]
    pub fn set_oss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
            .field("bsy", &self.bsy())
            .field("css", &self.css())
            .field("tint", &self.tint())
            .field("tfs", &self.tfs())
            .field("tnf", &self.tnf())
            .field("tfl", &self.tfl())
            .field("tur", &self.tur())
            .field("rfs", &self.rfs())
            .field("rne", &self.rne())
            .field("rfl", &self.rfl())
            .field("ror", &self.ror())
            .field("tx_oss", &self.tx_oss())
            .field("oss", &self.oss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Status {{ bsy: {=bool:?}, css: {=bool:?}, tint: {=bool:?}, tfs: {=bool:?}, tnf: {=bool:?}, tfl: {=u8:?}, tur: {=bool:?}, rfs: {=bool:?}, rne: {=bool:?}, rfl: {=u8:?}, ror: {=bool:?}, tx_oss: {=bool:?}, oss: {=bool:?} }}" , self . bsy () , self . css () , self . tint () , self . tfs () , self . tnf () , self . tfl () , self . tur () , self . rfs () , self . rne () , self . rfl () , self . ror () , self . tx_oss () , self . oss ())
    }
}
#[doc = "SPI Time Out Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct To(pub u32);
impl To {
    #[doc = "Timeout Value TIMEOUT value is the value (0 to 2^24-1) that defines the time-out interval. The time-out interval is given by the equation shown in the TIMEOUT Interval Equation."]
    #[inline(always)]
    pub const fn timeout(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Timeout Value TIMEOUT value is the value (0 to 2^24-1) that defines the time-out interval. The time-out interval is given by the equation shown in the TIMEOUT Interval Equation."]
    #[inline(always)]
    pub fn set_timeout(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for To {
    #[inline(always)]
    fn default() -> To {
        To(0)
    }
}
impl core::fmt::Debug for To {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("To")
            .field("timeout", &self.timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for To {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "To {{ timeout: {=u32:?} }}", self.timeout())
    }
}
#[doc = "Top Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TopCtrl(pub u32);
impl TopCtrl {
    #[doc = "SPI controller Enable 0: SPI controller is disabled 1: SPI controller is enabled"]
    #[inline(always)]
    pub const fn sse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI controller Enable 0: SPI controller is disabled 1: SPI controller is enabled"]
    #[inline(always)]
    pub fn set_sse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame Format 0x0: Motorola* Serial Peripheral Interface (SPI) 0x1: Texas Instruments* Synchronous Serial Protocol (SSP) 0x2: National Semiconductor Microwire* 0x3: RSVD"]
    #[inline(always)]
    pub const fn frf(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Frame Format 0x0: Motorola* Serial Peripheral Interface (SPI) 0x1: Texas Instruments* Synchronous Serial Protocol (SSP) 0x2: National Semiconductor Microwire* 0x3: RSVD"]
    #[inline(always)]
    pub fn set_frf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "SPI_CLK Direction 0: Master mode, SPI controller drives SPI_CLK 1: Slave mode, SPI controller receives SPI_CLK"]
    #[inline(always)]
    pub const fn sclkdir(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SPI_CLK Direction 0: Master mode, SPI controller drives SPI_CLK 1: Slave mode, SPI controller receives SPI_CLK"]
    #[inline(always)]
    pub fn set_sclkdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SPI_CS Direction 0: Master mode, SPI controller drives SPI_CS 1: Slave mode, SPI controller receives SPI_CS"]
    #[inline(always)]
    pub const fn sfrmdir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SPI_CS Direction 0: Master mode, SPI controller drives SPI_CS 1: Slave mode, SPI controller receives SPI_CS"]
    #[inline(always)]
    pub fn set_sfrmdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SPI controller Work data size, register bits value 0~31 indicated data size 1~32 bits, usually use data size 8bits, 16bits, 24bits, 32bits"]
    #[inline(always)]
    pub const fn dss(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "SPI controller Work data size, register bits value 0~31 indicated data size 1~32 bits, usually use data size 8bits, 16bits, 24bits, 32bits"]
    #[inline(always)]
    pub fn set_dss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Motorola SPI SPI_CLK Polarity Setting 0: The inactive or idle state of SPI_CLK is low 1: The inactive or idle state of SPI_CLK is high"]
    #[inline(always)]
    pub const fn spo(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Motorola SPI SPI_CLK Polarity Setting 0: The inactive or idle state of SPI_CLK is low 1: The inactive or idle state of SPI_CLK is high"]
    #[inline(always)]
    pub fn set_spo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Motorola SPI SPI_CLK phase setting 0: SPI_CLK is inactive until one cycle after the start of a frame and active until 1/2 cycle before the end of a frame 1: SPI_CLK is inactive until 1/2 cycle after the start of a frame and active until one cycle before the end of a frame"]
    #[inline(always)]
    pub const fn sph(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Motorola SPI SPI_CLK phase setting 0: SPI_CLK is inactive until one cycle after the start of a frame and active until 1/2 cycle before the end of a frame 1: SPI_CLK is inactive until 1/2 cycle after the start of a frame and active until one cycle before the end of a frame"]
    #[inline(always)]
    pub fn set_sph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Trailing Byte 0: Trailing bytes are handled by CPU 1: Trailing bytes are handled by DMA bursts"]
    #[inline(always)]
    pub const fn trail(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Trailing Byte 0: Trailing bytes are handled by CPU 1: Trailing bytes are handled by DMA bursts"]
    #[inline(always)]
    pub fn set_trail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Hold Frame Low Control 0:After this field is set to 1 and the SPI controller is operating in master mode,the output frame signal SPI_CS will be determined by control FSM. 1:After this field is set to 1 and the SPI controller is operating in master mode, the output frame signal SPI_CS will hold low."]
    #[inline(always)]
    pub const fn hold_frame_low(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Hold Frame Low Control 0:After this field is set to 1 and the SPI controller is operating in master mode,the output frame signal SPI_CS will be determined by control FSM. 1:After this field is set to 1 and the SPI controller is operating in master mode, the output frame signal SPI_CS will hold low."]
    #[inline(always)]
    pub fn set_hold_frame_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Invert Frame Signal 0: SPI_CS polarity is as defined in protocol 1: SPI_CS will be inverted from normal-SPI_CS"]
    #[inline(always)]
    pub const fn ifs(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Invert Frame Signal 0: SPI_CS polarity is as defined in protocol 1: SPI_CS will be inverted from normal-SPI_CS"]
    #[inline(always)]
    pub fn set_ifs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SPI_DO Three-State Enable 0: SPI_DO output signal is not three-stated 1: SPI_DO is three-stated when not transmitting data"]
    #[inline(always)]
    pub const fn tte(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SPI_DO Three-State Enable 0: SPI_DO output signal is not three-stated 1: SPI_DO is three-stated when not transmitting data"]
    #[inline(always)]
    pub fn set_tte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SPI_DO Three-state Enable On Last Phase (can be set only when TI-SSP) 0: SPI_DO is three-stated 1/2 clock cycle after the beginning of the LSB 1: SPI_DO output signal is three-stated on the clock edge that ends the LSB"]
    #[inline(always)]
    pub const fn ttelp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SPI_DO Three-state Enable On Last Phase (can be set only when TI-SSP) 0: SPI_DO is three-stated 1/2 clock cycle after the beginning of the LSB 1: SPI_DO output signal is three-stated on the clock edge that ends the LSB"]
    #[inline(always)]
    pub fn set_ttelp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for TopCtrl {
    #[inline(always)]
    fn default() -> TopCtrl {
        TopCtrl(0)
    }
}
impl core::fmt::Debug for TopCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TopCtrl")
            .field("sse", &self.sse())
            .field("frf", &self.frf())
            .field("sclkdir", &self.sclkdir())
            .field("sfrmdir", &self.sfrmdir())
            .field("dss", &self.dss())
            .field("spo", &self.spo())
            .field("sph", &self.sph())
            .field("trail", &self.trail())
            .field("hold_frame_low", &self.hold_frame_low())
            .field("ifs", &self.ifs())
            .field("tte", &self.tte())
            .field("ttelp", &self.ttelp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TopCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TopCtrl {{ sse: {=bool:?}, frf: {=u8:?}, sclkdir: {=bool:?}, sfrmdir: {=bool:?}, dss: {=u8:?}, spo: {=bool:?}, sph: {=bool:?}, trail: {=bool:?}, hold_frame_low: {=bool:?}, ifs: {=bool:?}, tte: {=bool:?}, ttelp: {=bool:?} }}" , self . sse () , self . frf () , self . sclkdir () , self . sfrmdir () , self . dss () , self . spo () , self . sph () , self . trail () , self . hold_frame_low () , self . ifs () , self . tte () , self . ttelp ())
    }
}
#[doc = "Three Wire Mode Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TriwireCtrl(pub u32);
impl TriwireCtrl {
    #[doc = "SPI_THREE_WIRE_MODE_EN 0: normal mode 1: enable TRI-WIRE mode"]
    #[inline(always)]
    pub const fn spi_tri_wire_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI_THREE_WIRE_MODE_EN 0: normal mode 1: enable TRI-WIRE mode"]
    #[inline(always)]
    pub fn set_spi_tri_wire_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TXD_OEN control when TRI-WIRE mode 1: SPI_DIO is input 0: SPI_DIO is output"]
    #[inline(always)]
    pub const fn txd_oen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TXD_OEN control when TRI-WIRE mode 1: SPI_DIO is input 0: SPI_DIO is output"]
    #[inline(always)]
    pub fn set_txd_oen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WORK_WIDTH_DYN_CHNAGE 1: SW can dynamicly change TOP_CTRL\\[9:5\\] without disabling TOP_CTRL\\[0\\] and re-enabling TOP_CTRL\\[0\\]"]
    #[inline(always)]
    pub const fn work_width_dyn_change(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WORK_WIDTH_DYN_CHNAGE 1: SW can dynamicly change TOP_CTRL\\[9:5\\] without disabling TOP_CTRL\\[0\\] and re-enabling TOP_CTRL\\[0\\]"]
    #[inline(always)]
    pub fn set_work_width_dyn_change(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for TriwireCtrl {
    #[inline(always)]
    fn default() -> TriwireCtrl {
        TriwireCtrl(0)
    }
}
impl core::fmt::Debug for TriwireCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TriwireCtrl")
            .field("spi_tri_wire_en", &self.spi_tri_wire_en())
            .field("txd_oen", &self.txd_oen())
            .field("work_width_dyn_change", &self.work_width_dyn_change())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TriwireCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TriwireCtrl {{ spi_tri_wire_en: {=bool:?}, txd_oen: {=bool:?}, work_width_dyn_change: {=bool:?} }}" , self . spi_tri_wire_en () , self . txd_oen () , self . work_width_dyn_change ())
    }
}
