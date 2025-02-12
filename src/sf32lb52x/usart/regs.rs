#[doc = "Baud Rate Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc = "Fractional part of baud rate prescaler"]
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Fractional part of baud rate prescaler"]
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Integer part of baud rate prescaler If OVER8 = 0, Baud Rate = 48000000 / (INT + FRAC/16) / 16 If OVER8 = 1, Baud Rate = 48000000 / (INT + FRAC/16) / 8 For example: OVER=0, INT=3, FRAC=0, Baud Rate = 48000000/(3+0)/16 = 1Mbps OVER=0, INT=3, FRAC=4, Baud Rate = 48000000/(3+4/16)/16 = 923077 = 921600 + 1.6per mille OVER=1, INT=52, FRAC=1, Baud Rate = 48000000/(52+1/16)/8 = 115246 = 115200 + 0.4per mille"]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Integer part of baud rate prescaler If OVER8 = 0, Baud Rate = 48000000 / (INT + FRAC/16) / 16 If OVER8 = 1, Baud Rate = 48000000 / (INT + FRAC/16) / 8 For example: OVER=0, INT=3, FRAC=0, Baud Rate = 48000000/(3+0)/16 = 1Mbps OVER=0, INT=3, FRAC=4, Baud Rate = 48000000/(3+4/16)/16 = 923077 = 921600 + 1.6per mille OVER=1, INT=52, FRAC=1, Baud Rate = 48000000/(52+1/16)/8 = 115246 = 115200 + 0.4per mille"]
    #[inline(always)]
    pub fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
}
impl Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        Brr(0)
    }
}
impl core::fmt::Debug for Brr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brr")
            .field("frac", &self.frac())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Brr {
            frac: u8,
            int: u16,
        }
        let proxy = Brr {
            frac: self.frac(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "USART enable 0: disabled 1: enabled"]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART enable 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receiver enable 0: receiver is disabled 1: receiver is enabled"]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver enable 0: receiver is disabled 1: receiver is enabled"]
    #[inline(always)]
    pub fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmitter enable 0: transmitter is disabled 1: transmitter is enabled"]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter enable 0: transmitter is disabled 1: transmitter is enabled"]
    #[inline(always)]
    pub fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Idle line interrupt enable 0: interrupt disabled 1: interrupt is generated whenever IDLE=1 in the ISR register"]
    #[inline(always)]
    pub const fn idleie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Idle line interrupt enable 0: interrupt disabled 1: interrupt is generated whenever IDLE=1 in the ISR register"]
    #[inline(always)]
    pub fn set_idleie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx not empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenever RXNE=1 in the ISR register"]
    #[inline(always)]
    pub const fn rxneie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx not empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenever RXNE=1 in the ISR register"]
    #[inline(always)]
    pub fn set_rxneie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer compelete interrupt enable 0: interrupt disabled 1: interrupt is generated whenever TC=1 in the ISR register"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer compelete interrupt enable 0: interrupt disabled 1: interrupt is generated whenever TC=1 in the ISR register"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tx empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenver TXE=1 in the ISR register"]
    #[inline(always)]
    pub const fn txeie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tx empty interrupt enable 0: interrupt disabled 1: interrupt is generated whenver TXE=1 in the ISR register"]
    #[inline(always)]
    pub fn set_txeie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Parity error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever PE=1 in the ISR register"]
    #[inline(always)]
    pub const fn peie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever PE=1 in the ISR register"]
    #[inline(always)]
    pub fn set_peie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Parity select 0: even parity 1: odd parity"]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Parity select 0: even parity 1: odd parity"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Parity check enable. If enabled, parity bit is inserted at the MSB position 0: parity check disabled 1: parity check enabled"]
    #[inline(always)]
    pub const fn pce(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Parity check enable. If enabled, parity bit is inserted at the MSB position 0: parity check disabled 1: parity check enabled"]
    #[inline(always)]
    pub fn set_pce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Oversampling mode 0: Oversampling by 16 1: Oversampling by 8"]
    #[inline(always)]
    pub const fn over8(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Oversampling mode 0: Oversampling by 16 1: Oversampling by 8"]
    #[inline(always)]
    pub fn set_over8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Mode bit indicates the length of the packet, including data bits and parity. Stop bits not included. 0: 6 bits (e.g. 6 data bits + no parity bit) 1: 7 bits (e.g. 6 data bits + 1 parity bit) 2: 8 bits (e.g. 7 data bits + 1 parity bit, or 6 data bits + 2 parity bits) 3: 9 bits (e.g. 8 data bits + 1 parity bit, or 7 data bits + 2 parity bits)"]
    #[inline(always)]
    pub const fn m(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "Mode bit indicates the length of the packet, including data bits and parity. Stop bits not included. 0: 6 bits (e.g. 6 data bits + no parity bit) 1: 7 bits (e.g. 6 data bits + 1 parity bit) 2: 8 bits (e.g. 7 data bits + 1 parity bit, or 6 data bits + 2 parity bits) 3: 9 bits (e.g. 8 data bits + 1 parity bit, or 7 data bits + 2 parity bits)"]
    #[inline(always)]
    pub fn set_m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("ue", &self.ue())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("idleie", &self.idleie())
            .field("rxneie", &self.rxneie())
            .field("tcie", &self.tcie())
            .field("txeie", &self.txeie())
            .field("peie", &self.peie())
            .field("ps", &self.ps())
            .field("pce", &self.pce())
            .field("over8", &self.over8())
            .field("m", &self.m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr1 {
            ue: bool,
            re: bool,
            te: bool,
            idleie: bool,
            rxneie: bool,
            tcie: bool,
            txeie: bool,
            peie: bool,
            ps: bool,
            pce: bool,
            over8: bool,
            m: u8,
        }
        let proxy = Cr1 {
            ue: self.ue(),
            re: self.re(),
            te: self.te(),
            idleie: self.idleie(),
            rxneie: self.rxneie(),
            tcie: self.tcie(),
            txeie: self.txeie(),
            peie: self.peie(),
            ps: self.ps(),
            pce: self.pce(),
            over8: self.over8(),
            m: self.m(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Stop bits 0/1: 1 stop bit 2/3: 2 stop bits"]
    #[inline(always)]
    pub const fn stop(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Stop bits 0/1: 1 stop bit 2/3: 2 stop bits"]
    #[inline(always)]
    pub fn set_stop(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
impl core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr2").field("stop", &self.stop()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr2 {
            stop: u8,
        }
        let proxy = Cr2 { stop: self.stop() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc = "Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register"]
    #[inline(always)]
    pub const fn eie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt enable 0: interrupt disabled 1: interrupt is generated whenever FE=1 or ORE=1 or NF=1 in the ISR register"]
    #[inline(always)]
    pub fn set_eie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receiver DMA enable 0: DMA mode disabled for reception 1: DMA mode enabled for reception"]
    #[inline(always)]
    pub const fn dmar(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver DMA enable 0: DMA mode disabled for reception 1: DMA mode enabled for reception"]
    #[inline(always)]
    pub fn set_dmar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmitter DMA enable 0: DMA mode disabled for transmission 1: DMA mode enabled for transmission"]
    #[inline(always)]
    pub const fn dmat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter DMA enable 0: DMA mode disabled for transmission 1: DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn set_dmat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RTS enable 0: RTS hardware flow control disabled 1: RTS hardware flow control enabled, RTS output is asserted low when new data can be received"]
    #[inline(always)]
    pub const fn rtse(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RTS enable 0: RTS hardware flow control disabled 1: RTS hardware flow control enabled, RTS output is asserted low when new data can be received"]
    #[inline(always)]
    pub fn set_rtse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTS enable 0: CTS hardware flow control disabled 1: CTS hardware flow control enabled, data is transmitted only when CTS input is asserted low"]
    #[inline(always)]
    pub const fn ctse(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS enable 0: CTS hardware flow control disabled 1: CTS hardware flow control enabled, data is transmitted only when CTS input is asserted low"]
    #[inline(always)]
    pub fn set_ctse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CTS interrupt enable 0: interrupt disabled 1: interrupt is generated whenever CTSIF=1 in the ISR register"]
    #[inline(always)]
    pub const fn ctsie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CTS interrupt enable 0: interrupt disabled 1: interrupt is generated whenever CTSIF=1 in the ISR register"]
    #[inline(always)]
    pub fn set_ctsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "One bit sampling mode 0: 3-bit sampling mode, the sampling value is determined by the voted result out of 3 bits 1: 1-bit sampling mode"]
    #[inline(always)]
    pub const fn onebit(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "One bit sampling mode 0: 3-bit sampling mode, the sampling value is determined by the voted result out of 3 bits 1: 1-bit sampling mode"]
    #[inline(always)]
    pub fn set_onebit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Overrun disable 0: overrun error flag (ORE) will be set if new data received but previous data not read. New data will not overwrite the content in RDR register. 1: overrun disabled. If new data is received before previous data is read, the new data will overwrite the content in RDR register and ORE flag remains unset."]
    #[inline(always)]
    pub const fn ovrdis(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun disable 0: overrun error flag (ORE) will be set if new data received but previous data not read. New data will not overwrite the content in RDR register. 1: overrun disabled. If new data is received before previous data is read, the new data will overwrite the content in RDR register and ORE flag remains unset."]
    #[inline(always)]
    pub fn set_ovrdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Cr3 {
    #[inline(always)]
    fn default() -> Cr3 {
        Cr3(0)
    }
}
impl core::fmt::Debug for Cr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr3")
            .field("eie", &self.eie())
            .field("dmar", &self.dmar())
            .field("dmat", &self.dmat())
            .field("rtse", &self.rtse())
            .field("ctse", &self.ctse())
            .field("ctsie", &self.ctsie())
            .field("onebit", &self.onebit())
            .field("ovrdis", &self.ovrdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr3 {
            eie: bool,
            dmar: bool,
            dmat: bool,
            rtse: bool,
            ctse: bool,
            ctsie: bool,
            onebit: bool,
            ovrdis: bool,
        }
        let proxy = Cr3 {
            eie: self.eie(),
            dmar: self.dmar(),
            dmat: self.dmat(),
            rtse: self.rtse(),
            ctse: self.ctse(),
            ctsie: self.ctsie(),
            onebit: self.onebit(),
            ovrdis: self.ovrdis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Debug Receive Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Drdr(pub u32);
impl Drdr {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Drdr {
    #[inline(always)]
    fn default() -> Drdr {
        Drdr(0)
    }
}
impl core::fmt::Debug for Drdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Drdr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Drdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Drdr {
            data: u32,
        }
        let proxy = Drdr { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Debug Receive Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtdr(pub u32);
impl Dtdr {
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dtdr {
    #[inline(always)]
    fn default() -> Dtdr {
        Dtdr(0)
    }
}
impl core::fmt::Debug for Dtdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtdr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dtdr {
            data: u32,
        }
        let proxy = Dtdr { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Mutual Exclusive Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exr(pub u32);
impl Exr {
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn id(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Exr {
    #[inline(always)]
    fn default() -> Exr {
        Exr(0)
    }
}
impl core::fmt::Debug for Exr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Exr")
            .field("busy", &self.busy())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Exr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Exr {
            busy: bool,
            id: bool,
        }
        let proxy = Exr {
            busy: self.busy(),
            id: self.id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt flag Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Parity error clear flag. Wriring 1 to this bit clears the PE flag in the ISR register."]
    #[inline(always)]
    pub const fn pecf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error clear flag. Wriring 1 to this bit clears the PE flag in the ISR register."]
    #[inline(always)]
    pub fn set_pecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Framing error clear flag. Writing 1 to this bit clears the FE flag in the ISR register."]
    #[inline(always)]
    pub const fn fecf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error clear flag. Writing 1 to this bit clears the FE flag in the ISR register."]
    #[inline(always)]
    pub fn set_fecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Noise detected clear flag. Writing 1 to this bit clears the NF flag in the ISR register."]
    #[inline(always)]
    pub const fn ncf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Noise detected clear flag. Writing 1 to this bit clears the NF flag in the ISR register."]
    #[inline(always)]
    pub fn set_ncf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun error clear flag. Writing 1 to this bit clears the ORE flag in the ISR register."]
    #[inline(always)]
    pub const fn orecf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error clear flag. Writing 1 to this bit clears the ORE flag in the ISR register."]
    #[inline(always)]
    pub fn set_orecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Idle line detected clear flag. Writing 1 to this bit clears the IDLECF flag in the ISR register."]
    #[inline(always)]
    pub const fn idlecf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Idle line detected clear flag. Writing 1 to this bit clears the IDLECF flag in the ISR register."]
    #[inline(always)]
    pub fn set_idlecf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register."]
    #[inline(always)]
    pub const fn tccf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete clear flag. Writing 1 to this bit clears the TC flag in the ISR register."]
    #[inline(always)]
    pub fn set_tccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register."]
    #[inline(always)]
    pub const fn ctscf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS clear flag. Writing 1 to this bit clears the CTSIF flag in the ISR register."]
    #[inline(always)]
    pub fn set_ctscf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("pecf", &self.pecf())
            .field("fecf", &self.fecf())
            .field("ncf", &self.ncf())
            .field("orecf", &self.orecf())
            .field("idlecf", &self.idlecf())
            .field("tccf", &self.tccf())
            .field("ctscf", &self.ctscf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Icr {
            pecf: bool,
            fecf: bool,
            ncf: bool,
            orecf: bool,
            idlecf: bool,
            tccf: bool,
            ctscf: bool,
        }
        let proxy = Icr {
            pecf: self.pecf(),
            fecf: self.fecf(),
            ncf: self.ncf(),
            orecf: self.orecf(),
            idlecf: self.idlecf(),
            tccf: self.tccf(),
            ctscf: self.ctscf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Parity error. This bit is set when a parity error is detected in the received packet. 0: no parity error 1: parity error detected"]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error. This bit is set when a parity error is detected in the received packet. 0: no parity error 1: parity error detected"]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Framing error. This bit is set by hardware when stop bit is not correctly received 0: no framing error is detected 1: framing error is detected"]
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error. This bit is set by hardware when stop bit is not correctly received 0: no framing error is detected 1: framing error is detected"]
    #[inline(always)]
    pub fn set_fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Noise flag. Noise means the samping values in the 3-bit sampling mode are not the same. 0: no noise is detected 1: noise is detected"]
    #[inline(always)]
    pub const fn nf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Noise flag. Noise means the samping values in the 3-bit sampling mode are not the same. 0: no noise is detected 1: noise is detected"]
    #[inline(always)]
    pub fn set_nf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun error. When new data is received but Rx buffer is not empty (i.e. previous data is not read yet), ORE is asserted and current RDR content is not lost. This feature can be disabled by set CR3_OVRDIS to 1. 0: no overrun error 1: overrun error is detected"]
    #[inline(always)]
    pub const fn ore(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error. When new data is received but Rx buffer is not empty (i.e. previous data is not read yet), ORE is asserted and current RDR content is not lost. This feature can be disabled by set CR3_OVRDIS to 1. 0: no overrun error 1: overrun error is detected"]
    #[inline(always)]
    pub fn set_ore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Idle line detected 0: no idle line is detected 1: idle line is detected"]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Idle line detected 0: no idle line is detected 1: idle line is detected"]
    #[inline(always)]
    pub fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx data not empty. This bit is set by hardware when the received data is transferred into RDR register. 0: data is not received 1: data is ready in RDR to be read"]
    #[inline(always)]
    pub const fn rxne(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx data not empty. This bit is set by hardware when the received data is transferred into RDR register. 0: data is not received 1: data is ready in RDR to be read"]
    #[inline(always)]
    pub fn set_rxne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "transmission complete. This bit is set by hardware if the transmission is complete 0: transmission is not complete 1: transmission is complete"]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "transmission complete. This bit is set by hardware if the transmission is complete 0: transmission is not complete 1: transmission is complete"]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tx data empty 0: data is ready in TDR 1: data is already transferred to shift register, i.e. transmission is in progress or complete"]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tx data empty 0: data is ready in TDR 1: data is already transferred to shift register, i.e. transmission is in progress or complete"]
    #[inline(always)]
    pub fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CTS interrupt flag. This bit is set by hardware whenever CTS input toggles. 0: no change on the CTS line 1: there is a change on the CTS line"]
    #[inline(always)]
    pub const fn ctsif(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS interrupt flag. This bit is set by hardware whenever CTS input toggles. 0: no change on the CTS line 1: there is a change on the CTS line"]
    #[inline(always)]
    pub fn set_ctsif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CTS input. Read this bit to get the raw status of the CTS line."]
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CTS input. Read this bit to get the raw status of the CTS line."]
    #[inline(always)]
    pub fn set_cts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("pe", &self.pe())
            .field("fe", &self.fe())
            .field("nf", &self.nf())
            .field("ore", &self.ore())
            .field("idle", &self.idle())
            .field("rxne", &self.rxne())
            .field("tc", &self.tc())
            .field("txe", &self.txe())
            .field("ctsif", &self.ctsif())
            .field("cts", &self.cts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isr {
            pe: bool,
            fe: bool,
            nf: bool,
            ore: bool,
            idle: bool,
            rxne: bool,
            tc: bool,
            txe: bool,
            ctsif: bool,
            cts: bool,
        }
        let proxy = Isr {
            pe: self.pe(),
            fe: self.fe(),
            nf: self.nf(),
            ore: self.ore(),
            idle: self.idle(),
            rxne: self.rxne(),
            tc: self.tc(),
            txe: self.txe(),
            ctsif: self.ctsif(),
            cts: self.cts(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Miscellaneous Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miscr(pub u32);
impl Miscr {
    #[doc = "initial sample count, count down from this value to zero to reach the middle of the start bit in RxReserved-Do not modify"]
    #[inline(always)]
    pub const fn smplini(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "initial sample count, count down from this value to zero to reach the middle of the start bit in RxReserved-Do not modify"]
    #[inline(always)]
    pub fn set_smplini(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "assert RTS ahead of the frame completion (in number of bits)Reserved-Do not modify"]
    #[inline(always)]
    pub const fn rtsbit(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "assert RTS ahead of the frame completion (in number of bits)Reserved-Do not modify"]
    #[inline(always)]
    pub fn set_rtsbit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[inline(always)]
    pub const fn autocal(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_autocal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Miscr {
    #[inline(always)]
    fn default() -> Miscr {
        Miscr(0)
    }
}
impl core::fmt::Debug for Miscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Miscr")
            .field("smplini", &self.smplini())
            .field("rtsbit", &self.rtsbit())
            .field("autocal", &self.autocal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Miscr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Miscr {
            smplini: u8,
            rtsbit: u8,
            autocal: bool,
        }
        let proxy = Miscr {
            smplini: self.smplini(),
            rtsbit: self.rtsbit(),
            autocal: self.autocal(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc = "Received data"]
    #[inline(always)]
    pub const fn rdr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Received data"]
    #[inline(always)]
    pub fn set_rdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        Rdr(0)
    }
}
impl core::fmt::Debug for Rdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdr").field("rdr", &self.rdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rdr {
            rdr: u16,
        }
        let proxy = Rdr { rdr: self.rdr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Request Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rqr(pub u32);
impl Rqr {
    #[doc = "Rx data flush request. Write 1 to clear the RXNE flag and discard the current data in RDR"]
    #[inline(always)]
    pub const fn rxfrq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx data flush request. Write 1 to clear the RXNE flag and discard the current data in RDR"]
    #[inline(always)]
    pub fn set_rxfrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tx data flush requestReserved-Do not modify"]
    #[inline(always)]
    pub const fn txfrq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tx data flush requestReserved-Do not modify"]
    #[inline(always)]
    pub fn set_txfrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Rqr {
    #[inline(always)]
    fn default() -> Rqr {
        Rqr(0)
    }
}
impl core::fmt::Debug for Rqr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rqr")
            .field("rxfrq", &self.rxfrq())
            .field("txfrq", &self.txfrq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rqr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rqr {
            rxfrq: bool,
            txfrq: bool,
        }
        let proxy = Rqr {
            rxfrq: self.rxfrq(),
            txfrq: self.txfrq(),
        };
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
#[doc = "Transmit Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc = "Transmit data"]
    #[inline(always)]
    pub const fn tdr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Transmit data"]
    #[inline(always)]
    pub fn set_tdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        Tdr(0)
    }
}
impl core::fmt::Debug for Tdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr").field("tdr", &self.tdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tdr {
            tdr: u16,
        }
        let proxy = Tdr { tdr: self.tdr() };
        defmt::write!(f, "{}", proxy)
    }
}
