#[doc = "Data Input Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir0(pub u32);
impl Dir0 {}
impl Default for Dir0 {
    #[inline(always)]
    fn default() -> Dir0 {
        Dir0(0)
    }
}
impl core::fmt::Debug for Dir0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dir0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dir0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dir0 {{ }}",)
    }
}
#[doc = "Data Input Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir1(pub u32);
impl Dir1 {}
impl Default for Dir1 {
    #[inline(always)]
    fn default() -> Dir1 {
        Dir1(0)
    }
}
impl core::fmt::Debug for Dir1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dir1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dir1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dir1 {{ }}",)
    }
}
#[doc = "Data Output Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Docr0(pub u32);
impl Docr0 {}
impl Default for Docr0 {
    #[inline(always)]
    fn default() -> Docr0 {
        Docr0(0)
    }
}
impl core::fmt::Debug for Docr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Docr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Docr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Docr0 {{ }}",)
    }
}
#[doc = "Data Output Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Docr1(pub u32);
impl Docr1 {}
impl Default for Docr1 {
    #[inline(always)]
    fn default() -> Docr1 {
        Docr1(0)
    }
}
impl core::fmt::Debug for Docr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Docr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Docr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Docr1 {{ }}",)
    }
}
#[doc = "Data Output Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doecr0(pub u32);
impl Doecr0 {}
impl Default for Doecr0 {
    #[inline(always)]
    fn default() -> Doecr0 {
        Doecr0(0)
    }
}
impl core::fmt::Debug for Doecr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doecr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doecr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Doecr0 {{ }}",)
    }
}
#[doc = "Data Output Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doecr1(pub u32);
impl Doecr1 {}
impl Default for Doecr1 {
    #[inline(always)]
    fn default() -> Doecr1 {
        Doecr1(0)
    }
}
impl core::fmt::Debug for Doecr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doecr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doecr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Doecr1 {{ }}",)
    }
}
#[doc = "Data Output Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doer0(pub u32);
impl Doer0 {}
impl Default for Doer0 {
    #[inline(always)]
    fn default() -> Doer0 {
        Doer0(0)
    }
}
impl core::fmt::Debug for Doer0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doer0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doer0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Doer0 {{ }}",)
    }
}
#[doc = "Data Output Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doer1(pub u32);
impl Doer1 {}
impl Default for Doer1 {
    #[inline(always)]
    fn default() -> Doer1 {
        Doer1(0)
    }
}
impl core::fmt::Debug for Doer1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doer1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doer1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Doer1 {{ }}",)
    }
}
#[doc = "Data Output Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doesr0(pub u32);
impl Doesr0 {}
impl Default for Doesr0 {
    #[inline(always)]
    fn default() -> Doesr0 {
        Doesr0(0)
    }
}
impl core::fmt::Debug for Doesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doesr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doesr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Doesr0 {{ }}",)
    }
}
#[doc = "Data Output Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doesr1(pub u32);
impl Doesr1 {}
impl Default for Doesr1 {
    #[inline(always)]
    fn default() -> Doesr1 {
        Doesr1(0)
    }
}
impl core::fmt::Debug for Doesr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doesr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doesr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Doesr1 {{ }}",)
    }
}
#[doc = "Data Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dor0(pub u32);
impl Dor0 {}
impl Default for Dor0 {
    #[inline(always)]
    fn default() -> Dor0 {
        Dor0(0)
    }
}
impl core::fmt::Debug for Dor0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dor0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dor0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dor0 {{ }}",)
    }
}
#[doc = "Data Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dor1(pub u32);
impl Dor1 {}
impl Default for Dor1 {
    #[inline(always)]
    fn default() -> Dor1 {
        Dor1(0)
    }
}
impl core::fmt::Debug for Dor1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dor1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dor1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dor1 {{ }}",)
    }
}
#[doc = "Data Output Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dosr0(pub u32);
impl Dosr0 {}
impl Default for Dosr0 {
    #[inline(always)]
    fn default() -> Dosr0 {
        Dosr0(0)
    }
}
impl core::fmt::Debug for Dosr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dosr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dosr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dosr0 {{ }}",)
    }
}
#[doc = "Data Output Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dosr1(pub u32);
impl Dosr1 {}
impl Default for Dosr1 {
    #[inline(always)]
    fn default() -> Dosr1 {
        Dosr1(0)
    }
}
impl core::fmt::Debug for Dosr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dosr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dosr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dosr1 {{ }}",)
    }
}
#[doc = "Interrupt Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iecr0(pub u32);
impl Iecr0 {}
impl Default for Iecr0 {
    #[inline(always)]
    fn default() -> Iecr0 {
        Iecr0(0)
    }
}
impl core::fmt::Debug for Iecr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iecr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iecr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iecr0 {{ }}",)
    }
}
#[doc = "Interrupt Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iecr1(pub u32);
impl Iecr1 {}
impl Default for Iecr1 {
    #[inline(always)]
    fn default() -> Iecr1 {
        Iecr1(0)
    }
}
impl core::fmt::Debug for Iecr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iecr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iecr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iecr1 {{ }}",)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier0(pub u32);
impl Ier0 {}
impl Default for Ier0 {
    #[inline(always)]
    fn default() -> Ier0 {
        Ier0(0)
    }
}
impl core::fmt::Debug for Ier0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ier0 {{ }}",)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier1(pub u32);
impl Ier1 {}
impl Default for Ier1 {
    #[inline(always)]
    fn default() -> Ier1 {
        Ier1(0)
    }
}
impl core::fmt::Debug for Ier1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ier1 {{ }}",)
    }
}
#[doc = "Interrupt Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iesr0(pub u32);
impl Iesr0 {}
impl Default for Iesr0 {
    #[inline(always)]
    fn default() -> Iesr0 {
        Iesr0(0)
    }
}
impl core::fmt::Debug for Iesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iesr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iesr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iesr0 {{ }}",)
    }
}
#[doc = "Interrupt Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iesr1(pub u32);
impl Iesr1 {}
impl Default for Iesr1 {
    #[inline(always)]
    fn default() -> Iesr1 {
        Iesr1(0)
    }
}
impl core::fmt::Debug for Iesr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iesr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iesr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iesr1 {{ }}",)
    }
}
#[doc = "Interrupt Polarity High Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphcr0(pub u32);
impl Iphcr0 {}
impl Default for Iphcr0 {
    #[inline(always)]
    fn default() -> Iphcr0 {
        Iphcr0(0)
    }
}
impl core::fmt::Debug for Iphcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphcr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iphcr0 {{ }}",)
    }
}
#[doc = "Interrupt Polarity High Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphcr1(pub u32);
impl Iphcr1 {}
impl Default for Iphcr1 {
    #[inline(always)]
    fn default() -> Iphcr1 {
        Iphcr1(0)
    }
}
impl core::fmt::Debug for Iphcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphcr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iphcr1 {{ }}",)
    }
}
#[doc = "Interrupt Polarity High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphr0(pub u32);
impl Iphr0 {}
impl Default for Iphr0 {
    #[inline(always)]
    fn default() -> Iphr0 {
        Iphr0(0)
    }
}
impl core::fmt::Debug for Iphr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iphr0 {{ }}",)
    }
}
#[doc = "Interrupt Polarity High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphr1(pub u32);
impl Iphr1 {}
impl Default for Iphr1 {
    #[inline(always)]
    fn default() -> Iphr1 {
        Iphr1(0)
    }
}
impl core::fmt::Debug for Iphr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iphr1 {{ }}",)
    }
}
#[doc = "Interrupt Polarity High Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphsr0(pub u32);
impl Iphsr0 {}
impl Default for Iphsr0 {
    #[inline(always)]
    fn default() -> Iphsr0 {
        Iphsr0(0)
    }
}
impl core::fmt::Debug for Iphsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphsr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iphsr0 {{ }}",)
    }
}
#[doc = "Interrupt Polarity High Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphsr1(pub u32);
impl Iphsr1 {}
impl Default for Iphsr1 {
    #[inline(always)]
    fn default() -> Iphsr1 {
        Iphsr1(0)
    }
}
impl core::fmt::Debug for Iphsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphsr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iphsr1 {{ }}",)
    }
}
#[doc = "Interrupt Polarity Low Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplcr0(pub u32);
impl Iplcr0 {}
impl Default for Iplcr0 {
    #[inline(always)]
    fn default() -> Iplcr0 {
        Iplcr0(0)
    }
}
impl core::fmt::Debug for Iplcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplcr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iplcr0 {{ }}",)
    }
}
#[doc = "Interrupt Polarity Low Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplcr1(pub u32);
impl Iplcr1 {}
impl Default for Iplcr1 {
    #[inline(always)]
    fn default() -> Iplcr1 {
        Iplcr1(0)
    }
}
impl core::fmt::Debug for Iplcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplcr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iplcr1 {{ }}",)
    }
}
#[doc = "Interrupt Polarity Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplr0(pub u32);
impl Iplr0 {}
impl Default for Iplr0 {
    #[inline(always)]
    fn default() -> Iplr0 {
        Iplr0(0)
    }
}
impl core::fmt::Debug for Iplr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iplr0 {{ }}",)
    }
}
#[doc = "Interrupt Polarity Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplr1(pub u32);
impl Iplr1 {}
impl Default for Iplr1 {
    #[inline(always)]
    fn default() -> Iplr1 {
        Iplr1(0)
    }
}
impl core::fmt::Debug for Iplr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iplr1 {{ }}",)
    }
}
#[doc = "Interrupt Polarity Low Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplsr0(pub u32);
impl Iplsr0 {}
impl Default for Iplsr0 {
    #[inline(always)]
    fn default() -> Iplsr0 {
        Iplsr0(0)
    }
}
impl core::fmt::Debug for Iplsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplsr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iplsr0 {{ }}",)
    }
}
#[doc = "Interrupt Polarity Low Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplsr1(pub u32);
impl Iplsr1 {}
impl Default for Iplsr1 {
    #[inline(always)]
    fn default() -> Iplsr1 {
        Iplsr1(0)
    }
}
impl core::fmt::Debug for Iplsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplsr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iplsr1 {{ }}",)
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr0(pub u32);
impl Isr0 {}
impl Default for Isr0 {
    #[inline(always)]
    fn default() -> Isr0 {
        Isr0(0)
    }
}
impl core::fmt::Debug for Isr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr0 {{ }}",)
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr1(pub u32);
impl Isr1 {}
impl Default for Isr1 {
    #[inline(always)]
    fn default() -> Isr1 {
        Isr1(0)
    }
}
impl core::fmt::Debug for Isr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr1 {{ }}",)
    }
}
#[doc = "Interrupt Type Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itcr0(pub u32);
impl Itcr0 {}
impl Default for Itcr0 {
    #[inline(always)]
    fn default() -> Itcr0 {
        Itcr0(0)
    }
}
impl core::fmt::Debug for Itcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itcr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Itcr0 {{ }}",)
    }
}
#[doc = "Interrupt Type Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itcr1(pub u32);
impl Itcr1 {}
impl Default for Itcr1 {
    #[inline(always)]
    fn default() -> Itcr1 {
        Itcr1(0)
    }
}
impl core::fmt::Debug for Itcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itcr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Itcr1 {{ }}",)
    }
}
#[doc = "Interrupt Type Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itr0(pub u32);
impl Itr0 {}
impl Default for Itr0 {
    #[inline(always)]
    fn default() -> Itr0 {
        Itr0(0)
    }
}
impl core::fmt::Debug for Itr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Itr0 {{ }}",)
    }
}
#[doc = "Interrupt Type Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itr1(pub u32);
impl Itr1 {}
impl Default for Itr1 {
    #[inline(always)]
    fn default() -> Itr1 {
        Itr1(0)
    }
}
impl core::fmt::Debug for Itr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Itr1 {{ }}",)
    }
}
#[doc = "Interrupt Type Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itsr0(pub u32);
impl Itsr0 {}
impl Default for Itsr0 {
    #[inline(always)]
    fn default() -> Itsr0 {
        Itsr0(0)
    }
}
impl core::fmt::Debug for Itsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itsr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Itsr0 {{ }}",)
    }
}
#[doc = "Interrupt Type Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itsr1(pub u32);
impl Itsr1 {}
impl Default for Itsr1 {
    #[inline(always)]
    fn default() -> Itsr1 {
        Itsr1(0)
    }
}
impl core::fmt::Debug for Itsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itsr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Itsr1 {{ }}",)
    }
}
#[doc = "output mode Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemcr0(pub u32);
impl Oemcr0 {}
impl Default for Oemcr0 {
    #[inline(always)]
    fn default() -> Oemcr0 {
        Oemcr0(0)
    }
}
impl core::fmt::Debug for Oemcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemcr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oemcr0 {{ }}",)
    }
}
#[doc = "output mode Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemcr1(pub u32);
impl Oemcr1 {}
impl Default for Oemcr1 {
    #[inline(always)]
    fn default() -> Oemcr1 {
        Oemcr1(0)
    }
}
impl core::fmt::Debug for Oemcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemcr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oemcr1 {{ }}",)
    }
}
#[doc = "output mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemr0(pub u32);
impl Oemr0 {}
impl Default for Oemr0 {
    #[inline(always)]
    fn default() -> Oemr0 {
        Oemr0(0)
    }
}
impl core::fmt::Debug for Oemr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oemr0 {{ }}",)
    }
}
#[doc = "output mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemr1(pub u32);
impl Oemr1 {}
impl Default for Oemr1 {
    #[inline(always)]
    fn default() -> Oemr1 {
        Oemr1(0)
    }
}
impl core::fmt::Debug for Oemr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oemr1 {{ }}",)
    }
}
#[doc = "output mode Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemsr0(pub u32);
impl Oemsr0 {}
impl Default for Oemsr0 {
    #[inline(always)]
    fn default() -> Oemsr0 {
        Oemsr0(0)
    }
}
impl core::fmt::Debug for Oemsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemsr0").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oemsr0 {{ }}",)
    }
}
#[doc = "output mode Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemsr1(pub u32);
impl Oemsr1 {}
impl Default for Oemsr1 {
    #[inline(always)]
    fn default() -> Oemsr1 {
        Oemsr1(0)
    }
}
impl core::fmt::Debug for Oemsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemsr1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oemsr1 {{ }}",)
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
