#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "46 - LPTIM1"]
    LPTIM1 = 46,
    #[doc = "47 - LPTIM2"]
    LPTIM2 = 47,
    #[doc = "48 - PMUC"]
    PMUC = 48,
    #[doc = "49 - RTC"]
    RTC = 49,
    #[doc = "50 - DMAC1_CH1"]
    DMAC1_CH1 = 50,
    #[doc = "51 - DMAC1_CH2"]
    DMAC1_CH2 = 51,
    #[doc = "52 - DMAC1_CH3"]
    DMAC1_CH3 = 52,
    #[doc = "53 - DMAC1_CH4"]
    DMAC1_CH4 = 53,
    #[doc = "54 - DMAC1_CH5"]
    DMAC1_CH5 = 54,
    #[doc = "55 - DMAC1_CH6"]
    DMAC1_CH6 = 55,
    #[doc = "56 - DMAC1_CH7"]
    DMAC1_CH7 = 56,
    #[doc = "57 - DMAC1_CH8"]
    DMAC1_CH8 = 57,
    #[doc = "58 - MAILBOX2_CH1"]
    MAILBOX2_CH1 = 58,
    #[doc = "59 - USART1"]
    USART1 = 59,
    #[doc = "60 - SPI1"]
    SPI1 = 60,
    #[doc = "61 - I2C1"]
    I2C1 = 61,
    #[doc = "62 - EPIC"]
    EPIC = 62,
    #[doc = "63 - LCDC1"]
    LCDC1 = 63,
    #[doc = "64 - I2S1"]
    I2S1 = 64,
    #[doc = "65 - GPADC"]
    GPADC = 65,
    #[doc = "66 - EFUSEC"]
    EFUSEC = 66,
    #[doc = "67 - AES"]
    AES = 67,
    #[doc = "68 - PTC1"]
    PTC1 = 68,
    #[doc = "69 - TRNG"]
    TRNG = 69,
    #[doc = "70 - GPTIM1"]
    GPTIM1 = 70,
    #[doc = "71 - GPTIM2"]
    GPTIM2 = 71,
    #[doc = "72 - BTIM1"]
    BTIM1 = 72,
    #[doc = "73 - BTIM2"]
    BTIM2 = 73,
    #[doc = "74 - USART2"]
    USART2 = 74,
    #[doc = "75 - SPI2"]
    SPI2 = 75,
    #[doc = "76 - I2C2"]
    I2C2 = 76,
    #[doc = "77 - EXTDMA"]
    EXTDMA = 77,
    #[doc = "78 - I2C4"]
    I2C4 = 78,
    #[doc = "79 - SDMMC1"]
    SDMMC1 = 79,
    #[doc = "80 - MAILBOX2_CH2"]
    MAILBOX2_CH2 = 80,
    #[doc = "82 - PDM1"]
    PDM1 = 82,
    #[doc = "84 - GPIO1"]
    GPIO1 = 84,
    #[doc = "85 - MPI1"]
    MPI1 = 85,
    #[doc = "86 - MPI2"]
    MPI2 = 86,
    #[doc = "89 - EZIP1"]
    EZIP1 = 89,
    #[doc = "90 - AUDPRC"]
    AUDPRC = 90,
    #[doc = "91 - TSEN"]
    TSEN = 91,
    #[doc = "92 - USBC"]
    USBC = 92,
    #[doc = "93 - I2C3"]
    I2C3 = 93,
    #[doc = "94 - ATIM1"]
    ATIM1 = 94,
    #[doc = "95 - USART3"]
    USART3 = 95,
    #[doc = "96 - AUD_HP"]
    AUD_HP = 96,
    #[doc = "98 - SECU1"]
    SECU1 = 98,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
pub const LPSYS_RCC: lpsys_rcc::LpsysRcc =
    unsafe { lpsys_rcc::LpsysRcc::from_ptr(0x4000_0000usize as _) };
pub const DMAC2: dmac::Dmac = unsafe { dmac::Dmac::from_ptr(0x4000_1000usize as _) };
pub const MAILBOX2: mailbox::Mailbox2 =
    unsafe { mailbox::Mailbox2::from_ptr(0x4000_2000usize as _) };
pub const PATCH: patch::Patch = unsafe { patch::Patch::from_ptr(0x4000_4000usize as _) };
pub const PTC2: ptc::Ptc = unsafe { ptc::Ptc::from_ptr(0x4000_c000usize as _) };
pub const LPSYS_CFG: lpsys_cfg::LpsysCfg =
    unsafe { lpsys_cfg::LpsysCfg::from_ptr(0x4000_f000usize as _) };
pub const LPSYS_AON: lpsys_aon::LpsysAon =
    unsafe { lpsys_aon::LpsysAon::from_ptr(0x4004_0000usize as _) };
pub const BT_RFC: bt_rfc::BtRfc = unsafe { bt_rfc::BtRfc::from_ptr(0x4008_2800usize as _) };
pub const BT_PHY: bt_phy::BtPhy = unsafe { bt_phy::BtPhy::from_ptr(0x4008_4000usize as _) };
pub const BT_MAC: bt_mac::BtMac = unsafe { bt_mac::BtMac::from_ptr(0x4009_0000usize as _) };
pub const HPSYS_RCC: hpsys_rcc::HpsysRcc =
    unsafe { hpsys_rcc::HpsysRcc::from_ptr(0x5000_0000usize as _) };
pub const EXTDMA: extdma::Extdma = unsafe { extdma::Extdma::from_ptr(0x5000_1000usize as _) };
pub const HPSYS_PINMUX: hpsys_pinmux::HpsysPinmux =
    unsafe { hpsys_pinmux::HpsysPinmux::from_ptr(0x5000_3000usize as _) };
pub const ATIM1: atim::Atim = unsafe { atim::Atim::from_ptr(0x5000_4000usize as _) };
pub const AUDPRC: audprc::Audprc = unsafe { audprc::Audprc::from_ptr(0x5000_5000usize as _) };
pub const EZIP1: ezip::Ezip = unsafe { ezip::Ezip::from_ptr(0x5000_6000usize as _) };
pub const EPIC: epic::Epic = unsafe { epic::Epic::from_ptr(0x5000_7000usize as _) };
pub const LCDC1: lcdc::Lcdc = unsafe { lcdc::Lcdc::from_ptr(0x5000_8000usize as _) };
pub const I2S1: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x5000_9000usize as _) };
pub const HPSYS_CFG: hpsys_cfg::HpsysCfg =
    unsafe { hpsys_cfg::HpsysCfg::from_ptr(0x5000_b000usize as _) };
pub const EFUSEC: efusec::Efusec = unsafe { efusec::Efusec::from_ptr(0x5000_c000usize as _) };
pub const AES: aes::Aes = unsafe { aes::Aes::from_ptr(0x5000_d000usize as _) };
pub const TRNG: trng::Trng = unsafe { trng::Trng::from_ptr(0x5000_f000usize as _) };
pub const MPI1: mpi::Mpi = unsafe { mpi::Mpi::from_ptr(0x5004_1000usize as _) };
pub const MPI2: mpi::Mpi = unsafe { mpi::Mpi::from_ptr(0x5004_2000usize as _) };
pub const SDMMC1: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x5004_5000usize as _) };
pub const CRC1: crc::Crc = unsafe { crc::Crc::from_ptr(0x5004_8000usize as _) };
pub const PTC1: ptc::Ptc = unsafe { ptc::Ptc::from_ptr(0x5008_0000usize as _) };
pub const DMAC1: dmac::Dmac = unsafe { dmac::Dmac::from_ptr(0x5008_1000usize as _) };
pub const MAILBOX1: mailbox::Mailbox1 =
    unsafe { mailbox::Mailbox1::from_ptr(0x5008_2000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x5008_4000usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x5008_5000usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x5008_6000usize as _) };
pub const GPADC: gpadc::Gpadc = unsafe { gpadc::Gpadc::from_ptr(0x5008_7000usize as _) };
pub const AUDCODEC: audcodec::Audcodec =
    unsafe { audcodec::Audcodec::from_ptr(0x5008_8000usize as _) };
pub const TSEN: tsen::Tsen = unsafe { tsen::Tsen::from_ptr(0x5008_9000usize as _) };
pub const GPTIM1: gptim::Gptim = unsafe { gptim::Gptim::from_ptr(0x5009_0000usize as _) };
pub const BTIM1: btim::Btim = unsafe { btim::Btim::from_ptr(0x5009_2000usize as _) };
pub const WDT1: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x5009_4000usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x5009_5000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x5009_6000usize as _) };
pub const PDM1: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0x5009_a000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x5009_c000usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x5009_d000usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x5009_e000usize as _) };
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x5009_f000usize as _) };
pub const HPSYS_GPIO: hpsys_gpio::HpsysGpio =
    unsafe { hpsys_gpio::HpsysGpio::from_ptr(0x500a_0000usize as _) };
pub const GPTIM2: gptim::Gptim = unsafe { gptim::Gptim::from_ptr(0x500b_0000usize as _) };
pub const BTIM2: btim::Btim = unsafe { btim::Btim::from_ptr(0x500b_1000usize as _) };
pub const HPSYS_AON: hpsys_aon::HpsysAon =
    unsafe { hpsys_aon::HpsysAon::from_ptr(0x500c_0000usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x500c_1000usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x500c_2000usize as _) };
pub const PMUC: pmuc::Pmuc = unsafe { pmuc::Pmuc::from_ptr(0x500c_a000usize as _) };
pub const IWDT: iwdt::Iwdt = unsafe { iwdt::Iwdt::from_ptr(0x500c_c000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod aes;
pub mod atim;
pub mod audcodec;
pub mod audprc;
pub mod bt_mac;
pub mod bt_phy;
pub mod bt_rfc;
pub mod btim;
pub mod common;
pub mod crc;
pub mod dmac;
pub mod efusec;
pub mod epic;
pub mod extdma;
pub mod ezip;
pub mod gpadc;
pub mod gptim;
pub mod hpsys_aon;
pub mod hpsys_cfg;
pub mod hpsys_gpio;
pub mod hpsys_pinmux;
pub mod hpsys_rcc;
pub mod i2c;
pub mod i2s;
pub mod iwdt;
pub mod lcdc;
pub mod lpsys_aon;
pub mod lpsys_cfg;
pub mod lpsys_pinmux;
pub mod lpsys_rcc;
pub mod lptim;
pub mod mailbox;
pub mod mpi;
pub mod patch;
pub mod pdm;
pub mod pmuc;
pub mod ptc;
pub mod sdmmc;
pub mod spi;
pub mod tim_common;
pub mod trng;
pub mod tsen;
pub mod usart;
pub mod wdt;
