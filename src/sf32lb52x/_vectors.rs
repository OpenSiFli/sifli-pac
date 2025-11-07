unsafe extern "C" {
    fn LPTIM1();
    fn LPTIM2();
    fn PMUC();
    fn RTC();
    fn DMAC1_CH1();
    fn DMAC1_CH2();
    fn DMAC1_CH3();
    fn DMAC1_CH4();
    fn DMAC1_CH5();
    fn DMAC1_CH6();
    fn DMAC1_CH7();
    fn DMAC1_CH8();
    fn MAILBOX2_CH1();
    fn USART1();
    fn SPI1();
    fn I2C1();
    fn EPIC();
    fn LCDC1();
    fn I2S1();
    fn GPADC();
    fn EFUSEC();
    fn AES();
    fn PTC1();
    fn TRNG();
    fn GPTIM1();
    fn GPTIM2();
    fn BTIM1();
    fn BTIM2();
    fn USART2();
    fn SPI2();
    fn I2C2();
    fn EXTDMA();
    fn I2C4();
    fn SDMMC1();
    fn MAILBOX2_CH2();
    fn PDM1();
    fn GPIO1();
    fn MPI1();
    fn MPI2();
    fn EZIP1();
    fn AUDPRC();
    fn TSEN();
    fn USBC();
    fn I2C3();
    fn ATIM1();
    fn USART3();
    fn AUD_HP();
    fn SECU1();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 99] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: PMUC },
    Vector { _handler: RTC },
    Vector {
        _handler: DMAC1_CH1,
    },
    Vector {
        _handler: DMAC1_CH2,
    },
    Vector {
        _handler: DMAC1_CH3,
    },
    Vector {
        _handler: DMAC1_CH4,
    },
    Vector {
        _handler: DMAC1_CH5,
    },
    Vector {
        _handler: DMAC1_CH6,
    },
    Vector {
        _handler: DMAC1_CH7,
    },
    Vector {
        _handler: DMAC1_CH8,
    },
    Vector {
        _handler: MAILBOX2_CH1,
    },
    Vector { _handler: USART1 },
    Vector { _handler: SPI1 },
    Vector { _handler: I2C1 },
    Vector { _handler: EPIC },
    Vector { _handler: LCDC1 },
    Vector { _handler: I2S1 },
    Vector { _handler: GPADC },
    Vector { _handler: EFUSEC },
    Vector { _handler: AES },
    Vector { _handler: PTC1 },
    Vector { _handler: TRNG },
    Vector { _handler: GPTIM1 },
    Vector { _handler: GPTIM2 },
    Vector { _handler: BTIM1 },
    Vector { _handler: BTIM2 },
    Vector { _handler: USART2 },
    Vector { _handler: SPI2 },
    Vector { _handler: I2C2 },
    Vector { _handler: EXTDMA },
    Vector { _handler: I2C4 },
    Vector { _handler: SDMMC1 },
    Vector {
        _handler: MAILBOX2_CH2,
    },
    Vector { _reserved: 0 },
    Vector { _handler: PDM1 },
    Vector { _reserved: 0 },
    Vector { _handler: GPIO1 },
    Vector { _handler: MPI1 },
    Vector { _handler: MPI2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EZIP1 },
    Vector { _handler: AUDPRC },
    Vector { _handler: TSEN },
    Vector { _handler: USBC },
    Vector { _handler: I2C3 },
    Vector { _handler: ATIM1 },
    Vector { _handler: USART3 },
    Vector { _handler: AUD_HP },
    Vector { _reserved: 0 },
    Vector { _handler: SECU1 },
];
