#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr(pub u32);
impl Cdr {
    #[doc = "Channel data payload"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel data payload"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        Cdr(0)
    }
}
impl core::fmt::Debug for Cdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cdr {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cer(pub u32);
impl Cer {
    #[doc = "Channel enable bitmask"]
    #[must_use]
    #[inline(always)]
    pub const fn ce(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel enable bitmask"]
    #[inline(always)]
    pub const fn set_ce(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cer {
    #[inline(always)]
    fn default() -> Cer {
        Cer(0)
    }
}
impl core::fmt::Debug for Cer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cer").field("ce", &self.ce()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cer {{ ce: {=u32:?} }}", self.ce())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub u32);
impl Ch {
    #[doc = "Breakpoint entry address (bits 18:2)"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Breakpoint entry address (bits 18:2)"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 2usize)) | (((val as u32) & 0x0001_ffff) << 2usize);
    }
}
impl Default for Ch {
    #[inline(always)]
    fn default() -> Ch {
        Ch(0)
    }
}
impl core::fmt::Debug for Ch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Channel status bitmask"]
    #[must_use]
    #[inline(always)]
    pub const fn cs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel status bitmask"]
    #[inline(always)]
    pub const fn set_cs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        f.debug_struct("Csr").field("cs", &self.cs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Csr {{ cs: {=u32:?} }}", self.cs())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Channel interrupt clear bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cic(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel interrupt clear bits"]
    #[inline(always)]
    pub const fn set_cic(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        f.debug_struct("Icr").field("cic", &self.cic()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Icr {{ cic: {=u32:?} }}", self.cic())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Channel interrupt enable mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cie(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel interrupt enable mask"]
    #[inline(always)]
    pub const fn set_cie(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        f.debug_struct("Ier").field("cie", &self.cie()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ier {{ cie: {=u32:?} }}", self.cie())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Channel interrupt status bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cis(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel interrupt status bits"]
    #[inline(always)]
    pub const fn set_cis(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        f.debug_struct("Isr").field("cis", &self.cis()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr {{ cis: {=u32:?} }}", self.cis())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ver(pub u32);
impl Ver {
    #[doc = "Module version identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Module version identifier"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ver {
    #[inline(always)]
    fn default() -> Ver {
        Ver(0)
    }
}
impl core::fmt::Debug for Ver {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ver").field("id", &self.id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ver {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ver {{ id: {=u32:?} }}", self.id())
    }
}
