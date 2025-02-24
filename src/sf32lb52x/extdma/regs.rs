#[doc = "channel control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "extdma enable. Will be cleared if ccr_reset is written"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "extdma enable. Will be cleared if ccr_reset is written"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "transfer complete interrupt enable 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "transfer complete interrupt enable 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "half transfer interrupt enable 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "half transfer interrupt enable 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "transfer error interrupt enable 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error interrupt enable 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "destination increment mode Defines the increment mode for each DMA transfer to the destination memory. 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn dstinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "destination increment mode Defines the increment mode for each DMA transfer to the destination memory. 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_dstinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "source increment mode Defines the increment mode for each DMA transfer to the source memory. 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn srcinc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "source increment mode Defines the increment mode for each DMA transfer to the source memory. 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_srcinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "destination size Defines the data size of each DMA transfer to the destination memory. Should be fixed to 10 (32 bits), word access allowed only."]
    #[inline(always)]
    pub const fn dstsize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "destination size Defines the data size of each DMA transfer to the destination memory. Should be fixed to 10 (32 bits), word access allowed only."]
    #[inline(always)]
    pub fn set_dstsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "source size Defines the data size of each DMA transfer to the source memory. Should be fixed to 10 (32 bits), word access allowed only."]
    #[inline(always)]
    pub const fn srcsize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "source size Defines the data size of each DMA transfer to the source memory. Should be fixed to 10 (32 bits), word access allowed only."]
    #[inline(always)]
    pub fn set_srcsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "destination burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)"]
    #[inline(always)]
    pub const fn dstburst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "destination burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)"]
    #[inline(always)]
    pub fn set_dstburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "source burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)"]
    #[inline(always)]
    pub const fn srcburst(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "source burst transfer configuration 00: single transfer 01: INCR4 (incremental burst of 4 beats) 10: INCR8 (incremental burst of 8 beats) 11: INCR16 (incremental burst of 16 beats)"]
    #[inline(always)]
    pub fn set_srcburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Software reset, will clear extdma status. Active high. Will be cleared by HW automatically"]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software reset, will clear extdma status. Active high. Will be cleared by HW automatically"]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
        f.debug_struct("Ccr")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dstinc", &self.dstinc())
            .field("srcinc", &self.srcinc())
            .field("dstsize", &self.dstsize())
            .field("srcsize", &self.srcsize())
            .field("dstburst", &self.dstburst())
            .field("srcburst", &self.srcburst())
            .field("reset", &self.reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccr {{ en: {=bool:?}, tcie: {=bool:?}, htie: {=bool:?}, teie: {=bool:?}, dstinc: {=bool:?}, srcinc: {=bool:?}, dstsize: {=u8:?}, srcsize: {=u8:?}, dstburst: {=u8:?}, srcburst: {=u8:?}, reset: {=bool:?} }}" , self . en () , self . tcie () , self . htie () , self . teie () , self . dstinc () , self . srcinc () , self . dstsize () , self . srcsize () , self . dstburst () , self . srcburst () , self . reset ())
    }
}
#[doc = "number of data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr(pub u32);
impl Cndtr {
    #[doc = "number of data to transfer (0 to 2^20 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached. If this field is zero, no transfer can be served whatever the channel enabled or not"]
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "number of data to transfer (0 to 2^20 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached. If this field is zero, no transfer can be served whatever the channel enabled or not"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Cndtr {
    #[inline(always)]
    fn default() -> Cndtr {
        Cndtr(0)
    }
}
impl core::fmt::Debug for Cndtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cndtr").field("ndt", &self.ndt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cndtr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cndtr {{ ndt: {=u32:?} }}", self.ndt())
    }
}
#[doc = "destination 0 address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dstar(pub u32);
impl Dstar {
    #[doc = "destination address It contains the base address of the destination data to be written. Should be word aligned"]
    #[inline(always)]
    pub const fn dstaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "destination address It contains the base address of the destination data to be written. Should be word aligned"]
    #[inline(always)]
    pub fn set_dstaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dstar {
    #[inline(always)]
    fn default() -> Dstar {
        Dstar(0)
    }
}
impl core::fmt::Debug for Dstar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dstar")
            .field("dstaddr", &self.dstaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dstar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dstar {{ dstaddr: {=u32:?} }}", self.dstaddr())
    }
}
#[doc = "interrupt clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc = "CGIF, global interrupt flag clear"]
    #[inline(always)]
    pub const fn cgif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CGIF, global interrupt flag clear"]
    #[inline(always)]
    pub fn set_cgif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CTCIF, transfer complete flag clear"]
    #[inline(always)]
    pub const fn ctcif(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CTCIF, transfer complete flag clear"]
    #[inline(always)]
    pub fn set_ctcif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CHTIF, half transfer flag clear"]
    #[inline(always)]
    pub const fn chtif(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CHTIF, half transfer flag clear"]
    #[inline(always)]
    pub fn set_chtif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTEIF, transfer error flag clear"]
    #[inline(always)]
    pub const fn cteif(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTEIF, transfer error flag clear"]
    #[inline(always)]
    pub fn set_cteif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ifcr {
    #[inline(always)]
    fn default() -> Ifcr {
        Ifcr(0)
    }
}
impl core::fmt::Debug for Ifcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifcr")
            .field("cgif", &self.cgif())
            .field("ctcif", &self.ctcif())
            .field("chtif", &self.chtif())
            .field("cteif", &self.cteif())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifcr {{ cgif: {=bool:?}, ctcif: {=bool:?}, chtif: {=bool:?}, cteif: {=bool:?} }}",
            self.cgif(),
            self.ctcif(),
            self.chtif(),
            self.cteif()
        )
    }
}
#[doc = "interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "GIF, global interrupt flag"]
    #[inline(always)]
    pub const fn gif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GIF, global interrupt flag"]
    #[inline(always)]
    pub fn set_gif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIF, transfer complete flag"]
    #[inline(always)]
    pub const fn tcif(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF, transfer complete flag"]
    #[inline(always)]
    pub fn set_tcif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIF, half transfer flag"]
    #[inline(always)]
    pub const fn htif(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF, half transfer flag"]
    #[inline(always)]
    pub fn set_htif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIF, transfer error flag"]
    #[inline(always)]
    pub const fn teif(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF, transfer error flag"]
    #[inline(always)]
    pub fn set_teif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("gif", &self.gif())
            .field("tcif", &self.tcif())
            .field("htif", &self.htif())
            .field("teif", &self.teif())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isr {{ gif: {=bool:?}, tcif: {=bool:?}, htif: {=bool:?}, teif: {=bool:?} }}",
            self.gif(),
            self.tcif(),
            self.htif(),
            self.teif()
        )
    }
}
#[doc = "source address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcar(pub u32);
impl Srcar {
    #[doc = "source address It contains the base address of the source data to be read. Should be word aligned"]
    #[inline(always)]
    pub const fn srcaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "source address It contains the base address of the source data to be read. Should be word aligned"]
    #[inline(always)]
    pub fn set_srcaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Srcar {
    #[inline(always)]
    fn default() -> Srcar {
        Srcar(0)
    }
}
impl core::fmt::Debug for Srcar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srcar")
            .field("srcaddr", &self.srcaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srcar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srcar {{ srcaddr: {=u32:?} }}", self.srcaddr())
    }
}
