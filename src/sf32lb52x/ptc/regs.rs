#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio310(pub u32);
impl Gpio310 {
    #[doc = "select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31"]
    #[must_use]
    #[inline(always)]
    pub const fn sela(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31"]
    #[inline(always)]
    pub const fn set_sela(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "select trigger B of GPIO 31~0"]
    #[must_use]
    #[inline(always)]
    pub const fn selb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger B of GPIO 31~0"]
    #[inline(always)]
    pub const fn set_selb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "select trigger C of GPIO 31~0"]
    #[must_use]
    #[inline(always)]
    pub const fn selc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger C of GPIO 31~0"]
    #[inline(always)]
    pub const fn set_selc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "select trigger D of GPIO 31~0"]
    #[must_use]
    #[inline(always)]
    pub const fn seld(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger D of GPIO 31~0"]
    #[inline(always)]
    pub const fn set_seld(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Gpio310 {
    #[inline(always)]
    fn default() -> Gpio310 {
        Gpio310(0)
    }
}
impl core::fmt::Debug for Gpio310 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpio310")
            .field("sela", &self.sela())
            .field("selb", &self.selb())
            .field("selc", &self.selc())
            .field("seld", &self.seld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpio310 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpio310 {{ sela: {=u8:?}, selb: {=u8:?}, selc: {=u8:?}, seld: {=u8:?} }}",
            self.sela(),
            self.selb(),
            self.selc(),
            self.seld()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio6332(pub u32);
impl Gpio6332 {
    #[doc = "select trigger A of GPIO 63~32 0: select GPIO 32 1: select GPIO 33 ...... 31: select GPIO 63"]
    #[must_use]
    #[inline(always)]
    pub const fn sela(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger A of GPIO 63~32 0: select GPIO 32 1: select GPIO 33 ...... 31: select GPIO 63"]
    #[inline(always)]
    pub const fn set_sela(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "select trigger B of GPIO 63~32"]
    #[must_use]
    #[inline(always)]
    pub const fn selb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger B of GPIO 63~32"]
    #[inline(always)]
    pub const fn set_selb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "select trigger C of GPIO 63~32"]
    #[must_use]
    #[inline(always)]
    pub const fn selc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger C of GPIO 63~32"]
    #[inline(always)]
    pub const fn set_selc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "select trigger D of GPIO 63~32"]
    #[must_use]
    #[inline(always)]
    pub const fn seld(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger D of GPIO 63~32"]
    #[inline(always)]
    pub const fn set_seld(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Gpio6332 {
    #[inline(always)]
    fn default() -> Gpio6332 {
        Gpio6332(0)
    }
}
impl core::fmt::Debug for Gpio6332 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpio6332")
            .field("sela", &self.sela())
            .field("selb", &self.selb())
            .field("selc", &self.selc())
            .field("seld", &self.seld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpio6332 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpio6332 {{ sela: {=u8:?}, selb: {=u8:?}, selc: {=u8:?}, seld: {=u8:?} }}",
            self.sela(),
            self.selb(),
            self.selc(),
            self.seld()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio9564(pub u32);
impl Gpio9564 {
    #[doc = "select trigger A of GPIO 95~64 0: select GPIO 64 1: select GPIO 65 ...... 31: select GPIO 95"]
    #[must_use]
    #[inline(always)]
    pub const fn sela(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger A of GPIO 95~64 0: select GPIO 64 1: select GPIO 65 ...... 31: select GPIO 95"]
    #[inline(always)]
    pub const fn set_sela(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "select trigger B of GPIO 95~64"]
    #[must_use]
    #[inline(always)]
    pub const fn selb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger B of GPIO 95~64"]
    #[inline(always)]
    pub const fn set_selb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "select trigger C of GPIO 95~64"]
    #[must_use]
    #[inline(always)]
    pub const fn selc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger C of GPIO 95~64"]
    #[inline(always)]
    pub const fn set_selc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "select trigger D of GPIO 95~64"]
    #[must_use]
    #[inline(always)]
    pub const fn seld(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "select trigger D of GPIO 95~64"]
    #[inline(always)]
    pub const fn set_seld(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Gpio9564 {
    #[inline(always)]
    fn default() -> Gpio9564 {
        Gpio9564(0)
    }
}
impl core::fmt::Debug for Gpio9564 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpio9564")
            .field("sela", &self.sela())
            .field("selb", &self.selb())
            .field("selc", &self.selc())
            .field("seld", &self.seld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpio9564 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpio9564 {{ sela: {=u8:?}, selb: {=u8:?}, selc: {=u8:?}, seld: {=u8:?} }}",
            self.sela(),
            self.selb(),
            self.selc(),
            self.seld()
        )
    }
}
#[doc = "interrupt clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "clear task complete interrupt flag for task 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "clear task complete interrupt flag for task 1"]
    #[inline(always)]
    pub const fn set_ctcif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "clear task complete interrupt flag for task 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "clear task complete interrupt flag for task 2"]
    #[inline(always)]
    pub const fn set_ctcif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "clear task complete interrupt flag for task 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "clear task complete interrupt flag for task 3"]
    #[inline(always)]
    pub const fn set_ctcif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "clear task complete interrupt flag for task 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "clear task complete interrupt flag for task 4"]
    #[inline(always)]
    pub const fn set_ctcif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "clear task complete interrupt flag for task 5"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif5(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "clear task complete interrupt flag for task 5"]
    #[inline(always)]
    pub const fn set_ctcif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "clear task complete interrupt flag for task 6"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif6(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "clear task complete interrupt flag for task 6"]
    #[inline(always)]
    pub const fn set_ctcif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "clear task complete interrupt flag for task 7"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif7(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "clear task complete interrupt flag for task 7"]
    #[inline(always)]
    pub const fn set_ctcif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "clear task complete interrupt flag for task 8"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif8(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "clear task complete interrupt flag for task 8"]
    #[inline(always)]
    pub const fn set_ctcif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "clear transfer error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cteif(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "clear transfer error flag"]
    #[inline(always)]
    pub const fn set_cteif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("ctcif1", &self.ctcif1())
            .field("ctcif2", &self.ctcif2())
            .field("ctcif3", &self.ctcif3())
            .field("ctcif4", &self.ctcif4())
            .field("ctcif5", &self.ctcif5())
            .field("ctcif6", &self.ctcif6())
            .field("ctcif7", &self.ctcif7())
            .field("ctcif8", &self.ctcif8())
            .field("cteif", &self.cteif())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Icr {{ ctcif1: {=bool:?}, ctcif2: {=bool:?}, ctcif3: {=bool:?}, ctcif4: {=bool:?}, ctcif5: {=bool:?}, ctcif6: {=bool:?}, ctcif7: {=bool:?}, ctcif8: {=bool:?}, cteif: {=bool:?} }}" , self . ctcif1 () , self . ctcif2 () , self . ctcif3 () , self . ctcif4 () , self . ctcif5 () , self . ctcif6 () , self . ctcif7 () , self . ctcif8 () , self . cteif ())
    }
}
#[doc = "interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "enable task complete interrupt for task 1"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable task complete interrupt for task 1"]
    #[inline(always)]
    pub const fn set_tcie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "enable task complete interrupt for task 2"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "enable task complete interrupt for task 2"]
    #[inline(always)]
    pub const fn set_tcie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "enable task complete interrupt for task 3"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "enable task complete interrupt for task 3"]
    #[inline(always)]
    pub const fn set_tcie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "enable task complete interrupt for task 4"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "enable task complete interrupt for task 4"]
    #[inline(always)]
    pub const fn set_tcie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "enable task complete interrupt for task 5"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie5(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "enable task complete interrupt for task 5"]
    #[inline(always)]
    pub const fn set_tcie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "enable task complete interrupt for task 6"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie6(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "enable task complete interrupt for task 6"]
    #[inline(always)]
    pub const fn set_tcie6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "enable task complete interrupt for task 7"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie7(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "enable task complete interrupt for task 7"]
    #[inline(always)]
    pub const fn set_tcie7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "enable task complete interrupt for task 8"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie8(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "enable task complete interrupt for task 8"]
    #[inline(always)]
    pub const fn set_tcie8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "enable transfer error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "enable transfer error flag"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("tcie1", &self.tcie1())
            .field("tcie2", &self.tcie2())
            .field("tcie3", &self.tcie3())
            .field("tcie4", &self.tcie4())
            .field("tcie5", &self.tcie5())
            .field("tcie6", &self.tcie6())
            .field("tcie7", &self.tcie7())
            .field("tcie8", &self.tcie8())
            .field("teie", &self.teie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ier {{ tcie1: {=bool:?}, tcie2: {=bool:?}, tcie3: {=bool:?}, tcie4: {=bool:?}, tcie5: {=bool:?}, tcie6: {=bool:?}, tcie7: {=bool:?}, tcie8: {=bool:?}, teie: {=bool:?} }}" , self . tcie1 () , self . tcie2 () , self . tcie3 () , self . tcie4 () , self . tcie5 () , self . tcie6 () , self . tcie7 () , self . tcie8 () , self . teie ())
    }
}
#[doc = "interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "task complete interrupt flag for task 1"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "task complete interrupt flag for task 1"]
    #[inline(always)]
    pub const fn set_tcif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "task complete interrupt flag for task 2"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "task complete interrupt flag for task 2"]
    #[inline(always)]
    pub const fn set_tcif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "task complete interrupt flag for task 3"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "task complete interrupt flag for task 3"]
    #[inline(always)]
    pub const fn set_tcif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "task complete interrupt flag for task 4"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "task complete interrupt flag for task 4"]
    #[inline(always)]
    pub const fn set_tcif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "task complete interrupt flag for task 5"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif5(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "task complete interrupt flag for task 5"]
    #[inline(always)]
    pub const fn set_tcif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "task complete interrupt flag for task 6"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif6(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "task complete interrupt flag for task 6"]
    #[inline(always)]
    pub const fn set_tcif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "task complete interrupt flag for task 7"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif7(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "task complete interrupt flag for task 7"]
    #[inline(always)]
    pub const fn set_tcif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "task complete interrupt flag for task 8"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif8(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "task complete interrupt flag for task 8"]
    #[inline(always)]
    pub const fn set_tcif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "transfer error flag for task 1"]
    #[must_use]
    #[inline(always)]
    pub const fn teif1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error flag for task 1"]
    #[inline(always)]
    pub const fn set_teif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "transfer error flag for task 2"]
    #[must_use]
    #[inline(always)]
    pub const fn teif2(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error flag for task 2"]
    #[inline(always)]
    pub const fn set_teif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "transfer error flag for task 3"]
    #[must_use]
    #[inline(always)]
    pub const fn teif3(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error flag for task 3"]
    #[inline(always)]
    pub const fn set_teif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "transfer error flag for task 4"]
    #[must_use]
    #[inline(always)]
    pub const fn teif4(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error flag for task 4"]
    #[inline(always)]
    pub const fn set_teif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "transfer error flag for task 5"]
    #[must_use]
    #[inline(always)]
    pub const fn teif5(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error flag for task 5"]
    #[inline(always)]
    pub const fn set_teif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "transfer error flag for task 6"]
    #[must_use]
    #[inline(always)]
    pub const fn teif6(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error flag for task 6"]
    #[inline(always)]
    pub const fn set_teif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "transfer error flag for task 7"]
    #[must_use]
    #[inline(always)]
    pub const fn teif7(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error flag for task 7"]
    #[inline(always)]
    pub const fn set_teif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "transfer error flag for task 8"]
    #[must_use]
    #[inline(always)]
    pub const fn teif8(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "transfer error flag for task 8"]
    #[inline(always)]
    pub const fn set_teif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
            .field("tcif1", &self.tcif1())
            .field("tcif2", &self.tcif2())
            .field("tcif3", &self.tcif3())
            .field("tcif4", &self.tcif4())
            .field("tcif5", &self.tcif5())
            .field("tcif6", &self.tcif6())
            .field("tcif7", &self.tcif7())
            .field("tcif8", &self.tcif8())
            .field("teif1", &self.teif1())
            .field("teif2", &self.teif2())
            .field("teif3", &self.teif3())
            .field("teif4", &self.teif4())
            .field("teif5", &self.teif5())
            .field("teif6", &self.teif6())
            .field("teif7", &self.teif7())
            .field("teif8", &self.teif8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Isr {{ tcif1: {=bool:?}, tcif2: {=bool:?}, tcif3: {=bool:?}, tcif4: {=bool:?}, tcif5: {=bool:?}, tcif6: {=bool:?}, tcif7: {=bool:?}, tcif8: {=bool:?}, teif1: {=bool:?}, teif2: {=bool:?}, teif3: {=bool:?}, teif4: {=bool:?}, teif5: {=bool:?}, teif6: {=bool:?}, teif7: {=bool:?}, teif8: {=bool:?} }}" , self . tcif1 () , self . tcif2 () , self . tcif3 () , self . tcif4 () , self . tcif5 () , self . tcif6 () , self . tcif7 () , self . tcif8 () , self . teif1 () , self . teif2 () , self . teif3 () , self . teif4 () , self . teif5 () , self . teif6 () , self . teif7 () , self . teif8 ())
    }
}
#[doc = "temporary memory 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem1(pub u32);
impl Mem1 {
    #[doc = "memory to store temporary variables"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "memory to store temporary variables"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mem1 {
    #[inline(always)]
    fn default() -> Mem1 {
        Mem1(0)
    }
}
impl core::fmt::Debug for Mem1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem1").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "temporary memory 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem2(pub u32);
impl Mem2 {
    #[doc = "memory to store temporary variables"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "memory to store temporary variables"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mem2 {
    #[inline(always)]
    fn default() -> Mem2 {
        Mem2(0)
    }
}
impl core::fmt::Debug for Mem2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem2").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem2 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "temporary memory 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem3(pub u32);
impl Mem3 {
    #[doc = "memory to store temporary variables"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "memory to store temporary variables"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mem3 {
    #[inline(always)]
    fn default() -> Mem3 {
        Mem3(0)
    }
}
impl core::fmt::Debug for Mem3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem3").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem3 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "temporary memory 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem4(pub u32);
impl Mem4 {
    #[doc = "memory to store temporary variables"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "memory to store temporary variables"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mem4 {
    #[inline(always)]
    fn default() -> Mem4 {
        Mem4(0)
    }
}
impl core::fmt::Debug for Mem4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mem4").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mem4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mem4 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "task 1 repetition and delay counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr1(pub u32);
impl Rcr1 {
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation."]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        Rcr1(0)
    }
}
impl core::fmt::Debug for Rcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr1")
            .field("rep", &self.rep())
            .field("dly", &self.dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr1 {{ rep: {=u16:?}, dly: {=u16:?} }}",
            self.rep(),
            self.dly()
        )
    }
}
#[doc = "task 2 repetition and delay counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2(pub u32);
impl Rcr2 {
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation."]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        Rcr2(0)
    }
}
impl core::fmt::Debug for Rcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr2")
            .field("rep", &self.rep())
            .field("dly", &self.dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr2 {{ rep: {=u16:?}, dly: {=u16:?} }}",
            self.rep(),
            self.dly()
        )
    }
}
#[doc = "task 3 repetition and delay counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr3(pub u32);
impl Rcr3 {
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation."]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Rcr3 {
    #[inline(always)]
    fn default() -> Rcr3 {
        Rcr3(0)
    }
}
impl core::fmt::Debug for Rcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr3")
            .field("rep", &self.rep())
            .field("dly", &self.dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr3 {{ rep: {=u16:?}, dly: {=u16:?} }}",
            self.rep(),
            self.dly()
        )
    }
}
#[doc = "task 4 repetition and delay counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr4(pub u32);
impl Rcr4 {
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Delay time before task operation after triggered 0: no delay others: delay DLY HCLK cycles before task operation DLY is read as left delay time. DLY will be reloaded automatically after each operation."]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Rcr4 {
    #[inline(always)]
    fn default() -> Rcr4 {
        Rcr4(0)
    }
}
impl core::fmt::Debug for Rcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr4")
            .field("rep", &self.rep())
            .field("dly", &self.dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr4 {{ rep: {=u16:?}, dly: {=u16:?} }}",
            self.rep(),
            self.dly()
        )
    }
}
#[doc = "task 5 repetition counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr5(pub u32);
impl Rcr5 {
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Rcr5 {
    #[inline(always)]
    fn default() -> Rcr5 {
        Rcr5(0)
    }
}
impl core::fmt::Debug for Rcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr5").field("rep", &self.rep()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rcr5 {{ rep: {=u16:?} }}", self.rep())
    }
}
#[doc = "task 6 repetition counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr6(pub u32);
impl Rcr6 {
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Rcr6 {
    #[inline(always)]
    fn default() -> Rcr6 {
        Rcr6(0)
    }
}
impl core::fmt::Debug for Rcr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr6").field("rep", &self.rep()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rcr6 {{ rep: {=u16:?} }}", self.rep())
    }
}
#[doc = "task 7 repetition counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr7(pub u32);
impl Rcr7 {
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Rcr7 {
    #[inline(always)]
    fn default() -> Rcr7 {
        Rcr7(0)
    }
}
impl core::fmt::Debug for Rcr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr7").field("rep", &self.rep()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rcr7 {{ rep: {=u16:?} }}", self.rep())
    }
}
#[doc = "task 8 repetition counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr8(pub u32);
impl Rcr8 {
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Repetition counter value if REPEN is 1, task will only be triggerd when REP is not 0. when REP is larger than 0, it will be decrease by 1 automatically each time task triggered."]
    #[inline(always)]
    pub const fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Rcr8 {
    #[inline(always)]
    fn default() -> Rcr8 {
        Rcr8(0)
    }
}
impl core::fmt::Debug for Rcr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr8").field("rep", &self.rep()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rcr8 {{ rep: {=u16:?} }}", self.rep())
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
#[doc = "task 1 address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar1(pub u32);
impl Tar1 {
    #[doc = "peripheral address to access to"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address to access to"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar1 {
    #[inline(always)]
    fn default() -> Tar1 {
        Tar1(0)
    }
}
impl core::fmt::Debug for Tar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar1").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar1 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar2(pub u32);
impl Tar2 {
    #[doc = "peripheral address to access to"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address to access to"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar2 {
    #[inline(always)]
    fn default() -> Tar2 {
        Tar2(0)
    }
}
impl core::fmt::Debug for Tar2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar2").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar2 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar3(pub u32);
impl Tar3 {
    #[doc = "peripheral address to access to"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address to access to"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar3 {
    #[inline(always)]
    fn default() -> Tar3 {
        Tar3(0)
    }
}
impl core::fmt::Debug for Tar3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar3").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar3 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar4(pub u32);
impl Tar4 {
    #[doc = "peripheral address to access to"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address to access to"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar4 {
    #[inline(always)]
    fn default() -> Tar4 {
        Tar4(0)
    }
}
impl core::fmt::Debug for Tar4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar4").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar4 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar5(pub u32);
impl Tar5 {
    #[doc = "peripheral address to access to"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address to access to"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar5 {
    #[inline(always)]
    fn default() -> Tar5 {
        Tar5(0)
    }
}
impl core::fmt::Debug for Tar5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar5").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar5 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar6(pub u32);
impl Tar6 {
    #[doc = "peripheral address to access to"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address to access to"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar6 {
    #[inline(always)]
    fn default() -> Tar6 {
        Tar6(0)
    }
}
impl core::fmt::Debug for Tar6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar6").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar6 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar7(pub u32);
impl Tar7 {
    #[doc = "peripheral address to access to"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address to access to"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar7 {
    #[inline(always)]
    fn default() -> Tar7 {
        Tar7(0)
    }
}
impl core::fmt::Debug for Tar7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar7").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar7 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar8(pub u32);
impl Tar8 {
    #[doc = "peripheral address to access to"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "peripheral address to access to"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar8 {
    #[inline(always)]
    fn default() -> Tar8 {
        Tar8(0)
    }
}
impl core::fmt::Debug for Tar8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar8").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar8 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "task 1 control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr1(pub u32);
impl Tcr1 {
    #[doc = "select trigger source 0: task will only be triggered by SWTRIG others: task will be triggered by selected source or SWTRIG"]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "select trigger source 0: task will only be triggered by SWTRIG others: task will be triggered by selected source or SWTRIG"]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn reptrig(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_reptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn repirq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_repirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Tcr1 {
    #[inline(always)]
    fn default() -> Tcr1 {
        Tcr1(0)
    }
}
impl core::fmt::Debug for Tcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr1")
            .field("trigsel", &self.trigsel())
            .field("op", &self.op())
            .field("trigpol", &self.trigpol())
            .field("swtrig", &self.swtrig())
            .field("repen", &self.repen())
            .field("reptrig", &self.reptrig())
            .field("repirq", &self.repirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr1 {{ trigsel: {=u8:?}, op: {=u8:?}, trigpol: {=bool:?}, swtrig: {=bool:?}, repen: {=bool:?}, reptrig: {=bool:?}, repirq: {=bool:?} }}" , self . trigsel () , self . op () , self . trigpol () , self . swtrig () , self . repen () , self . reptrig () , self . repirq ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr2(pub u32);
impl Tcr2 {
    #[doc = "select trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "select trigger source"]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn reptrig(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_reptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn repirq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_repirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Tcr2 {
    #[inline(always)]
    fn default() -> Tcr2 {
        Tcr2(0)
    }
}
impl core::fmt::Debug for Tcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr2")
            .field("trigsel", &self.trigsel())
            .field("op", &self.op())
            .field("trigpol", &self.trigpol())
            .field("swtrig", &self.swtrig())
            .field("repen", &self.repen())
            .field("reptrig", &self.reptrig())
            .field("repirq", &self.repirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr2 {{ trigsel: {=u8:?}, op: {=u8:?}, trigpol: {=bool:?}, swtrig: {=bool:?}, repen: {=bool:?}, reptrig: {=bool:?}, repirq: {=bool:?} }}" , self . trigsel () , self . op () , self . trigpol () , self . swtrig () , self . repen () , self . reptrig () , self . repirq ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr3(pub u32);
impl Tcr3 {
    #[doc = "select trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "select trigger source"]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn reptrig(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_reptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn repirq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_repirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Tcr3 {
    #[inline(always)]
    fn default() -> Tcr3 {
        Tcr3(0)
    }
}
impl core::fmt::Debug for Tcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr3")
            .field("trigsel", &self.trigsel())
            .field("op", &self.op())
            .field("trigpol", &self.trigpol())
            .field("swtrig", &self.swtrig())
            .field("repen", &self.repen())
            .field("reptrig", &self.reptrig())
            .field("repirq", &self.repirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr3 {{ trigsel: {=u8:?}, op: {=u8:?}, trigpol: {=bool:?}, swtrig: {=bool:?}, repen: {=bool:?}, reptrig: {=bool:?}, repirq: {=bool:?} }}" , self . trigsel () , self . op () , self . trigpol () , self . swtrig () , self . repen () , self . reptrig () , self . repirq ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr4(pub u32);
impl Tcr4 {
    #[doc = "select trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "select trigger source"]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn reptrig(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_reptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn repirq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_repirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Tcr4 {
    #[inline(always)]
    fn default() -> Tcr4 {
        Tcr4(0)
    }
}
impl core::fmt::Debug for Tcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr4")
            .field("trigsel", &self.trigsel())
            .field("op", &self.op())
            .field("trigpol", &self.trigpol())
            .field("swtrig", &self.swtrig())
            .field("repen", &self.repen())
            .field("reptrig", &self.reptrig())
            .field("repirq", &self.repirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr4 {{ trigsel: {=u8:?}, op: {=u8:?}, trigpol: {=bool:?}, swtrig: {=bool:?}, repen: {=bool:?}, reptrig: {=bool:?}, repirq: {=bool:?} }}" , self . trigsel () , self . op () , self . trigpol () , self . swtrig () , self . repen () , self . reptrig () , self . repirq ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr5(pub u32);
impl Tcr5 {
    #[doc = "select trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "select trigger source"]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn reptrig(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_reptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn repirq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_repirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Tcr5 {
    #[inline(always)]
    fn default() -> Tcr5 {
        Tcr5(0)
    }
}
impl core::fmt::Debug for Tcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr5")
            .field("trigsel", &self.trigsel())
            .field("op", &self.op())
            .field("trigpol", &self.trigpol())
            .field("swtrig", &self.swtrig())
            .field("repen", &self.repen())
            .field("reptrig", &self.reptrig())
            .field("repirq", &self.repirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr5 {{ trigsel: {=u8:?}, op: {=u8:?}, trigpol: {=bool:?}, swtrig: {=bool:?}, repen: {=bool:?}, reptrig: {=bool:?}, repirq: {=bool:?} }}" , self . trigsel () , self . op () , self . trigpol () , self . swtrig () , self . repen () , self . reptrig () , self . repirq ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr6(pub u32);
impl Tcr6 {
    #[doc = "select trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "select trigger source"]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn reptrig(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_reptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn repirq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_repirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Tcr6 {
    #[inline(always)]
    fn default() -> Tcr6 {
        Tcr6(0)
    }
}
impl core::fmt::Debug for Tcr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr6")
            .field("trigsel", &self.trigsel())
            .field("op", &self.op())
            .field("trigpol", &self.trigpol())
            .field("swtrig", &self.swtrig())
            .field("repen", &self.repen())
            .field("reptrig", &self.reptrig())
            .field("repirq", &self.repirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr6 {{ trigsel: {=u8:?}, op: {=u8:?}, trigpol: {=bool:?}, swtrig: {=bool:?}, repen: {=bool:?}, reptrig: {=bool:?}, repirq: {=bool:?} }}" , self . trigsel () , self . op () , self . trigpol () , self . swtrig () , self . repen () , self . reptrig () , self . repirq ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr7(pub u32);
impl Tcr7 {
    #[doc = "select trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "select trigger source"]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn reptrig(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_reptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn repirq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_repirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Tcr7 {
    #[inline(always)]
    fn default() -> Tcr7 {
        Tcr7(0)
    }
}
impl core::fmt::Debug for Tcr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr7")
            .field("trigsel", &self.trigsel())
            .field("op", &self.op())
            .field("trigpol", &self.trigpol())
            .field("swtrig", &self.swtrig())
            .field("repen", &self.repen())
            .field("reptrig", &self.reptrig())
            .field("repirq", &self.repirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr7 {{ trigsel: {=u8:?}, op: {=u8:?}, trigpol: {=bool:?}, swtrig: {=bool:?}, repen: {=bool:?}, reptrig: {=bool:?}, repirq: {=bool:?} }}" , self . trigsel () , self . op () , self . trigpol () , self . swtrig () , self . repen () , self . reptrig () , self . repirq ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr8(pub u32);
impl Tcr8 {
    #[doc = "select trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "select trigger source"]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "task operation 3'b000: direct write data 3'b100: read then XOR with data and write back 3'b101: read then OR with data and write back 3'b110: read then AND with data and write back 3'b111: read then add with data and write back"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "trigger polarity 0: select positive edge of trigger 1: select negative edge of trigger"]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrig(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "software trigger task will be triggerd at once after SWTRIG set. SWTRIG will be cleared automatically."]
    #[inline(always)]
    pub const fn set_swtrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "repetition enable 0: task will be triggerd no matter what value REP is 1: task will only be triggerd when REP is not 0"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn reptrig(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "repetition trigger 0: ptc trigger will be generated after each operation 1: ptc trigger will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_reptrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[must_use]
    #[inline(always)]
    pub const fn repirq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "repetition interrupt 0: interrupt will be generated after each operation 1: interrupt will be generated after operation for REP times"]
    #[inline(always)]
    pub const fn set_repirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Tcr8 {
    #[inline(always)]
    fn default() -> Tcr8 {
        Tcr8(0)
    }
}
impl core::fmt::Debug for Tcr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr8")
            .field("trigsel", &self.trigsel())
            .field("op", &self.op())
            .field("trigpol", &self.trigpol())
            .field("swtrig", &self.swtrig())
            .field("repen", &self.repen())
            .field("reptrig", &self.reptrig())
            .field("repirq", &self.repirq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Tcr8 {{ trigsel: {=u8:?}, op: {=u8:?}, trigpol: {=bool:?}, swtrig: {=bool:?}, repen: {=bool:?}, reptrig: {=bool:?}, repirq: {=bool:?} }}" , self . trigsel () , self . op () , self . trigpol () , self . swtrig () , self . repen () , self . reptrig () , self . repirq ())
    }
}
#[doc = "task 1 data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr1(pub u32);
impl Tdr1 {
    #[doc = "data value for task operation"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "data value for task operation"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr1 {
    #[inline(always)]
    fn default() -> Tdr1 {
        Tdr1(0)
    }
}
impl core::fmt::Debug for Tdr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr1").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr1 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr2(pub u32);
impl Tdr2 {
    #[doc = "data value for task operation"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "data value for task operation"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr2 {
    #[inline(always)]
    fn default() -> Tdr2 {
        Tdr2(0)
    }
}
impl core::fmt::Debug for Tdr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr2").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr2 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr3(pub u32);
impl Tdr3 {
    #[doc = "data value for task operation"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "data value for task operation"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr3 {
    #[inline(always)]
    fn default() -> Tdr3 {
        Tdr3(0)
    }
}
impl core::fmt::Debug for Tdr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr3").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr3 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr4(pub u32);
impl Tdr4 {
    #[doc = "data value for task operation"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "data value for task operation"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr4 {
    #[inline(always)]
    fn default() -> Tdr4 {
        Tdr4(0)
    }
}
impl core::fmt::Debug for Tdr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr4").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr4 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr5(pub u32);
impl Tdr5 {
    #[doc = "data value for task operation"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "data value for task operation"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr5 {
    #[inline(always)]
    fn default() -> Tdr5 {
        Tdr5(0)
    }
}
impl core::fmt::Debug for Tdr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr5").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr5 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr6(pub u32);
impl Tdr6 {
    #[doc = "data value for task operation"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "data value for task operation"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr6 {
    #[inline(always)]
    fn default() -> Tdr6 {
        Tdr6(0)
    }
}
impl core::fmt::Debug for Tdr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr6").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr6 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr7(pub u32);
impl Tdr7 {
    #[doc = "data value for task operation"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "data value for task operation"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr7 {
    #[inline(always)]
    fn default() -> Tdr7 {
        Tdr7(0)
    }
}
impl core::fmt::Debug for Tdr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr7").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr7 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr8(pub u32);
impl Tdr8 {
    #[doc = "data value for task operation"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "data value for task operation"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr8 {
    #[inline(always)]
    fn default() -> Tdr8 {
        Tdr8(0)
    }
}
impl core::fmt::Debug for Tdr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr8").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr8 {{ data: {=u32:?} }}", self.data())
    }
}
