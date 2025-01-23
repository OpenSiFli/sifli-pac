#[doc = "Data Input Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir0(pub u32);
impl Dir0 {
    #[doc = "GPIO\\[31:0\\] input value"]
    #[inline(always)]
    pub const fn in_(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO\\[31:0\\] input value"]
    #[inline(always)]
    pub fn set_in_(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dir0 {
    #[inline(always)]
    fn default() -> Dir0 {
        Dir0(0)
    }
}
impl core::fmt::Debug for Dir0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dir0").field("in_", &self.in_()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dir0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dir0 {
            in_: u32,
        }
        let proxy = Dir0 { in_: self.in_() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Input Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir1(pub u32);
impl Dir1 {
    #[doc = "GPIO\\[44:32\\] input value"]
    #[inline(always)]
    pub const fn in_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "GPIO\\[44:32\\] input value"]
    #[inline(always)]
    pub fn set_in_(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Dir1 {
    #[inline(always)]
    fn default() -> Dir1 {
        Dir1(0)
    }
}
impl core::fmt::Debug for Dir1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dir1").field("in_", &self.in_()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dir1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dir1 {
            in_: u16,
        }
        let proxy = Dir1 { in_: self.in_() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Docr0(pub u32);
impl Docr0 {
    #[doc = "set 1 to pull down output of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn doc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 to pull down output of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_doc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Docr0 {
    #[inline(always)]
    fn default() -> Docr0 {
        Docr0(0)
    }
}
impl core::fmt::Debug for Docr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Docr0").field("doc", &self.doc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Docr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Docr0 {
            doc: u32,
        }
        let proxy = Docr0 { doc: self.doc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Docr1(pub u32);
impl Docr1 {
    #[doc = "set 1 to pull down output of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn doc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 to pull down output of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_doc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Docr1 {
    #[inline(always)]
    fn default() -> Docr1 {
        Docr1(0)
    }
}
impl core::fmt::Debug for Docr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Docr1").field("doc", &self.doc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Docr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Docr1 {
            doc: u16,
        }
        let proxy = Docr1 { doc: self.doc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doecr0(pub u32);
impl Doecr0 {
    #[doc = "set 1 to disable output of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn doec(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 to disable output of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_doec(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Doecr0 {
    #[inline(always)]
    fn default() -> Doecr0 {
        Doecr0(0)
    }
}
impl core::fmt::Debug for Doecr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doecr0")
            .field("doec", &self.doec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doecr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Doecr0 {
            doec: u32,
        }
        let proxy = Doecr0 { doec: self.doec() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doecr1(pub u32);
impl Doecr1 {
    #[doc = "set 1 to disable output of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn doec(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 to disable output of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_doec(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Doecr1 {
    #[inline(always)]
    fn default() -> Doecr1 {
        Doecr1(0)
    }
}
impl core::fmt::Debug for Doecr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doecr1")
            .field("doec", &self.doec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doecr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Doecr1 {
            doec: u16,
        }
        let proxy = Doecr1 { doec: self.doec() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doer0(pub u32);
impl Doer0 {
    #[doc = "GPIO\\[31:0\\] output enable"]
    #[inline(always)]
    pub const fn doe(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO\\[31:0\\] output enable"]
    #[inline(always)]
    pub fn set_doe(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Doer0 {
    #[inline(always)]
    fn default() -> Doer0 {
        Doer0(0)
    }
}
impl core::fmt::Debug for Doer0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doer0").field("doe", &self.doe()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doer0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Doer0 {
            doe: u32,
        }
        let proxy = Doer0 { doe: self.doe() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doer1(pub u32);
impl Doer1 {
    #[doc = "GPIO\\[44:32\\] output enable"]
    #[inline(always)]
    pub const fn doe(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "GPIO\\[44:32\\] output enable"]
    #[inline(always)]
    pub fn set_doe(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Doer1 {
    #[inline(always)]
    fn default() -> Doer1 {
        Doer1(0)
    }
}
impl core::fmt::Debug for Doer1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doer1").field("doe", &self.doe()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doer1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Doer1 {
            doe: u16,
        }
        let proxy = Doer1 { doe: self.doe() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doesr0(pub u32);
impl Doesr0 {
    #[doc = "set 1 to enable output of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn does(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 to enable output of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_does(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Doesr0 {
    #[inline(always)]
    fn default() -> Doesr0 {
        Doesr0(0)
    }
}
impl core::fmt::Debug for Doesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doesr0")
            .field("does", &self.does())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doesr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Doesr0 {
            does: u32,
        }
        let proxy = Doesr0 { does: self.does() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doesr1(pub u32);
impl Doesr1 {
    #[doc = "set 1 to enable output of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn does(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 to enable output of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_does(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Doesr1 {
    #[inline(always)]
    fn default() -> Doesr1 {
        Doesr1(0)
    }
}
impl core::fmt::Debug for Doesr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Doesr1")
            .field("does", &self.does())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Doesr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Doesr1 {
            does: u16,
        }
        let proxy = Doesr1 { does: self.does() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dor0(pub u32);
impl Dor0 {
    #[doc = "GPIO\\[31:0\\] output value if output enabled"]
    #[inline(always)]
    pub const fn out(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO\\[31:0\\] output value if output enabled"]
    #[inline(always)]
    pub fn set_out(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dor0 {
    #[inline(always)]
    fn default() -> Dor0 {
        Dor0(0)
    }
}
impl core::fmt::Debug for Dor0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dor0").field("out", &self.out()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dor0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dor0 {
            out: u32,
        }
        let proxy = Dor0 { out: self.out() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dor1(pub u32);
impl Dor1 {
    #[doc = "GPIO\\[44:32\\] output value if output enabled"]
    #[inline(always)]
    pub const fn out(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "GPIO\\[44:32\\] output value if output enabled"]
    #[inline(always)]
    pub fn set_out(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Dor1 {
    #[inline(always)]
    fn default() -> Dor1 {
        Dor1(0)
    }
}
impl core::fmt::Debug for Dor1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dor1").field("out", &self.out()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dor1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dor1 {
            out: u16,
        }
        let proxy = Dor1 { out: self.out() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dosr0(pub u32);
impl Dosr0 {
    #[doc = "set 1 to pull up output of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn dos(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 to pull up output of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_dos(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dosr0 {
    #[inline(always)]
    fn default() -> Dosr0 {
        Dosr0(0)
    }
}
impl core::fmt::Debug for Dosr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dosr0").field("dos", &self.dos()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dosr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dosr0 {
            dos: u32,
        }
        let proxy = Dosr0 { dos: self.dos() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Output Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dosr1(pub u32);
impl Dosr1 {
    #[doc = "set 1 to pull up output of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn dos(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 to pull up output of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_dos(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Dosr1 {
    #[inline(always)]
    fn default() -> Dosr1 {
        Dosr1(0)
    }
}
impl core::fmt::Debug for Dosr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dosr1").field("dos", &self.dos()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dosr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dosr1 {
            dos: u16,
        }
        let proxy = Dosr1 { dos: self.dos() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iecr0(pub u32);
impl Iecr0 {
    #[doc = "set 1 to disable interrupt of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn iec(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 to disable interrupt of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_iec(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iecr0 {
    #[inline(always)]
    fn default() -> Iecr0 {
        Iecr0(0)
    }
}
impl core::fmt::Debug for Iecr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iecr0").field("iec", &self.iec()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iecr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iecr0 {
            iec: u32,
        }
        let proxy = Iecr0 { iec: self.iec() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iecr1(pub u32);
impl Iecr1 {
    #[doc = "set 1 to disable interrupt of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn iec(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 to disable interrupt of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_iec(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Iecr1 {
    #[inline(always)]
    fn default() -> Iecr1 {
        Iecr1(0)
    }
}
impl core::fmt::Debug for Iecr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iecr1").field("iec", &self.iec()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iecr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iecr1 {
            iec: u16,
        }
        let proxy = Iecr1 { iec: self.iec() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier0(pub u32);
impl Ier0 {
    #[doc = "GPIO\\[31:0\\] interrupt enable"]
    #[inline(always)]
    pub const fn ier(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO\\[31:0\\] interrupt enable"]
    #[inline(always)]
    pub fn set_ier(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ier0 {
    #[inline(always)]
    fn default() -> Ier0 {
        Ier0(0)
    }
}
impl core::fmt::Debug for Ier0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier0").field("ier", &self.ier()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ier0 {
            ier: u32,
        }
        let proxy = Ier0 { ier: self.ier() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier1(pub u32);
impl Ier1 {
    #[doc = "GPIO\\[44:32\\] interrupt enable"]
    #[inline(always)]
    pub const fn ier(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "GPIO\\[44:32\\] interrupt enable"]
    #[inline(always)]
    pub fn set_ier(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Ier1 {
    #[inline(always)]
    fn default() -> Ier1 {
        Ier1(0)
    }
}
impl core::fmt::Debug for Ier1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier1").field("ier", &self.ier()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ier1 {
            ier: u16,
        }
        let proxy = Ier1 { ier: self.ier() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iesr0(pub u32);
impl Iesr0 {
    #[doc = "set 1 to enable interrupt of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn ies(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 to enable interrupt of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_ies(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iesr0 {
    #[inline(always)]
    fn default() -> Iesr0 {
        Iesr0(0)
    }
}
impl core::fmt::Debug for Iesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iesr0").field("ies", &self.ies()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iesr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iesr0 {
            ies: u32,
        }
        let proxy = Iesr0 { ies: self.ies() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iesr1(pub u32);
impl Iesr1 {
    #[doc = "set 1 to enable interrupt of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn ies(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 to enable interrupt of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_ies(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Iesr1 {
    #[inline(always)]
    fn default() -> Iesr1 {
        Iesr1(0)
    }
}
impl core::fmt::Debug for Iesr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iesr1").field("ies", &self.ies()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iesr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iesr1 {
            ies: u16,
        }
        let proxy = Iesr1 { ies: self.ies() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity High Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphcr0(pub u32);
impl Iphcr0 {
    #[doc = "set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn iphc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_iphc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iphcr0 {
    #[inline(always)]
    fn default() -> Iphcr0 {
        Iphcr0(0)
    }
}
impl core::fmt::Debug for Iphcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphcr0")
            .field("iphc", &self.iphc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iphcr0 {
            iphc: u32,
        }
        let proxy = Iphcr0 { iphc: self.iphc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity High Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphcr1(pub u32);
impl Iphcr1 {
    #[doc = "set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn iphc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 for disable rising edge in edge mode, or high level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_iphc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Iphcr1 {
    #[inline(always)]
    fn default() -> Iphcr1 {
        Iphcr1(0)
    }
}
impl core::fmt::Debug for Iphcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphcr1")
            .field("iphc", &self.iphc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iphcr1 {
            iphc: u16,
        }
        let proxy = Iphcr1 { iphc: self.iphc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphr0(pub u32);
impl Iphr0 {
    #[doc = "rising edge in edge mode, or high level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn iph(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "rising edge in edge mode, or high level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_iph(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iphr0 {
    #[inline(always)]
    fn default() -> Iphr0 {
        Iphr0(0)
    }
}
impl core::fmt::Debug for Iphr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphr0").field("iph", &self.iph()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iphr0 {
            iph: u32,
        }
        let proxy = Iphr0 { iph: self.iph() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity High Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphr1(pub u32);
impl Iphr1 {
    #[doc = "rising edge in edge mode, or high level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn iph(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "rising edge in edge mode, or high level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_iph(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Iphr1 {
    #[inline(always)]
    fn default() -> Iphr1 {
        Iphr1(0)
    }
}
impl core::fmt::Debug for Iphr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphr1").field("iph", &self.iph()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iphr1 {
            iph: u16,
        }
        let proxy = Iphr1 { iph: self.iph() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity High Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphsr0(pub u32);
impl Iphsr0 {
    #[doc = "set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn iphs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_iphs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iphsr0 {
    #[inline(always)]
    fn default() -> Iphsr0 {
        Iphsr0(0)
    }
}
impl core::fmt::Debug for Iphsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphsr0")
            .field("iphs", &self.iphs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphsr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iphsr0 {
            iphs: u32,
        }
        let proxy = Iphsr0 { iphs: self.iphs() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity High Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iphsr1(pub u32);
impl Iphsr1 {
    #[doc = "set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn iphs(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_iphs(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Iphsr1 {
    #[inline(always)]
    fn default() -> Iphsr1 {
        Iphsr1(0)
    }
}
impl core::fmt::Debug for Iphsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iphsr1")
            .field("iphs", &self.iphs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iphsr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iphsr1 {
            iphs: u16,
        }
        let proxy = Iphsr1 { iphs: self.iphs() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity Low Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplcr0(pub u32);
impl Iplcr0 {
    #[doc = "set 1 for disable falling edge in edge mode, or low level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn iplc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 for disable falling edge in edge mode, or low level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_iplc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iplcr0 {
    #[inline(always)]
    fn default() -> Iplcr0 {
        Iplcr0(0)
    }
}
impl core::fmt::Debug for Iplcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplcr0")
            .field("iplc", &self.iplc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iplcr0 {
            iplc: u32,
        }
        let proxy = Iplcr0 { iplc: self.iplc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity Low Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplcr1(pub u32);
impl Iplcr1 {
    #[doc = "set 1 for disable falling edge in edge mode, or low level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn iplc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 for disable falling edge in edge mode, or low level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_iplc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Iplcr1 {
    #[inline(always)]
    fn default() -> Iplcr1 {
        Iplcr1(0)
    }
}
impl core::fmt::Debug for Iplcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplcr1")
            .field("iplc", &self.iplc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iplcr1 {
            iplc: u16,
        }
        let proxy = Iplcr1 { iplc: self.iplc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplr0(pub u32);
impl Iplr0 {
    #[doc = "falling edge in edge mode, or low level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn ipl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "falling edge in edge mode, or low level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_ipl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iplr0 {
    #[inline(always)]
    fn default() -> Iplr0 {
        Iplr0(0)
    }
}
impl core::fmt::Debug for Iplr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplr0").field("ipl", &self.ipl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iplr0 {
            ipl: u32,
        }
        let proxy = Iplr0 { ipl: self.ipl() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity Low Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplr1(pub u32);
impl Iplr1 {
    #[doc = "falling edge in edge mode, or low level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn ipl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "falling edge in edge mode, or low level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_ipl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Iplr1 {
    #[inline(always)]
    fn default() -> Iplr1 {
        Iplr1(0)
    }
}
impl core::fmt::Debug for Iplr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplr1").field("ipl", &self.ipl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iplr1 {
            ipl: u16,
        }
        let proxy = Iplr1 { ipl: self.ipl() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity Low Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplsr0(pub u32);
impl Iplsr0 {
    #[doc = "set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn ipls(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_ipls(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iplsr0 {
    #[inline(always)]
    fn default() -> Iplsr0 {
        Iplsr0(0)
    }
}
impl core::fmt::Debug for Iplsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplsr0")
            .field("ipls", &self.ipls())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplsr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iplsr0 {
            ipls: u32,
        }
        let proxy = Iplsr0 { ipls: self.ipls() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Polarity Low Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iplsr1(pub u32);
impl Iplsr1 {
    #[doc = "set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn ipls(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_ipls(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Iplsr1 {
    #[inline(always)]
    fn default() -> Iplsr1 {
        Iplsr1(0)
    }
}
impl core::fmt::Debug for Iplsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iplsr1")
            .field("ipls", &self.ipls())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iplsr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iplsr1 {
            ipls: u16,
        }
        let proxy = Iplsr1 { ipls: self.ipls() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr0(pub u32);
impl Isr0 {
    #[doc = "Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn is(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_is(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isr0 {
    #[inline(always)]
    fn default() -> Isr0 {
        Isr0(0)
    }
}
impl core::fmt::Debug for Isr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr0").field("is", &self.is()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isr0 {
            is: u32,
        }
        let proxy = Isr0 { is: self.is() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr1(pub u32);
impl Isr1 {
    #[doc = "Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn is(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_is(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Isr1 {
    #[inline(always)]
    fn default() -> Isr1 {
        Isr1(0)
    }
}
impl core::fmt::Debug for Isr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr1").field("is", &self.is()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isr1 {
            is: u16,
        }
        let proxy = Isr1 { is: self.is() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Type Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itcr0(pub u32);
impl Itcr0 {
    #[doc = "set 1 for level-sensitive interrupt mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn itc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 for level-sensitive interrupt mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_itc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Itcr0 {
    #[inline(always)]
    fn default() -> Itcr0 {
        Itcr0(0)
    }
}
impl core::fmt::Debug for Itcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itcr0").field("itc", &self.itc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Itcr0 {
            itc: u32,
        }
        let proxy = Itcr0 { itc: self.itc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Type Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itcr1(pub u32);
impl Itcr1 {
    #[doc = "set 1 for level-sensitive interrupt mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn itc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 for level-sensitive interrupt mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_itc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Itcr1 {
    #[inline(always)]
    fn default() -> Itcr1 {
        Itcr1(0)
    }
}
impl core::fmt::Debug for Itcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itcr1").field("itc", &self.itc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Itcr1 {
            itc: u16,
        }
        let proxy = Itcr1 { itc: self.itc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Type Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itr0(pub u32);
impl Itr0 {
    #[doc = "GPIO\\[31:0\\] interrupt type"]
    #[inline(always)]
    pub const fn itr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO\\[31:0\\] interrupt type"]
    #[inline(always)]
    pub fn set_itr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Itr0 {
    #[inline(always)]
    fn default() -> Itr0 {
        Itr0(0)
    }
}
impl core::fmt::Debug for Itr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itr0").field("itr", &self.itr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Itr0 {
            itr: u32,
        }
        let proxy = Itr0 { itr: self.itr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Type Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itr1(pub u32);
impl Itr1 {
    #[doc = "GPIO\\[44:32\\] interrupt type"]
    #[inline(always)]
    pub const fn itr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "GPIO\\[44:32\\] interrupt type"]
    #[inline(always)]
    pub fn set_itr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Itr1 {
    #[inline(always)]
    fn default() -> Itr1 {
        Itr1(0)
    }
}
impl core::fmt::Debug for Itr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itr1").field("itr", &self.itr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Itr1 {
            itr: u16,
        }
        let proxy = Itr1 { itr: self.itr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Type Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itsr0(pub u32);
impl Itsr0 {
    #[doc = "set 1 for edge-sensitive interrupt mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn its(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "set 1 for edge-sensitive interrupt mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_its(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Itsr0 {
    #[inline(always)]
    fn default() -> Itsr0 {
        Itsr0(0)
    }
}
impl core::fmt::Debug for Itsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itsr0").field("its", &self.its()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itsr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Itsr0 {
            its: u32,
        }
        let proxy = Itsr0 { its: self.its() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Type Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itsr1(pub u32);
impl Itsr1 {
    #[doc = "set 1 for edge-sensitive interrupt mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn its(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "set 1 for edge-sensitive interrupt mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_its(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Itsr1 {
    #[inline(always)]
    fn default() -> Itsr1 {
        Itsr1(0)
    }
}
impl core::fmt::Debug for Itsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itsr1").field("its", &self.its()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itsr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Itsr1 {
            its: u16,
        }
        let proxy = Itsr1 { its: self.its() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "output mode Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemcr0(pub u32);
impl Oemcr0 {
    #[doc = "output mode Clear of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn oemc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "output mode Clear of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_oemc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Oemcr0 {
    #[inline(always)]
    fn default() -> Oemcr0 {
        Oemcr0(0)
    }
}
impl core::fmt::Debug for Oemcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemcr0")
            .field("oemc", &self.oemc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Oemcr0 {
            oemc: u32,
        }
        let proxy = Oemcr0 { oemc: self.oemc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "output mode Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemcr1(pub u32);
impl Oemcr1 {
    #[doc = "output mode Clear of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn oemc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "output mode Clear of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_oemc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Oemcr1 {
    #[inline(always)]
    fn default() -> Oemcr1 {
        Oemcr1(0)
    }
}
impl core::fmt::Debug for Oemcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemcr1")
            .field("oemc", &self.oemc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Oemcr1 {
            oemc: u16,
        }
        let proxy = Oemcr1 { oemc: self.oemc() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "output mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemr0(pub u32);
impl Oemr0 {
    #[doc = "output mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn oem(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "output mode of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_oem(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Oemr0 {
    #[inline(always)]
    fn default() -> Oemr0 {
        Oemr0(0)
    }
}
impl core::fmt::Debug for Oemr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemr0").field("oem", &self.oem()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Oemr0 {
            oem: u32,
        }
        let proxy = Oemr0 { oem: self.oem() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "output mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemr1(pub u32);
impl Oemr1 {
    #[doc = "output mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn oem(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "output mode of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_oem(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Oemr1 {
    #[inline(always)]
    fn default() -> Oemr1 {
        Oemr1(0)
    }
}
impl core::fmt::Debug for Oemr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemr1").field("oem", &self.oem()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Oemr1 {
            oem: u16,
        }
        let proxy = Oemr1 { oem: self.oem() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "output mode Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemsr0(pub u32);
impl Oemsr0 {
    #[doc = "output mode Set of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub const fn oems(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "output mode Set of corresponding GPIO\\[31:0\\]"]
    #[inline(always)]
    pub fn set_oems(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Oemsr0 {
    #[inline(always)]
    fn default() -> Oemsr0 {
        Oemsr0(0)
    }
}
impl core::fmt::Debug for Oemsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemsr0")
            .field("oems", &self.oems())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemsr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Oemsr0 {
            oems: u32,
        }
        let proxy = Oemsr0 { oems: self.oems() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "output mode Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oemsr1(pub u32);
impl Oemsr1 {
    #[doc = "output mode Set of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub const fn oems(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "output mode Set of corresponding GPIO\\[44:32\\]"]
    #[inline(always)]
    pub fn set_oems(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Oemsr1 {
    #[inline(always)]
    fn default() -> Oemsr1 {
        Oemsr1(0)
    }
}
impl core::fmt::Debug for Oemsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oemsr1")
            .field("oems", &self.oems())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oemsr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Oemsr1 {
            oems: u16,
        }
        let proxy = Oemsr1 { oems: self.oems() };
        defmt::write!(f, "{}", proxy)
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
        #[derive(defmt :: Format)]
        struct Rsvd1 {}
        let proxy = Rsvd1 {};
        defmt::write!(f, "{}", proxy)
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
        #[derive(defmt :: Format)]
        struct Rsvd2 {}
        let proxy = Rsvd2 {};
        defmt::write!(f, "{}", proxy)
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
        #[derive(defmt :: Format)]
        struct Rsvd3 {}
        let proxy = Rsvd3 {};
        defmt::write!(f, "{}", proxy)
    }
}
