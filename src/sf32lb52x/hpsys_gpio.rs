#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HpsysGpio {
    ptr: *mut u8,
}
unsafe impl Send for HpsysGpio {}
unsafe impl Sync for HpsysGpio {}
impl HpsysGpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data Input Register"]
    #[inline(always)]
    pub const fn dir0(self) -> crate::common::Reg<regs::Dir0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Data Output Register"]
    #[inline(always)]
    pub const fn dor0(self) -> crate::common::Reg<regs::Dor0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Data Output Set Register"]
    #[inline(always)]
    pub const fn dosr0(self) -> crate::common::Reg<regs::Dosr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Data Output Clear Register"]
    #[inline(always)]
    pub const fn docr0(self) -> crate::common::Reg<regs::Docr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Data Output Enable Register"]
    #[inline(always)]
    pub const fn doer0(self) -> crate::common::Reg<regs::Doer0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Data Output Enable Set Register"]
    #[inline(always)]
    pub const fn doesr0(self) -> crate::common::Reg<regs::Doesr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Data Output Enable Clear Register"]
    #[inline(always)]
    pub const fn doecr0(self) -> crate::common::Reg<regs::Doecr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier0(self) -> crate::common::Reg<regs::Ier0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn iesr0(self) -> crate::common::Reg<regs::Iesr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn iecr0(self) -> crate::common::Reg<regs::Iecr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Interrupt Type Register"]
    #[inline(always)]
    pub const fn itr0(self) -> crate::common::Reg<regs::Itr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Interrupt Type Set Register"]
    #[inline(always)]
    pub const fn itsr0(self) -> crate::common::Reg<regs::Itsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Type Clear Register"]
    #[inline(always)]
    pub const fn itcr0(self) -> crate::common::Reg<regs::Itcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Interrupt Polarity High Register"]
    #[inline(always)]
    pub const fn iphr0(self) -> crate::common::Reg<regs::Iphr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Interrupt Polarity High Set Register"]
    #[inline(always)]
    pub const fn iphsr0(self) -> crate::common::Reg<regs::Iphsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Interrupt Polarity High Clear Register"]
    #[inline(always)]
    pub const fn iphcr0(self) -> crate::common::Reg<regs::Iphcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Interrupt Polarity Low Register"]
    #[inline(always)]
    pub const fn iplr0(self) -> crate::common::Reg<regs::Iplr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Interrupt Polarity Low Set Register"]
    #[inline(always)]
    pub const fn iplsr0(self) -> crate::common::Reg<regs::Iplsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Interrupt Polarity Low Clear Register"]
    #[inline(always)]
    pub const fn iplcr0(self) -> crate::common::Reg<regs::Iplcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr0(self) -> crate::common::Reg<regs::Isr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "output mode Register"]
    #[inline(always)]
    pub const fn oemr0(self) -> crate::common::Reg<regs::Oemr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "output mode Set Register"]
    #[inline(always)]
    pub const fn oemsr0(self) -> crate::common::Reg<regs::Oemsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "output mode Clear Register"]
    #[inline(always)]
    pub const fn oemcr0(self) -> crate::common::Reg<regs::Oemcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Data Input Register"]
    #[inline(always)]
    pub const fn dir1(self) -> crate::common::Reg<regs::Dir1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Data Output Register"]
    #[inline(always)]
    pub const fn dor1(self) -> crate::common::Reg<regs::Dor1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Data Output Set Register"]
    #[inline(always)]
    pub const fn dosr1(self) -> crate::common::Reg<regs::Dosr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Data Output Clear Register"]
    #[inline(always)]
    pub const fn docr1(self) -> crate::common::Reg<regs::Docr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Data Output Enable Register"]
    #[inline(always)]
    pub const fn doer1(self) -> crate::common::Reg<regs::Doer1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Data Output Enable Set Register"]
    #[inline(always)]
    pub const fn doesr1(self) -> crate::common::Reg<regs::Doesr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Data Output Enable Clear Register"]
    #[inline(always)]
    pub const fn doecr1(self) -> crate::common::Reg<regs::Doecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier1(self) -> crate::common::Reg<regs::Ier1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn iesr1(self) -> crate::common::Reg<regs::Iesr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn iecr1(self) -> crate::common::Reg<regs::Iecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Interrupt Type Register"]
    #[inline(always)]
    pub const fn itr1(self) -> crate::common::Reg<regs::Itr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Interrupt Type Set Register"]
    #[inline(always)]
    pub const fn itsr1(self) -> crate::common::Reg<regs::Itsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Interrupt Type Clear Register"]
    #[inline(always)]
    pub const fn itcr1(self) -> crate::common::Reg<regs::Itcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Interrupt Polarity High Register"]
    #[inline(always)]
    pub const fn iphr1(self) -> crate::common::Reg<regs::Iphr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "Interrupt Polarity High Set Register"]
    #[inline(always)]
    pub const fn iphsr1(self) -> crate::common::Reg<regs::Iphsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Interrupt Polarity High Clear Register"]
    #[inline(always)]
    pub const fn iphcr1(self) -> crate::common::Reg<regs::Iphcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "Interrupt Polarity Low Register"]
    #[inline(always)]
    pub const fn iplr1(self) -> crate::common::Reg<regs::Iplr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Interrupt Polarity Low Set Register"]
    #[inline(always)]
    pub const fn iplsr1(self) -> crate::common::Reg<regs::Iplsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Interrupt Polarity Low Clear Register"]
    #[inline(always)]
    pub const fn iplcr1(self) -> crate::common::Reg<regs::Iplcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "output mode Register"]
    #[inline(always)]
    pub const fn oemr1(self) -> crate::common::Reg<regs::Oemr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "output mode Set Register"]
    #[inline(always)]
    pub const fn oemsr1(self) -> crate::common::Reg<regs::Oemsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "output mode Clear Register"]
    #[inline(always)]
    pub const fn oemcr1(self) -> crate::common::Reg<regs::Oemcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
}
pub mod regs;
