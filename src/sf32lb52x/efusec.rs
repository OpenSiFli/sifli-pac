#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Efusec {
    ptr: *mut u8,
}
unsafe impl Send for Efusec {}
unsafe impl Sync for Efusec {}
impl Efusec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer Register"]
    #[inline(always)]
    pub const fn timr(self) -> crate::common::Reg<regs::Timr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Program Data0"]
    #[inline(always)]
    pub const fn pgm_data0(self) -> crate::common::Reg<regs::PgmData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Program Data1"]
    #[inline(always)]
    pub const fn pgm_data1(self) -> crate::common::Reg<regs::PgmData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Program Data2"]
    #[inline(always)]
    pub const fn pgm_data2(self) -> crate::common::Reg<regs::PgmData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Program Data3"]
    #[inline(always)]
    pub const fn pgm_data3(self) -> crate::common::Reg<regs::PgmData3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Program Data4"]
    #[inline(always)]
    pub const fn pgm_data4(self) -> crate::common::Reg<regs::PgmData4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Program Data5"]
    #[inline(always)]
    pub const fn pgm_data5(self) -> crate::common::Reg<regs::PgmData5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Program Data6"]
    #[inline(always)]
    pub const fn pgm_data6(self) -> crate::common::Reg<regs::PgmData6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Program Data7"]
    #[inline(always)]
    pub const fn pgm_data7(self) -> crate::common::Reg<regs::PgmData7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Bank0 Data0"]
    #[inline(always)]
    pub const fn bank0_data0(self) -> crate::common::Reg<regs::Bank0Data0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Bank0 Data1"]
    #[inline(always)]
    pub const fn bank0_data1(self) -> crate::common::Reg<regs::Bank0Data1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Bank0 Data2"]
    #[inline(always)]
    pub const fn bank0_data2(self) -> crate::common::Reg<regs::Bank0Data2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Bank0 Data3"]
    #[inline(always)]
    pub const fn bank0_data3(self) -> crate::common::Reg<regs::Bank0Data3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Bank0 Data4"]
    #[inline(always)]
    pub const fn bank0_data4(self) -> crate::common::Reg<regs::Bank0Data4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Bank0 Data5"]
    #[inline(always)]
    pub const fn bank0_data5(self) -> crate::common::Reg<regs::Bank0Data5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Bank0 Data6"]
    #[inline(always)]
    pub const fn bank0_data6(self) -> crate::common::Reg<regs::Bank0Data6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Bank0 Data7"]
    #[inline(always)]
    pub const fn bank0_data7(self) -> crate::common::Reg<regs::Bank0Data7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Bank1 Data0"]
    #[inline(always)]
    pub const fn bank1_data0(self) -> crate::common::Reg<regs::Bank1Data0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Bank1 Data1"]
    #[inline(always)]
    pub const fn bank1_data1(self) -> crate::common::Reg<regs::Bank1Data1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Bank1 Data2"]
    #[inline(always)]
    pub const fn bank1_data2(self) -> crate::common::Reg<regs::Bank1Data2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Bank1 Data3"]
    #[inline(always)]
    pub const fn bank1_data3(self) -> crate::common::Reg<regs::Bank1Data3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Bank1 Data4"]
    #[inline(always)]
    pub const fn bank1_data4(self) -> crate::common::Reg<regs::Bank1Data4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Bank1 Data5"]
    #[inline(always)]
    pub const fn bank1_data5(self) -> crate::common::Reg<regs::Bank1Data5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Bank1 Data6"]
    #[inline(always)]
    pub const fn bank1_data6(self) -> crate::common::Reg<regs::Bank1Data6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Bank1 Data7"]
    #[inline(always)]
    pub const fn bank1_data7(self) -> crate::common::Reg<regs::Bank1Data7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Bank2 Data0"]
    #[inline(always)]
    pub const fn bank2_data0(self) -> crate::common::Reg<regs::Bank2Data0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Bank2 Data1"]
    #[inline(always)]
    pub const fn bank2_data1(self) -> crate::common::Reg<regs::Bank2Data1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Bank2 Data2"]
    #[inline(always)]
    pub const fn bank2_data2(self) -> crate::common::Reg<regs::Bank2Data2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Bank2 Data3"]
    #[inline(always)]
    pub const fn bank2_data3(self) -> crate::common::Reg<regs::Bank2Data3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Bank2 Data4"]
    #[inline(always)]
    pub const fn bank2_data4(self) -> crate::common::Reg<regs::Bank2Data4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Bank2 Data5"]
    #[inline(always)]
    pub const fn bank2_data5(self) -> crate::common::Reg<regs::Bank2Data5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Bank2 Data6"]
    #[inline(always)]
    pub const fn bank2_data6(self) -> crate::common::Reg<regs::Bank2Data6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Bank2 Data7"]
    #[inline(always)]
    pub const fn bank2_data7(self) -> crate::common::Reg<regs::Bank2Data7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Bank3 Data0"]
    #[inline(always)]
    pub const fn bank3_data0(self) -> crate::common::Reg<regs::Bank3Data0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Bank3 Data1"]
    #[inline(always)]
    pub const fn bank3_data1(self) -> crate::common::Reg<regs::Bank3Data1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Bank3 Data2"]
    #[inline(always)]
    pub const fn bank3_data2(self) -> crate::common::Reg<regs::Bank3Data2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Bank3 Data3"]
    #[inline(always)]
    pub const fn bank3_data3(self) -> crate::common::Reg<regs::Bank3Data3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Bank3 Data4"]
    #[inline(always)]
    pub const fn bank3_data4(self) -> crate::common::Reg<regs::Bank3Data4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Bank3 Data5"]
    #[inline(always)]
    pub const fn bank3_data5(self) -> crate::common::Reg<regs::Bank3Data5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Bank3 Data6"]
    #[inline(always)]
    pub const fn bank3_data6(self) -> crate::common::Reg<regs::Bank3Data6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Bank3 Data7"]
    #[inline(always)]
    pub const fn bank3_data7(self) -> crate::common::Reg<regs::Bank3Data7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Bank3 Data7"]
    #[inline(always)]
    pub const fn anacr(self) -> crate::common::Reg<regs::Anacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "debug signal select"]
    #[inline(always)]
    pub const fn db_sel(self) -> crate::common::Reg<regs::DbSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
}
pub mod regs;
