#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtMac {
    ptr: *mut u8,
}
unsafe impl Send for BtMac {}
unsafe impl Sync for BtMac {}
impl BtMac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn rwdmcntl(self) -> crate::common::Reg<regs::Rwdmcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn dmversion(self) -> crate::common::Reg<regs::Dmversion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn dmintcntl0(self) -> crate::common::Reg<regs::Dmintcntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmintstat0(self) -> crate::common::Reg<regs::Dmintstat0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn dmintack0(self) -> crate::common::Reg<regs::Dmintack0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn dmintcntl1(self) -> crate::common::Reg<regs::Dmintcntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn dmintstat1(self) -> crate::common::Reg<regs::Dmintstat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmintack1(self) -> crate::common::Reg<regs::Dmintack1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn actfifostat(self) -> crate::common::Reg<regs::Actfifostat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn etptr(self) -> crate::common::Reg<regs::Etptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn deepslcntl(self) -> crate::common::Reg<regs::Deepslcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn finecntcorr(self) -> crate::common::Reg<regs::Finecntcorr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn clkncntcorr(self) -> crate::common::Reg<regs::Clkncntcorr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn dmdiagcntl(self) -> crate::common::Reg<regs::Dmdiagcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn dmdiagstat(self) -> crate::common::Reg<regs::Dmdiagstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn dmdebugaddmax(self) -> crate::common::Reg<regs::Dmdebugaddmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn dmdebugaddmin(self) -> crate::common::Reg<regs::Dmdebugaddmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmerrortypestat(
        self,
    ) -> crate::common::Reg<regs::Dmerrortypestat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn dmswprofiling(self) -> crate::common::Reg<regs::Dmswprofiling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn dmradiocntl0(self) -> crate::common::Reg<regs::Dmradiocntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn dmradiocntl1(self) -> crate::common::Reg<regs::Dmradiocntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn dmradiocntl2(self) -> crate::common::Reg<regs::Dmradiocntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn dmradiocntl3(self) -> crate::common::Reg<regs::Dmradiocntl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn dmradiocntl4(self) -> crate::common::Reg<regs::Dmradiocntl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn aescntl(self) -> crate::common::Reg<regs::Aescntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn aeskey31_0(self) -> crate::common::Reg<regs::Aeskey310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn aeskey63_32(self) -> crate::common::Reg<regs::Aeskey6332, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn aeskey95_64(self) -> crate::common::Reg<regs::Aeskey9564, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn aeskey127_96(self) -> crate::common::Reg<regs::Aeskey12796, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn dmaesptr(self) -> crate::common::Reg<regs::Dmaesptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn txmicval(self) -> crate::common::Reg<regs::Txmicval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn rxmicval(self) -> crate::common::Reg<regs::Rxmicval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn prioscharb(self) -> crate::common::Reg<regs::Prioscharb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn dmtimgencntl(self) -> crate::common::Reg<regs::Dmtimgencntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn finetimtgt(self) -> crate::common::Reg<regs::Finetimtgt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn clkntgt1(self) -> crate::common::Reg<regs::Clkntgt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn hmicrosectgt1(self) -> crate::common::Reg<regs::Hmicrosectgt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn clkntgt2(self) -> crate::common::Reg<regs::Clkntgt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn hmicrosectgt2(self) -> crate::common::Reg<regs::Hmicrosectgt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn clkntgt3(self) -> crate::common::Reg<regs::Clkntgt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn hmicrosectgt3(self) -> crate::common::Reg<regs::Hmicrosectgt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn slotclk(self) -> crate::common::Reg<regs::Slotclk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn finetimecnt(self) -> crate::common::Reg<regs::Finetimecnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn actschcntl(self) -> crate::common::Reg<regs::Actschcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn rccal_ctrl(self) -> crate::common::Reg<regs::RccalCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn rccal_result(self) -> crate::common::Reg<regs::RccalResult, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn dfancntl(self) -> crate::common::Reg<regs::Dfancntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[inline(always)]
    pub const fn rwbtcntl(self) -> crate::common::Reg<regs::Rwbtcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn btversion(self) -> crate::common::Reg<regs::Btversion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[inline(always)]
    pub const fn btintcntl0(self) -> crate::common::Reg<regs::Btintcntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[inline(always)]
    pub const fn btintstat0(self) -> crate::common::Reg<regs::Btintstat0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[inline(always)]
    pub const fn btintack0(self) -> crate::common::Reg<regs::Btintack0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[inline(always)]
    pub const fn btcurrentrxdescptr(
        self,
    ) -> crate::common::Reg<regs::Btcurrentrxdescptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[inline(always)]
    pub const fn btdiagcntl(self) -> crate::common::Reg<regs::Btdiagcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[inline(always)]
    pub const fn btdiagstat(self) -> crate::common::Reg<regs::Btdiagstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[inline(always)]
    pub const fn btdebugaddmax(self) -> crate::common::Reg<regs::Btdebugaddmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[inline(always)]
    pub const fn btdebugaddmin(self) -> crate::common::Reg<regs::Btdebugaddmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[inline(always)]
    pub const fn bterrortypestat(
        self,
    ) -> crate::common::Reg<regs::Bterrortypestat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[inline(always)]
    pub const fn btswprofiling(self) -> crate::common::Reg<regs::Btswprofiling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[inline(always)]
    pub const fn btradiocntl2(self) -> crate::common::Reg<regs::Btradiocntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    #[inline(always)]
    pub const fn btradiocntl3(self) -> crate::common::Reg<regs::Btradiocntl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[inline(always)]
    pub const fn btradiopwrupdn(
        self,
    ) -> crate::common::Reg<regs::Btradiopwrupdn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x048cusize) as _) }
    }
    #[inline(always)]
    pub const fn btradiotxrxtim(
        self,
    ) -> crate::common::Reg<regs::Btradiotxrxtim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[inline(always)]
    pub const fn btrftestcntl(self) -> crate::common::Reg<regs::Btrftestcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[inline(always)]
    pub const fn btrftestfreq(self) -> crate::common::Reg<regs::Btrftestfreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[inline(always)]
    pub const fn btrftesttxstat(
        self,
    ) -> crate::common::Reg<regs::Btrftesttxstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[inline(always)]
    pub const fn btrftestrxstat(
        self,
    ) -> crate::common::Reg<regs::Btrftestrxstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[inline(always)]
    pub const fn startfrmclknts(
        self,
    ) -> crate::common::Reg<regs::Startfrmclknts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[inline(always)]
    pub const fn startfrmfinecntts(
        self,
    ) -> crate::common::Reg<regs::Startfrmfinecntts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[inline(always)]
    pub const fn endfrmclknts(self) -> crate::common::Reg<regs::Endfrmclknts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[inline(always)]
    pub const fn endfrmfinecntts(
        self,
    ) -> crate::common::Reg<regs::Endfrmfinecntts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[inline(always)]
    pub const fn skipfrmclknts(self) -> crate::common::Reg<regs::Skipfrmclknts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[inline(always)]
    pub const fn skipfrmfinecntts(
        self,
    ) -> crate::common::Reg<regs::Skipfrmfinecntts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[inline(always)]
    pub const fn abtraincntl(self) -> crate::common::Reg<regs::Abtraincntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[inline(always)]
    pub const fn edrcntl(self) -> crate::common::Reg<regs::Edrcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[inline(always)]
    pub const fn pcacntl0(self) -> crate::common::Reg<regs::Pcacntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[inline(always)]
    pub const fn pcacntl1(self) -> crate::common::Reg<regs::Pcacntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[inline(always)]
    pub const fn pcastat(self) -> crate::common::Reg<regs::Pcastat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[inline(always)]
    pub const fn btcoexifcntl0(self) -> crate::common::Reg<regs::Btcoexifcntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[inline(always)]
    pub const fn btcoexifcntl1(self) -> crate::common::Reg<regs::Btcoexifcntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[inline(always)]
    pub const fn btcoexifcntl2(self) -> crate::common::Reg<regs::Btcoexifcntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[inline(always)]
    pub const fn btmprio0(self) -> crate::common::Reg<regs::Btmprio0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[inline(always)]
    pub const fn btmprio1(self) -> crate::common::Reg<regs::Btmprio1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[inline(always)]
    pub const fn btmprio2(self) -> crate::common::Reg<regs::Btmprio2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[inline(always)]
    pub const fn coexchn0(self) -> crate::common::Reg<regs::Coexchn0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[inline(always)]
    pub const fn coexchn1(self) -> crate::common::Reg<regs::Coexchn1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[inline(always)]
    pub const fn coexchn2(self) -> crate::common::Reg<regs::Coexchn2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[inline(always)]
    pub const fn escochancntl0(self) -> crate::common::Reg<regs::Escochancntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[inline(always)]
    pub const fn escomutecntl0(self) -> crate::common::Reg<regs::Escomutecntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[inline(always)]
    pub const fn escocurrenttxptr0(
        self,
    ) -> crate::common::Reg<regs::Escocurrenttxptr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[inline(always)]
    pub const fn escocurrentrxptr0(
        self,
    ) -> crate::common::Reg<regs::Escocurrentrxptr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[inline(always)]
    pub const fn escoltcntl0(self) -> crate::common::Reg<regs::Escoltcntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[inline(always)]
    pub const fn escotrcntl0(self) -> crate::common::Reg<regs::Escotrcntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[inline(always)]
    pub const fn escodaycnt0(self) -> crate::common::Reg<regs::Escodaycnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[inline(always)]
    pub const fn escochancntl1(self) -> crate::common::Reg<regs::Escochancntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[inline(always)]
    pub const fn escomutecntl1(self) -> crate::common::Reg<regs::Escomutecntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0634usize) as _) }
    }
    #[inline(always)]
    pub const fn escocurrenttxptr1(
        self,
    ) -> crate::common::Reg<regs::Escocurrenttxptr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0638usize) as _) }
    }
    #[inline(always)]
    pub const fn escocurrentrxptr1(
        self,
    ) -> crate::common::Reg<regs::Escocurrentrxptr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x063cusize) as _) }
    }
    #[inline(always)]
    pub const fn escoltcntl1(self) -> crate::common::Reg<regs::Escoltcntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[inline(always)]
    pub const fn escotrcntl1(self) -> crate::common::Reg<regs::Escotrcntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0644usize) as _) }
    }
    #[inline(always)]
    pub const fn escodaycnt1(self) -> crate::common::Reg<regs::Escodaycnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0648usize) as _) }
    }
    #[inline(always)]
    pub const fn escochancntl2(self) -> crate::common::Reg<regs::Escochancntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0650usize) as _) }
    }
    #[inline(always)]
    pub const fn escomutecntl2(self) -> crate::common::Reg<regs::Escomutecntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0654usize) as _) }
    }
    #[inline(always)]
    pub const fn escocurrenttxptr2(
        self,
    ) -> crate::common::Reg<regs::Escocurrenttxptr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0658usize) as _) }
    }
    #[inline(always)]
    pub const fn escocurrentrxptr2(
        self,
    ) -> crate::common::Reg<regs::Escocurrentrxptr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x065cusize) as _) }
    }
    #[inline(always)]
    pub const fn escoltcntl2(self) -> crate::common::Reg<regs::Escoltcntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[inline(always)]
    pub const fn escotrcntl2(self) -> crate::common::Reg<regs::Escotrcntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0664usize) as _) }
    }
    #[inline(always)]
    pub const fn escodaycnt2(self) -> crate::common::Reg<regs::Escodaycnt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0668usize) as _) }
    }
    #[inline(always)]
    pub const fn audiocntl0(self) -> crate::common::Reg<regs::Audiocntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0670usize) as _) }
    }
    #[inline(always)]
    pub const fn audiocntl1(self) -> crate::common::Reg<regs::Audiocntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0674usize) as _) }
    }
    #[inline(always)]
    pub const fn audiocntl2(self) -> crate::common::Reg<regs::Audiocntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0678usize) as _) }
    }
    #[inline(always)]
    pub const fn rwblecntl(self) -> crate::common::Reg<regs::Rwblecntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[inline(always)]
    pub const fn bleversion(self) -> crate::common::Reg<regs::Bleversion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[inline(always)]
    pub const fn bleintcntl0(self) -> crate::common::Reg<regs::Bleintcntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[inline(always)]
    pub const fn bleintstat0(self) -> crate::common::Reg<regs::Bleintstat0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[inline(always)]
    pub const fn bleintack0(self) -> crate::common::Reg<regs::Bleintack0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[inline(always)]
    pub const fn blecurrentrxdescptr(
        self,
    ) -> crate::common::Reg<regs::Blecurrentrxdescptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0828usize) as _) }
    }
    #[inline(always)]
    pub const fn blediagcntl(self) -> crate::common::Reg<regs::Blediagcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize) as _) }
    }
    #[inline(always)]
    pub const fn blediagstat(self) -> crate::common::Reg<regs::Blediagstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0854usize) as _) }
    }
    #[inline(always)]
    pub const fn bledebugaddmax(
        self,
    ) -> crate::common::Reg<regs::Bledebugaddmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0858usize) as _) }
    }
    #[inline(always)]
    pub const fn bledebugaddmin(
        self,
    ) -> crate::common::Reg<regs::Bledebugaddmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x085cusize) as _) }
    }
    #[inline(always)]
    pub const fn bleerrortypestat(
        self,
    ) -> crate::common::Reg<regs::Bleerrortypestat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0860usize) as _) }
    }
    #[inline(always)]
    pub const fn bleswprofiling(
        self,
    ) -> crate::common::Reg<regs::Bleswprofiling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0864usize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiocntl2(self) -> crate::common::Reg<regs::Bleradiocntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0878usize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiocntl3(self) -> crate::common::Reg<regs::Bleradiocntl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x087cusize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiopwrupdn0(
        self,
    ) -> crate::common::Reg<regs::Bleradiopwrupdn0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiopwrupdn1(
        self,
    ) -> crate::common::Reg<regs::Bleradiopwrupdn1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0884usize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiopwrupdn2(
        self,
    ) -> crate::common::Reg<regs::Bleradiopwrupdn2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0888usize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiopwrupdn3(
        self,
    ) -> crate::common::Reg<regs::Bleradiopwrupdn3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x088cusize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiotxrxtim0(
        self,
    ) -> crate::common::Reg<regs::Bleradiotxrxtim0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0890usize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiotxrxtim1(
        self,
    ) -> crate::common::Reg<regs::Bleradiotxrxtim1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0894usize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiotxrxtim2(
        self,
    ) -> crate::common::Reg<regs::Bleradiotxrxtim2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0898usize) as _) }
    }
    #[inline(always)]
    pub const fn bleradiotxrxtim3(
        self,
    ) -> crate::common::Reg<regs::Bleradiotxrxtim3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x089cusize) as _) }
    }
    #[inline(always)]
    pub const fn blerftestcntl(self) -> crate::common::Reg<regs::Blerftestcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d0usize) as _) }
    }
    #[inline(always)]
    pub const fn blerftesttxstat(
        self,
    ) -> crate::common::Reg<regs::Blerftesttxstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d4usize) as _) }
    }
    #[inline(always)]
    pub const fn blerftestrxstat(
        self,
    ) -> crate::common::Reg<regs::Blerftestrxstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d8usize) as _) }
    }
    #[inline(always)]
    pub const fn startevtclkn(self) -> crate::common::Reg<regs::Startevtclkn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0914usize) as _) }
    }
    #[inline(always)]
    pub const fn startevtfinecnt(
        self,
    ) -> crate::common::Reg<regs::Startevtfinecnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0918usize) as _) }
    }
    #[inline(always)]
    pub const fn endevtclkn(self) -> crate::common::Reg<regs::Endevtclkn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x091cusize) as _) }
    }
    #[inline(always)]
    pub const fn endevtfinecnt(self) -> crate::common::Reg<regs::Endevtfinecnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0920usize) as _) }
    }
    #[inline(always)]
    pub const fn skipevtclkn(self) -> crate::common::Reg<regs::Skipevtclkn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0924usize) as _) }
    }
    #[inline(always)]
    pub const fn skipevtfinecnt(
        self,
    ) -> crate::common::Reg<regs::Skipevtfinecnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0928usize) as _) }
    }
    #[inline(always)]
    pub const fn advtim(self) -> crate::common::Reg<regs::Advtim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0930usize) as _) }
    }
    #[inline(always)]
    pub const fn actscancntl(self) -> crate::common::Reg<regs::Actscancntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0934usize) as _) }
    }
    #[inline(always)]
    pub const fn wpalcntl(self) -> crate::common::Reg<regs::Wpalcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[inline(always)]
    pub const fn wpalcurrent(self) -> crate::common::Reg<regs::Wpalcurrent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0944usize) as _) }
    }
    #[inline(always)]
    pub const fn search_timeout(
        self,
    ) -> crate::common::Reg<regs::SearchTimeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0948usize) as _) }
    }
    #[inline(always)]
    pub const fn blecoexifcntl0(
        self,
    ) -> crate::common::Reg<regs::Blecoexifcntl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0950usize) as _) }
    }
    #[inline(always)]
    pub const fn blecoexifcntl1(
        self,
    ) -> crate::common::Reg<regs::Blecoexifcntl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0954usize) as _) }
    }
    #[inline(always)]
    pub const fn blecoexifcntl2(
        self,
    ) -> crate::common::Reg<regs::Blecoexifcntl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0958usize) as _) }
    }
    #[inline(always)]
    pub const fn blemprio0(self) -> crate::common::Reg<regs::Blemprio0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0960usize) as _) }
    }
    #[inline(always)]
    pub const fn blemprio1(self) -> crate::common::Reg<regs::Blemprio1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0964usize) as _) }
    }
    #[inline(always)]
    pub const fn blemprio2(self) -> crate::common::Reg<regs::Blemprio2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0968usize) as _) }
    }
    #[inline(always)]
    pub const fn ralcntl(self) -> crate::common::Reg<regs::Ralcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0970usize) as _) }
    }
    #[inline(always)]
    pub const fn ralcurrent(self) -> crate::common::Reg<regs::Ralcurrent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0974usize) as _) }
    }
    #[inline(always)]
    pub const fn ral_local_rnd(self) -> crate::common::Reg<regs::RalLocalRnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0978usize) as _) }
    }
    #[inline(always)]
    pub const fn ral_peer_rnd(self) -> crate::common::Reg<regs::RalPeerRnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x097cusize) as _) }
    }
    #[inline(always)]
    pub const fn dfcntl0_1us(self) -> crate::common::Reg<regs::Dfcntl01us, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    #[inline(always)]
    pub const fn dfcntl0_2us(self) -> crate::common::Reg<regs::Dfcntl02us, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0984usize) as _) }
    }
    #[inline(always)]
    pub const fn dfcntl1_1us(self) -> crate::common::Reg<regs::Dfcntl11us, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0988usize) as _) }
    }
    #[inline(always)]
    pub const fn dfcntl1_2us(self) -> crate::common::Reg<regs::Dfcntl12us, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x098cusize) as _) }
    }
    #[inline(always)]
    pub const fn dfcurrentptr(self) -> crate::common::Reg<regs::Dfcurrentptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0990usize) as _) }
    }
    #[inline(always)]
    pub const fn dfantcntl(self) -> crate::common::Reg<regs::Dfantcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0994usize) as _) }
    }
    #[inline(always)]
    pub const fn dfifcntl(self) -> crate::common::Reg<regs::Dfifcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0998usize) as _) }
    }
    #[inline(always)]
    pub const fn freqselcntl(self) -> crate::common::Reg<regs::Freqselcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a0usize) as _) }
    }
    #[inline(always)]
    pub const fn freqselptr(self) -> crate::common::Reg<regs::Freqselptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a4usize) as _) }
    }
    #[inline(always)]
    pub const fn freqsel_cs1_seed(
        self,
    ) -> crate::common::Reg<regs::FreqselCs1Seed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a8usize) as _) }
    }
    #[inline(always)]
    pub const fn freq_cs2_seed(self) -> crate::common::Reg<regs::FreqCs2Seed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09acusize) as _) }
    }
    #[inline(always)]
    pub const fn freqsel_llchmap0(
        self,
    ) -> crate::common::Reg<regs::FreqselLlchmap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09b0usize) as _) }
    }
    #[inline(always)]
    pub const fn freqsel_llchmap1(
        self,
    ) -> crate::common::Reg<regs::FreqselLlchmap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09b4usize) as _) }
    }
    #[inline(always)]
    pub const fn isocntcntl(self) -> crate::common::Reg<regs::Isocntcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c0usize) as _) }
    }
    #[inline(always)]
    pub const fn isocntsamp(self) -> crate::common::Reg<regs::Isocntsamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c4usize) as _) }
    }
    #[inline(always)]
    pub const fn isocntcorr(self) -> crate::common::Reg<regs::Isocntcorr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c8usize) as _) }
    }
    #[inline(always)]
    pub const fn isointcorr_hus(
        self,
    ) -> crate::common::Reg<regs::IsointcorrHus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09ccusize) as _) }
    }
    #[inline(always)]
    pub const fn isointcntl(self) -> crate::common::Reg<regs::Isointcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d0usize) as _) }
    }
    #[inline(always)]
    pub const fn isointstat(self) -> crate::common::Reg<regs::Isointstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d4usize) as _) }
    }
    #[inline(always)]
    pub const fn isointack(self) -> crate::common::Reg<regs::Isointack, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d8usize) as _) }
    }
    #[inline(always)]
    pub const fn isogpiocntl(self) -> crate::common::Reg<regs::Isogpiocntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e0usize) as _) }
    }
    #[inline(always)]
    pub const fn isotimertgt0(self) -> crate::common::Reg<regs::Isotimertgt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f0usize) as _) }
    }
    #[inline(always)]
    pub const fn isotimertgt1(self) -> crate::common::Reg<regs::Isotimertgt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f4usize) as _) }
    }
    #[inline(always)]
    pub const fn isotimertgt2(self) -> crate::common::Reg<regs::Isotimertgt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f8usize) as _) }
    }
    #[inline(always)]
    pub const fn isotimertgt3(self) -> crate::common::Reg<regs::Isotimertgt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09fcusize) as _) }
    }
    #[inline(always)]
    pub const fn isotimertgt4(self) -> crate::common::Reg<regs::Isotimertgt4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a00usize) as _) }
    }
    #[inline(always)]
    pub const fn isotimertgt5(self) -> crate::common::Reg<regs::Isotimertgt5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a04usize) as _) }
    }
    #[inline(always)]
    pub const fn isotimertgt6(self) -> crate::common::Reg<regs::Isotimertgt6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a08usize) as _) }
    }
    #[inline(always)]
    pub const fn isotimertgt7(self) -> crate::common::Reg<regs::Isotimertgt7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a0cusize) as _) }
    }
}
pub mod regs;
