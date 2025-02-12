#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdc {
    ptr: *mut u8,
}
unsafe impl Send for Lcdc {}
unsafe impl Sync for Lcdc {}
impl Lcdc {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn irq(self) -> crate::common::Reg<regs::Irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn setting(self) -> crate::common::Reg<regs::Setting, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn canvas_tl_pos(self) -> crate::common::Reg<regs::CanvasTlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn canvas_br_pos(self) -> crate::common::Reg<regs::CanvasBrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn canvas_bg(self) -> crate::common::Reg<regs::CanvasBg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_config(self) -> crate::common::Reg<regs::Layer0Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_tl_pos(self) -> crate::common::Reg<regs::Layer0TlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_br_pos(self) -> crate::common::Reg<regs::Layer0BrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_filter(self) -> crate::common::Reg<regs::Layer0Filter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_src(self) -> crate::common::Reg<regs::Layer0Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_fill(self) -> crate::common::Reg<regs::Layer0Fill, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_decomp(self) -> crate::common::Reg<regs::Layer0Decomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_decomp_cfg0(
        self,
    ) -> crate::common::Reg<regs::Layer0DecompCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_decomp_cfg1(
        self,
    ) -> crate::common::Reg<regs::Layer0DecompCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn layer0_decomp_stat(
        self,
    ) -> crate::common::Reg<regs::Layer0DecompStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn layer1_config(self) -> crate::common::Reg<regs::Layer1Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn layer1_tl_pos(self) -> crate::common::Reg<regs::Layer1TlPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn layer1_br_pos(self) -> crate::common::Reg<regs::Layer1BrPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn layer1_filter(self) -> crate::common::Reg<regs::Layer1Filter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn layer1_src(self) -> crate::common::Reg<regs::Layer1Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn layer1_fill(self) -> crate::common::Reg<regs::Layer1Fill, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn dither_conf(self) -> crate::common::Reg<regs::DitherConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn dither_lfsr(self) -> crate::common::Reg<regs::DitherLfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn lcd_conf(self) -> crate::common::Reg<regs::LcdConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn lcd_if_conf(self) -> crate::common::Reg<regs::LcdIfConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn lcd_mem(self) -> crate::common::Reg<regs::LcdMem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn lcd_o_width(self) -> crate::common::Reg<regs::LcdOWidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn lcd_single(self) -> crate::common::Reg<regs::LcdSingle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn lcd_wr(self) -> crate::common::Reg<regs::LcdWr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn lcd_rd(self) -> crate::common::Reg<regs::LcdRd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn spi_if_conf(self) -> crate::common::Reg<regs::SpiIfConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn te_conf(self) -> crate::common::Reg<regs::TeConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn te_conf2(self) -> crate::common::Reg<regs::TeConf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn dpi_if_conf1(self) -> crate::common::Reg<regs::DpiIfConf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn dpi_if_conf2(self) -> crate::common::Reg<regs::DpiIfConf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn dpi_if_conf3(self) -> crate::common::Reg<regs::DpiIfConf3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn dpi_if_conf4(self) -> crate::common::Reg<regs::DpiIfConf4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn dpi_if_conf5(self) -> crate::common::Reg<regs::DpiIfConf5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn dpi_ctrl(self) -> crate::common::Reg<regs::DpiCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn dpi_stat(self) -> crate::common::Reg<regs::DpiStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_ser_conf1(self) -> crate::common::Reg<regs::JdiSerConf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_ser_conf2(self) -> crate::common::Reg<regs::JdiSerConf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_ser_ctrl(self) -> crate::common::Reg<regs::JdiSerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf1(self) -> crate::common::Reg<regs::JdiParConf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf2(self) -> crate::common::Reg<regs::JdiParConf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf3(self) -> crate::common::Reg<regs::JdiParConf3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf4(self) -> crate::common::Reg<regs::JdiParConf4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf5(self) -> crate::common::Reg<regs::JdiParConf5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf6(self) -> crate::common::Reg<regs::JdiParConf6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf7(self) -> crate::common::Reg<regs::JdiParConf7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_ctrl(self) -> crate::common::Reg<regs::JdiParCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_stat(self) -> crate::common::Reg<regs::JdiParStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_ex_ctrl(
        self,
    ) -> crate::common::Reg<regs::JdiParExCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf8(self) -> crate::common::Reg<regs::JdiParConf8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf9(self) -> crate::common::Reg<regs::JdiParConf9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn jdi_par_conf10(self) -> crate::common::Reg<regs::JdiParConf10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn canvas_stat0(self) -> crate::common::Reg<regs::CanvasStat0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn canvas_stat1(self) -> crate::common::Reg<regs::CanvasStat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn ol0_stat(self) -> crate::common::Reg<regs::Ol0Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn ol1_stat(self) -> crate::common::Reg<regs::Ol1Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn mem_if_stat(self) -> crate::common::Reg<regs::MemIfStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn perf_cnt(self) -> crate::common::Reg<regs::PerfCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
}
pub mod regs;
