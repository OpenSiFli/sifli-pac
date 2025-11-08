#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epic {
    ptr: *mut u8,
}
unsafe impl Send for Epic {}
unsafe impl Sync for Epic {}
impl Epic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn command(self) -> crate::common::Reg<regs::Command, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn eof_irq(self) -> crate::common::Reg<regs::EofIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn setting(self) -> crate::common::Reg<regs::Setting, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Top-Left pixel coordinate"]
    #[inline(always)]
    pub const fn canvas_tl_pos(self) -> crate::common::Reg<regs::CanvasTlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Bottom-Right pixel coordinate"]
    #[inline(always)]
    pub const fn canvas_br_pos(self) -> crate::common::Reg<regs::CanvasBrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Background color"]
    #[inline(always)]
    pub const fn canvas_bg(self) -> crate::common::Reg<regs::CanvasBg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_cfg(self) -> crate::common::Reg<regs::VlCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn vl_tl_pos(self) -> crate::common::Reg<regs::VlTlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_br_pos(self) -> crate::common::Reg<regs::VlBrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_extents(self) -> crate::common::Reg<regs::VlExtents, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_filter(self) -> crate::common::Reg<regs::VlFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn vl_src(self) -> crate::common::Reg<regs::VlSrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_rot(self) -> crate::common::Reg<regs::VlRot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_rot_stat(self) -> crate::common::Reg<regs::VlRotStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_scale_ratio_h(
        self,
    ) -> crate::common::Reg<regs::VlScaleRatioH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn vl_scale_ratio_v(
        self,
    ) -> crate::common::Reg<regs::VlScaleRatioV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_fill(self) -> crate::common::Reg<regs::VlFill, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_misc_cfg(self) -> crate::common::Reg<regs::VlMiscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn l0_cfg(self) -> crate::common::Reg<regs::L0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn l0_tl_pos(self) -> crate::common::Reg<regs::L0TlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn l0_br_pos(self) -> crate::common::Reg<regs::L0BrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn l0_filter(self) -> crate::common::Reg<regs::L0Filter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn l0_src(self) -> crate::common::Reg<regs::L0Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn l0_fill(self) -> crate::common::Reg<regs::L0Fill, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn l0_misc_cfg(self) -> crate::common::Reg<regs::L0MiscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn l1_cfg(self) -> crate::common::Reg<regs::L1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn l1_tl_pos(self) -> crate::common::Reg<regs::L1TlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn l1_br_pos(self) -> crate::common::Reg<regs::L1BrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn l1_filter(self) -> crate::common::Reg<regs::L1Filter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn l1_src(self) -> crate::common::Reg<regs::L1Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn l1_fill(self) -> crate::common::Reg<regs::L1Fill, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn l1_misc_cfg(self) -> crate::common::Reg<regs::L1MiscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_cfg(self) -> crate::common::Reg<regs::L2Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_tl_pos(self) -> crate::common::Reg<regs::L2TlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_br_pos(self) -> crate::common::Reg<regs::L2BrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_extents(self) -> crate::common::Reg<regs::L2Extents, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn l2_filter(self) -> crate::common::Reg<regs::L2Filter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_src(self) -> crate::common::Reg<regs::L2Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_rot(self) -> crate::common::Reg<regs::L2Rot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_rot_stat(self) -> crate::common::Reg<regs::L2RotStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn l2_scale_ratio_h(
        self,
    ) -> crate::common::Reg<regs::L2ScaleRatioH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_scale_ratio_v(
        self,
    ) -> crate::common::Reg<regs::L2ScaleRatioV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_fill(self) -> crate::common::Reg<regs::L2Fill, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_misc_cfg(self) -> crate::common::Reg<regs::L2MiscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn mask_cfg(self) -> crate::common::Reg<regs::MaskCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn mask_tl_pos(self) -> crate::common::Reg<regs::MaskTlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn mask_br_pos(self) -> crate::common::Reg<regs::MaskBrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn mask_src(self) -> crate::common::Reg<regs::MaskSrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn coeng_cfg(self) -> crate::common::Reg<regs::CoengCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn yuv_eng_cfg0(self) -> crate::common::Reg<regs::YuvEngCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn yuv_eng_cfg1(self) -> crate::common::Reg<regs::YuvEngCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn y_src(self) -> crate::common::Reg<regs::YSrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn u_src(self) -> crate::common::Reg<regs::USrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn v_src(self) -> crate::common::Reg<regs::VSrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn coef0(self) -> crate::common::Reg<regs::Coef0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn coef1(self) -> crate::common::Reg<regs::Coef1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn dither_conf(self) -> crate::common::Reg<regs::DitherConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn dither_lfsr(self) -> crate::common::Reg<regs::DitherLfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn ahb_ctrl(self) -> crate::common::Reg<regs::AhbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn ahb_mem(self) -> crate::common::Reg<regs::AhbMem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn ahb_stride(self) -> crate::common::Reg<regs::AhbStride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn debug(self) -> crate::common::Reg<regs::Debug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_rot_m_cfg1(self) -> crate::common::Reg<regs::VlRotMCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_rot_m_cfg2(self) -> crate::common::Reg<regs::VlRotMCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn vl_rot_m_cfg3(self) -> crate::common::Reg<regs::VlRotMCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_scale_init_cfg1(
        self,
    ) -> crate::common::Reg<regs::VlScaleInitCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn vl_scale_init_cfg2(
        self,
    ) -> crate::common::Reg<regs::VlScaleInitCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_rot_m_cfg1(self) -> crate::common::Reg<regs::L2RotMCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn l2_rot_m_cfg2(self) -> crate::common::Reg<regs::L2RotMCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_rot_m_cfg3(self) -> crate::common::Reg<regs::L2RotMCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_scale_init_cfg1(
        self,
    ) -> crate::common::Reg<regs::L2ScaleInitCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn l2_scale_init_cfg2(
        self,
    ) -> crate::common::Reg<regs::L2ScaleInitCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[inline(always)]
    pub const fn perf_cnt(self) -> crate::common::Reg<regs::PerfCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn canvas_stat(self) -> crate::common::Reg<regs::CanvasStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn ezip_stat(self) -> crate::common::Reg<regs::EzipStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn ol_stat(self) -> crate::common::Reg<regs::OlStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn ol2_stat(self) -> crate::common::Reg<regs::Ol2Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[inline(always)]
    pub const fn vl_stat(self) -> crate::common::Reg<regs::VlStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn ml_stat(self) -> crate::common::Reg<regs::MlStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn mem_if_stat(self) -> crate::common::Reg<regs::MemIfStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
}
pub mod regs;
