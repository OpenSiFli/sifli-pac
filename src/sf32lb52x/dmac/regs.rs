#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbsr(pub u32);
impl Cbsr {
    #[doc = "burst size in non-m2m mode When BS>1, DMA will transfer for BS times for each request if left NDT is larger than BS, or else transfer for left NDT times. When BS=0 or 1, DMA will always do single transfer for each request. In memory-to-memory mode, BS is ignored."]
    #[inline(always)]
    pub const fn bs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "burst size in non-m2m mode When BS>1, DMA will transfer for BS times for each request if left NDT is larger than BS, or else transfer for left NDT times. When BS=0 or 1, DMA will always do single transfer for each request. In memory-to-memory mode, BS is ignored."]
    #[inline(always)]
    pub fn set_bs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cbsr {
    #[inline(always)]
    fn default() -> Cbsr {
        Cbsr(0)
    }
}
impl core::fmt::Debug for Cbsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbsr").field("bs", &self.bs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cbsr {{ bs: {=u8:?} }}", self.bs())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the ISR register is cleared (by setting the CTEIFx bit of the IFCR register). 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the ISR register is cleared (by setting the CTEIFx bit of the IFCR register). 0: disabled 1: enabled"]
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
    #[doc = "data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. 0: read from peripheral Source attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode. 1: read from memory Destination attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode."]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Dir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dir::from_bits(val as u8)
    }
    #[doc = "data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. 0: read from peripheral Source attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode. 1: read from memory Destination attributes are defined by PSIZE and PINC, plus the CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the CM0ARx register. This is still valid in a peripheral-to-peripheral mode."]
    #[inline(always)]
    pub fn set_dir(&mut self, val: super::vals::Dir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "circular mode 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "circular mode 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved"]
    #[inline(always)]
    pub const fn psize(&self) -> super::vals::Size {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Size::from_bits(val as u8)
    }
    #[doc = "peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: super::vals::Size) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved"]
    #[inline(always)]
    pub const fn msize(&self) -> super::vals::Size {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Size::from_bits(val as u8)
    }
    #[doc = "memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. 00: 8 bits 01: 16 bits 10: 32 bits 11: reserved"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: super::vals::Size) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "priority level 00: low 01: medium 10: high 11: very high"]
    #[inline(always)]
    pub const fn pl(&self) -> super::vals::Pl {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Pl::from_bits(val as u8)
    }
    #[doc = "priority level 00: low 01: medium 10: high 11: very high"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: super::vals::Pl) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "memory-to-memory mode 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "memory-to-memory mode 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccr {{ en: {=bool:?}, tcie: {=bool:?}, htie: {=bool:?}, teie: {=bool:?}, dir: {:?}, circ: {=bool:?}, pinc: {=bool:?}, minc: {=bool:?}, psize: {:?}, msize: {:?}, pl: {:?}, mem2mem: {=bool:?} }}" , self . en () , self . tcie () , self . htie () , self . teie () , self . dir () , self . circ () , self . pinc () , self . minc () , self . psize () , self . msize () , self . pl () , self . mem2mem ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm0ar(pub u32);
impl Cm0ar {
    #[doc = "memory address It contains the base address of the memory from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0."]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "memory address It contains the base address of the memory from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0."]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cm0ar {
    #[inline(always)]
    fn default() -> Cm0ar {
        Cm0ar(0)
    }
}
impl core::fmt::Debug for Cm0ar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cm0ar").field("ma", &self.ma()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cm0ar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cm0ar {{ ma: {=u32:?} }}", self.ma())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr(pub u32);
impl Cndtr {
    #[doc = "number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not)."]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "number of data to transfer (0 to 2^16 - 1) This field is updated by hardware when the channel is enabled: It is decremented after each single DMA 'read followed by write' transfer, indicating the remaining amount of data items to transfer. It is kept at zero when the programmed amount of data to transfer is reached, if the channel is not in circular mode (CIRC = 0 in the CCRx register). It is reloaded automatically by the previously programmed value, when the transfer is complete, if the channel is in circular mode (CIRC = 1). If this field is zero, no transfer can be served whatever the channel status (enabled or not)."]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
        defmt::write!(f, "Cndtr {{ ndt: {=u16:?} }}", self.ndt())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar(pub u32);
impl Cpar {
    #[doc = "peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0."]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0."]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar {
    #[inline(always)]
    fn default() -> Cpar {
        Cpar(0)
    }
}
impl core::fmt::Debug for Cpar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpar").field("pa", &self.pa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpar {{ pa: {=u32:?} }}", self.pa())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cselr(pub u32);
impl Cselr {
    #[doc = "DMA channel 1 selection"]
    #[inline(always)]
    pub const fn cs(&self, n: usize) -> u8 {
        assert!(n < 4usize);
        let offs = 0usize + n * 8usize;
        let val = (self.0 >> offs) & 0x3f;
        val as u8
    }
    #[doc = "DMA channel 1 selection"]
    #[inline(always)]
    pub fn set_cs(&mut self, n: usize, val: u8) {
        assert!(n < 4usize);
        let offs = 0usize + n * 8usize;
        self.0 = (self.0 & !(0x3f << offs)) | (((val as u32) & 0x3f) << offs);
    }
}
impl Default for Cselr {
    #[inline(always)]
    fn default() -> Cselr {
        Cselr(0)
    }
}
impl core::fmt::Debug for Cselr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cselr")
            .field("cs[0]", &self.cs(0usize))
            .field("cs[1]", &self.cs(1usize))
            .field("cs[2]", &self.cs(2usize))
            .field("cs[3]", &self.cs(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cselr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cselr {{ cs[0]: {=u8:?}, cs[1]: {=u8:?}, cs[2]: {=u8:?}, cs[3]: {=u8:?} }}",
            self.cs(0usize),
            self.cs(1usize),
            self.cs(2usize),
            self.cs(3usize)
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc = "CGIF, global interrupt flag clear. Write 1 to clear all TEIF/HTIF/TCIF."]
    #[inline(always)]
    pub const fn cgif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "CGIF, global interrupt flag clear. Write 1 to clear all TEIF/HTIF/TCIF."]
    #[inline(always)]
    pub fn set_cgif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "CTCIF, transfer complete flag clear. Write 1 to clear TCIF."]
    #[inline(always)]
    pub const fn ctcif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 1usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "CTCIF, transfer complete flag clear. Write 1 to clear TCIF."]
    #[inline(always)]
    pub fn set_ctcif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 1usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "CHTIF, half transfer flag clear. Write 1 to clear HTIF."]
    #[inline(always)]
    pub const fn chtif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 2usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "CHTIF, half transfer flag clear. Write 1 to clear HTIF."]
    #[inline(always)]
    pub fn set_chtif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 2usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "CTEIF, transfer error flag clear. Write 1 to clear TEIF."]
    #[inline(always)]
    pub const fn cteif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "CTEIF, transfer error flag clear. Write 1 to clear TEIF."]
    #[inline(always)]
    pub fn set_cteif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("cgif[0]", &self.cgif(0usize))
            .field("cgif[1]", &self.cgif(1usize))
            .field("cgif[2]", &self.cgif(2usize))
            .field("cgif[3]", &self.cgif(3usize))
            .field("cgif[4]", &self.cgif(4usize))
            .field("cgif[5]", &self.cgif(5usize))
            .field("cgif[6]", &self.cgif(6usize))
            .field("cgif[7]", &self.cgif(7usize))
            .field("ctcif[0]", &self.ctcif(0usize))
            .field("ctcif[1]", &self.ctcif(1usize))
            .field("ctcif[2]", &self.ctcif(2usize))
            .field("ctcif[3]", &self.ctcif(3usize))
            .field("ctcif[4]", &self.ctcif(4usize))
            .field("ctcif[5]", &self.ctcif(5usize))
            .field("ctcif[6]", &self.ctcif(6usize))
            .field("ctcif[7]", &self.ctcif(7usize))
            .field("chtif[0]", &self.chtif(0usize))
            .field("chtif[1]", &self.chtif(1usize))
            .field("chtif[2]", &self.chtif(2usize))
            .field("chtif[3]", &self.chtif(3usize))
            .field("chtif[4]", &self.chtif(4usize))
            .field("chtif[5]", &self.chtif(5usize))
            .field("chtif[6]", &self.chtif(6usize))
            .field("chtif[7]", &self.chtif(7usize))
            .field("cteif[0]", &self.cteif(0usize))
            .field("cteif[1]", &self.cteif(1usize))
            .field("cteif[2]", &self.cteif(2usize))
            .field("cteif[3]", &self.cteif(3usize))
            .field("cteif[4]", &self.cteif(4usize))
            .field("cteif[5]", &self.cteif(5usize))
            .field("cteif[6]", &self.cteif(6usize))
            .field("cteif[7]", &self.cteif(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ifcr {{ cgif[0]: {=bool:?}, cgif[1]: {=bool:?}, cgif[2]: {=bool:?}, cgif[3]: {=bool:?}, cgif[4]: {=bool:?}, cgif[5]: {=bool:?}, cgif[6]: {=bool:?}, cgif[7]: {=bool:?}, ctcif[0]: {=bool:?}, ctcif[1]: {=bool:?}, ctcif[2]: {=bool:?}, ctcif[3]: {=bool:?}, ctcif[4]: {=bool:?}, ctcif[5]: {=bool:?}, ctcif[6]: {=bool:?}, ctcif[7]: {=bool:?}, chtif[0]: {=bool:?}, chtif[1]: {=bool:?}, chtif[2]: {=bool:?}, chtif[3]: {=bool:?}, chtif[4]: {=bool:?}, chtif[5]: {=bool:?}, chtif[6]: {=bool:?}, chtif[7]: {=bool:?}, cteif[0]: {=bool:?}, cteif[1]: {=bool:?}, cteif[2]: {=bool:?}, cteif[3]: {=bool:?}, cteif[4]: {=bool:?}, cteif[5]: {=bool:?}, cteif[6]: {=bool:?}, cteif[7]: {=bool:?} }}" , self . cgif (0usize) , self . cgif (1usize) , self . cgif (2usize) , self . cgif (3usize) , self . cgif (4usize) , self . cgif (5usize) , self . cgif (6usize) , self . cgif (7usize) , self . ctcif (0usize) , self . ctcif (1usize) , self . ctcif (2usize) , self . ctcif (3usize) , self . ctcif (4usize) , self . ctcif (5usize) , self . ctcif (6usize) , self . ctcif (7usize) , self . chtif (0usize) , self . chtif (1usize) , self . chtif (2usize) , self . chtif (3usize) , self . chtif (4usize) , self . chtif (5usize) , self . chtif (6usize) , self . chtif (7usize) , self . cteif (0usize) , self . cteif (1usize) , self . cteif (2usize) , self . cteif (3usize) , self . cteif (4usize) , self . cteif (5usize) , self . cteif (6usize) , self . cteif (7usize))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "channel global interrupt flag. Set when any of TEIF/HTIF/TCIF asserted. Cleared when TEIF/HTIF/TCIF all cleared."]
    #[inline(always)]
    pub const fn gif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "channel global interrupt flag. Set when any of TEIF/HTIF/TCIF asserted. Cleared when TEIF/HTIF/TCIF all cleared."]
    #[inline(always)]
    pub fn set_gif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "channel transfer complete flag. Set when all NDT are transferred. Cleared when write 1 to CTCIF or CGIF."]
    #[inline(always)]
    pub const fn tcif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 1usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "channel transfer complete flag. Set when all NDT are transferred. Cleared when write 1 to CTCIF or CGIF."]
    #[inline(always)]
    pub fn set_tcif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 1usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "channel half transfer flag. Set when half NDT are transferred. Cleared when write 1 to CHTIF or CGIF."]
    #[inline(always)]
    pub const fn htif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 2usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "channel half transfer flag. Set when half NDT are transferred. Cleared when write 1 to CHTIF or CGIF."]
    #[inline(always)]
    pub fn set_htif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 2usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "channel transfer error flag. Set when bus error detected. Cleared when write 1 to CTEIF or CGIF."]
    #[inline(always)]
    pub const fn teif(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "channel transfer error flag. Set when bus error detected. Cleared when write 1 to CTEIF or CGIF."]
    #[inline(always)]
    pub fn set_teif(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("gif[0]", &self.gif(0usize))
            .field("gif[1]", &self.gif(1usize))
            .field("gif[2]", &self.gif(2usize))
            .field("gif[3]", &self.gif(3usize))
            .field("gif[4]", &self.gif(4usize))
            .field("gif[5]", &self.gif(5usize))
            .field("gif[6]", &self.gif(6usize))
            .field("gif[7]", &self.gif(7usize))
            .field("tcif[0]", &self.tcif(0usize))
            .field("tcif[1]", &self.tcif(1usize))
            .field("tcif[2]", &self.tcif(2usize))
            .field("tcif[3]", &self.tcif(3usize))
            .field("tcif[4]", &self.tcif(4usize))
            .field("tcif[5]", &self.tcif(5usize))
            .field("tcif[6]", &self.tcif(6usize))
            .field("tcif[7]", &self.tcif(7usize))
            .field("htif[0]", &self.htif(0usize))
            .field("htif[1]", &self.htif(1usize))
            .field("htif[2]", &self.htif(2usize))
            .field("htif[3]", &self.htif(3usize))
            .field("htif[4]", &self.htif(4usize))
            .field("htif[5]", &self.htif(5usize))
            .field("htif[6]", &self.htif(6usize))
            .field("htif[7]", &self.htif(7usize))
            .field("teif[0]", &self.teif(0usize))
            .field("teif[1]", &self.teif(1usize))
            .field("teif[2]", &self.teif(2usize))
            .field("teif[3]", &self.teif(3usize))
            .field("teif[4]", &self.teif(4usize))
            .field("teif[5]", &self.teif(5usize))
            .field("teif[6]", &self.teif(6usize))
            .field("teif[7]", &self.teif(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Isr {{ gif[0]: {=bool:?}, gif[1]: {=bool:?}, gif[2]: {=bool:?}, gif[3]: {=bool:?}, gif[4]: {=bool:?}, gif[5]: {=bool:?}, gif[6]: {=bool:?}, gif[7]: {=bool:?}, tcif[0]: {=bool:?}, tcif[1]: {=bool:?}, tcif[2]: {=bool:?}, tcif[3]: {=bool:?}, tcif[4]: {=bool:?}, tcif[5]: {=bool:?}, tcif[6]: {=bool:?}, tcif[7]: {=bool:?}, htif[0]: {=bool:?}, htif[1]: {=bool:?}, htif[2]: {=bool:?}, htif[3]: {=bool:?}, htif[4]: {=bool:?}, htif[5]: {=bool:?}, htif[6]: {=bool:?}, htif[7]: {=bool:?}, teif[0]: {=bool:?}, teif[1]: {=bool:?}, teif[2]: {=bool:?}, teif[3]: {=bool:?}, teif[4]: {=bool:?}, teif[5]: {=bool:?}, teif[6]: {=bool:?}, teif[7]: {=bool:?} }}" , self . gif (0usize) , self . gif (1usize) , self . gif (2usize) , self . gif (3usize) , self . gif (4usize) , self . gif (5usize) , self . gif (6usize) , self . gif (7usize) , self . tcif (0usize) , self . tcif (1usize) , self . tcif (2usize) , self . tcif (3usize) , self . tcif (4usize) , self . tcif (5usize) , self . tcif (6usize) , self . tcif (7usize) , self . htif (0usize) , self . htif (1usize) , self . htif (2usize) , self . htif (3usize) , self . htif (4usize) , self . htif (5usize) , self . htif (6usize) , self . htif (7usize) , self . teif (0usize) , self . teif (1usize) , self . teif (2usize) , self . teif (3usize) , self . teif (4usize) , self . teif (5usize) , self . teif (6usize) , self . teif (7usize))
    }
}
