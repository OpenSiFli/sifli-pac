#[doc = "Patch channel register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub u32);
impl Ch {
    const ADDR_MASK: u32 = 0x1ffff << 2;
    #[doc = "Breakpoint entry address (bits 18:2)"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        (self.0 & Self::ADDR_MASK) >> 2
    }
    #[doc = "Set breakpoint entry address (bits 18:2)"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !Self::ADDR_MASK) | ((val & 0x1ffff) << 2);
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
        f.debug_struct("Ch")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch {{ addr: {=u32:?} }}", self.addr());
    }
}
#[doc = "Patch channel enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cer(pub u32);
impl Cer {
    #[doc = "Channel enable bitmask"]
    #[must_use]
    #[inline(always)]
    pub const fn ce(&self) -> u32 {
        self.0
    }
    #[doc = "Set channel enable bitmask"]
    #[inline(always)]
    pub fn set_ce(&mut self, val: u32) {
        self.0 = val;
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
        f.debug_struct("Cer")
            .field("ce", &format_args!("0x{:08x}", self.ce()))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cer {{ ce: {=u32:#010x} }}", self.ce());
    }
}

#[doc = "Patch channel status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Channel status bitmask"]
    #[must_use]
    #[inline(always)]
    pub const fn cs(&self) -> u32 {
        self.0
    }
    #[doc = "Set channel status bitmask"]
    #[inline(always)]
    pub fn set_cs(&mut self, val: u32) {
        self.0 = val;
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
            .field("cs", &format_args!("0x{:08x}", self.cs()))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Csr {{ cs: {=u32:#010x} }}", self.cs());
    }
}

#[doc = "Patch channel data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr(pub u32);
impl Cdr {
    #[doc = "Channel data payload"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        self.0
    }
    #[doc = "Set channel data payload"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = val;
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
        f.debug_struct("Cdr")
            .field("data", &format_args!("0x{:08x}", self.data()))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cdr {{ data: {=u32:#010x} }}", self.data());
    }
}

#[doc = "Patch interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Channel interrupt enable mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cie(&self) -> u32 {
        self.0
    }
    #[doc = "Set channel interrupt enable mask"]
    #[inline(always)]
    pub fn set_cie(&mut self, val: u32) {
        self.0 = val;
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
            .field("cie", &format_args!("0x{:08x}", self.cie()))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ier {{ cie: {=u32:#010x} }}", self.cie());
    }
}

#[doc = "Patch interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Channel interrupt status bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cis(&self) -> u32 {
        self.0
    }
    #[doc = "Set channel interrupt status bits"]
    #[inline(always)]
    pub fn set_cis(&mut self, val: u32) {
        self.0 = val;
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
            .field("cis", &format_args!("0x{:08x}", self.cis()))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr {{ cis: {=u32:#010x} }}", self.cis());
    }
}

#[doc = "Patch interrupt clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Channel interrupt clear bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cic(&self) -> u32 {
        self.0
    }
    #[doc = "Set channel interrupt clear bits"]
    #[inline(always)]
    pub fn set_cic(&mut self, val: u32) {
        self.0 = val;
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
            .field("cic", &format_args!("0x{:08x}", self.cic()))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Icr {{ cic: {=u32:#010x} }}", self.cic());
    }
}

#[doc = "Patch version register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ver(pub u32);
impl Ver {
    #[doc = "Module version identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        self.0
    }
    #[doc = "Set module version identifier"]
    #[inline(always)]
    pub fn set_id(&mut self, val: u32) {
        self.0 = val;
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
        f.debug_struct("Ver")
            .field("id", &format_args!("0x{:08x}", self.id()))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ver {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ver {{ id: {=u32:#010x} }}", self.id());
    }
}
