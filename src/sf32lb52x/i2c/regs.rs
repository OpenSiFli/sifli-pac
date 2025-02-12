#[doc = "Bus Monitor Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmr(pub u32);
impl Bmr {
    #[doc = "value of the SDA pin."]
    #[inline(always)]
    pub const fn sda(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "value of the SDA pin."]
    #[inline(always)]
    pub fn set_sda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "value of the SCL pin. Software can check bus level when the I2C bus is hung and the I2C unit must be reset."]
    #[inline(always)]
    pub const fn scl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "value of the SCL pin. Software can check bus level when the I2C bus is hung and the I2C unit must be reset."]
    #[inline(always)]
    pub fn set_scl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Bmr {
    #[inline(always)]
    fn default() -> Bmr {
        Bmr(0)
    }
}
impl core::fmt::Debug for Bmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bmr")
            .field("sda", &self.sda())
            .field("scl", &self.scl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bmr {
            sda: bool,
            scl: bool,
        }
        let proxy = Bmr {
            sda: self.sda(),
            scl: self.scl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Bus Mode (Master operation): 2'b00: standard-mode 2'b01: fast-mode and fast-mode plus 2'b10: HS-mode (standard mode when not doing a high speed transfer) 2'b11: HS-mode (fast mode when not doing a high speed transfer) Bus Mode (Slave operation): 2'b0x: HS-mode is disabled. I2C unit uses Standard/Fast mode timing on the SDA pin. 2'b1x: HS-mode is enabled. I2C unit uses HS-mode timing on the SDA pin when a master code is received."]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Bus Mode (Master operation): 2'b00: standard-mode 2'b01: fast-mode and fast-mode plus 2'b10: HS-mode (standard mode when not doing a high speed transfer) 2'b11: HS-mode (fast mode when not doing a high speed transfer) Bus Mode (Slave operation): 2'b0x: HS-mode is disabled. I2C unit uses Standard/Fast mode timing on the SDA pin. 2'b1x: HS-mode is enabled. I2C unit uses HS-mode timing on the SDA pin when a master code is received."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "I2C Unit Enable: 0 = Disables the unit and does not master any transactions or respond to any slave transactions. 1 = Enables the I2C (defaults to slave-receive mode). Software must guarantee the I2C bus is idle before setting this bit."]
    #[inline(always)]
    pub const fn iue(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Unit Enable: 0 = Disables the unit and does not master any transactions or respond to any slave transactions. 1 = Enables the I2C (defaults to slave-receive mode). Software must guarantee the I2C bus is idle before setting this bit."]
    #[inline(always)]
    pub fn set_iue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SCL Enable: 0 = Disables the I2C from driving the SCL line. 1 = Enables the I2C clock output for master-mode operation."]
    #[inline(always)]
    pub const fn scle(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SCL Enable: 0 = Disables the I2C from driving the SCL line. 1 = Enables the I2C clock output for master-mode operation."]
    #[inline(always)]
    pub fn set_scle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Enable for both TX and RX 0 = DMA mode is NOT enabled 1 = DMA mode enabled"]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable for both TX and RX 0 = DMA mode is NOT enabled 1 = DMA mode enabled"]
    #[inline(always)]
    pub fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Generate NACK for last DMA Read transfer"]
    #[inline(always)]
    pub const fn lastnack(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Generate NACK for last DMA Read transfer"]
    #[inline(always)]
    pub fn set_lastnack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Generate STOP for last DMA transfer"]
    #[inline(always)]
    pub const fn laststop(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Generate STOP for last DMA transfer"]
    #[inline(always)]
    pub fn set_laststop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Master Stop Detected Enable: 0 = Master Stop Detect (MSD) status is not enabled. 1 = Master Stop Detect (MSD) status is enabled."]
    #[inline(always)]
    pub const fn msde(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master Stop Detected Enable: 0 = Master Stop Detect (MSD) status is not enabled. 1 = Master Stop Detect (MSD) status is enabled."]
    #[inline(always)]
    pub fn set_msde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Push-pull mode Enable for SCL. 0 = open drain output for SCL. 1 = Push-pull output for SCL"]
    #[inline(always)]
    pub const fn sclpp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Push-pull mode Enable for SCL. 0 = open drain output for SCL. 1 = Push-pull output for SCL"]
    #[inline(always)]
    pub fn set_sclpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Slave mode Enable for SCL. 0 = Disable slave mode. Will not monitor slave address on I2C bus. 1 = Enable slave mode. Will monitor slave address on I2C bus."]
    #[inline(always)]
    pub const fn slven(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Slave mode Enable for SCL. 0 = Disable slave mode. Will not monitor slave address on I2C bus. 1 = Enable slave mode. Will monitor slave address on I2C bus."]
    #[inline(always)]
    pub fn set_slven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF*Tfclk. 0: Digital filter disabled 1: Digital filter enabled and filtering capability up to 1 Tfclk ... 7: digital filter enabled and filtering capability up to 7 Tfclk Digital filter is added to analog filter. Digital filter will introduce delay on SCL and SDA processing, which is essential in hs-mode."]
    #[inline(always)]
    pub const fn dnf(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF*Tfclk. 0: Digital filter disabled 1: Digital filter enabled and filtering capability up to 1 Tfclk ... 7: digital filter enabled and filtering capability up to 7 Tfclk Digital filter is added to analog filter. Digital filter will introduce delay on SCL and SDA processing, which is essential in hs-mode."]
    #[inline(always)]
    pub fn set_dnf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Reset bus related state machine and signals. Will be cleared by HW automatically 1 = request for reset 0 = reset finished"]
    #[inline(always)]
    pub const fn brgrst(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Reset bus related state machine and signals. Will be cleared by HW automatically 1 = request for reset 0 = reset finished"]
    #[inline(always)]
    pub fn set_brgrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "I2C will do bus reset upon this bit set. Will be cleared by HW automatically after RSTCYC cycles of SCL generated. 1 = request for i2c bus reset 0 = bus reset finished"]
    #[inline(always)]
    pub const fn rstreq(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "I2C will do bus reset upon this bit set. Will be cleared by HW automatically after RSTCYC cycles of SCL generated. 1 = request for i2c bus reset 0 = bus reset finished"]
    #[inline(always)]
    pub fn set_rstreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Unit Reset. Software need first assert to reset then deassert to release. 0 = No reset. 1 = Reset I2C module."]
    #[inline(always)]
    pub const fn ur(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Unit Reset. Software need first assert to reset then deassert to release. 0 = No reset. 1 = Reset I2C module."]
    #[inline(always)]
    pub fn set_ur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("mode", &self.mode())
            .field("iue", &self.iue())
            .field("scle", &self.scle())
            .field("dmaen", &self.dmaen())
            .field("lastnack", &self.lastnack())
            .field("laststop", &self.laststop())
            .field("msde", &self.msde())
            .field("sclpp", &self.sclpp())
            .field("slven", &self.slven())
            .field("dnf", &self.dnf())
            .field("brgrst", &self.brgrst())
            .field("rstreq", &self.rstreq())
            .field("ur", &self.ur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr {
            mode: u8,
            iue: bool,
            scle: bool,
            dmaen: bool,
            lastnack: bool,
            laststop: bool,
            msde: bool,
            sclpp: bool,
            slven: bool,
            dnf: u8,
            brgrst: bool,
            rstreq: bool,
            ur: bool,
        }
        let proxy = Cr {
            mode: self.mode(),
            iue: self.iue(),
            scle: self.scle(),
            dmaen: self.dmaen(),
            lastnack: self.lastnack(),
            laststop: self.laststop(),
            msde: self.msde(),
            sclpp: self.sclpp(),
            slven: self.slven(),
            dnf: self.dnf(),
            brgrst: self.brgrst(),
            rstreq: self.rstreq(),
            ur: self.ur(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Buffer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbr(pub u32);
impl Dbr {
    #[doc = "use the I2C Data Buffer register to transmit and receive data from the I2C bus. The DBR is accessed by software on one Side and by the I2C Shift register on the other. The DBR receives data coming into the I2C unit after a full byte is received and acknowledged. CPU writes data going out of the I2C to the DBR and sends it to the serial bus. When the I2C is in transmit mode (master or slave), CPU writes data to the DBR over the internal bus. CPU write data to the DBR when a master transaction is initiated or when the DBR transmit-empty interrupt is signalled. Data moves from the DBR to the Shift register when the transfer byte bit is set. The DBR transmit-empty interrupt is signalled (if enabled) when a byte is transferred on the I2C bus and the acknowledge cycle is complete. If the DBR is not written, and a Stop condition is not in place before the I2C bus is ready to transfer the next byte packet, the I2C unit inserts wait states until CPU writes the DBR and sets the transfer byte bit. When the I2C is in receive mode (master or slave), CPU reads DBR data over the internal bus. CPU reads data from the DBR when the DBR receive-full interrupt is signalled. The data moves from the Shift register to the DBR when the acknowledge cycle is complete. The I2C inserts wait states until the DBR is read. After the software reads the DBR, CR\\[NACK\\] are written by the software, allowing the next byte transfer to proceed to the I2C bus. In DMA mode, DBR is automatically filled from FIFO in master transmit mode, or fetched and stored in FIFO in master receive mode until DMA done or aborted."]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "use the I2C Data Buffer register to transmit and receive data from the I2C bus. The DBR is accessed by software on one Side and by the I2C Shift register on the other. The DBR receives data coming into the I2C unit after a full byte is received and acknowledged. CPU writes data going out of the I2C to the DBR and sends it to the serial bus. When the I2C is in transmit mode (master or slave), CPU writes data to the DBR over the internal bus. CPU write data to the DBR when a master transaction is initiated or when the DBR transmit-empty interrupt is signalled. Data moves from the DBR to the Shift register when the transfer byte bit is set. The DBR transmit-empty interrupt is signalled (if enabled) when a byte is transferred on the I2C bus and the acknowledge cycle is complete. If the DBR is not written, and a Stop condition is not in place before the I2C bus is ready to transfer the next byte packet, the I2C unit inserts wait states until CPU writes the DBR and sets the transfer byte bit. When the I2C is in receive mode (master or slave), CPU reads DBR data over the internal bus. CPU reads data from the DBR when the DBR receive-full interrupt is signalled. The data moves from the Shift register to the DBR when the acknowledge cycle is complete. The I2C inserts wait states until the DBR is read. After the software reads the DBR, CR\\[NACK\\] are written by the software, allowing the next byte transfer to proceed to the I2C bus. In DMA mode, DBR is automatically filled from FIFO in master transmit mode, or fetched and stored in FIFO in master receive mode until DMA done or aborted."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Dbr {
    #[inline(always)]
    fn default() -> Dbr {
        Dbr(0)
    }
}
impl core::fmt::Debug for Dbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbr {
            data: u8,
        }
        let proxy = Dbr { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA number register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dnr(pub u32);
impl Dnr {
    #[doc = "Write as number of data to transfer in byte. Read as left data number to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Write as number of data to transfer in byte. Read as left data number to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Dnr {
    #[inline(always)]
    fn default() -> Dnr {
        Dnr(0)
    }
}
impl core::fmt::Debug for Dnr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dnr").field("ndt", &self.ndt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dnr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dnr {
            ndt: u16,
        }
        let proxy = Dnr { ndt: self.ndt() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc = "Write to push send data into FIFO. Read to pop received data from FIFO"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write to push send data into FIFO. Read to pop received data from FIFO"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Fifo {
    #[inline(always)]
    fn default() -> Fifo {
        Fifo(0)
    }
}
impl core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifo").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fifo {
            data: u8,
        }
        let proxy = Fifo { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Slave Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when it detects a Stop condition while in slave mode."]
    #[inline(always)]
    pub const fn ssdie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when it detects a Stop condition while in slave mode."]
    #[inline(always)]
    pub fn set_ssdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Arbitration Loss Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon losing arbitration while in master mode."]
    #[inline(always)]
    pub const fn aldie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Loss Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon losing arbitration while in master mode."]
    #[inline(always)]
    pub fn set_aldie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DBR Transmit Empty Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt after transmitting a byte onto the I2C bus."]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DBR Transmit Empty Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt after transmitting a byte onto the I2C bus."]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DBR Receive Full Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when the DBR has received a data byte from the I2C bus."]
    #[inline(always)]
    pub const fn rfie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DBR Receive Full Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt when the DBR has received a data byte from the I2C bus."]
    #[inline(always)]
    pub fn set_rfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slave Address Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon detecting a slave address match or a general call address."]
    #[inline(always)]
    pub const fn sadie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt upon detecting a slave address match or a general call address."]
    #[inline(always)]
    pub fn set_sadie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bus Error Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt for the following I2C bus errors: As a master transmitter, no ACK was detected after a byte was sent. As a slave receiver, the I2C generated a NACK pulse. Software is responsible for guaranteeing that misplaced Start and Stop conditions do not occur."]
    #[inline(always)]
    pub const fn bedie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C to interrupt for the following I2C bus errors: As a master transmitter, no ACK was detected after a byte was sent. As a slave receiver, the I2C generated a NACK pulse. Software is responsible for guaranteeing that misplaced Start and Stop conditions do not occur."]
    #[inline(always)]
    pub fn set_bedie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Master Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C unit to interrupt upon detecting a Master Stop sent by the I2C unit."]
    #[inline(always)]
    pub const fn msdie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Master Stop Detected Interrupt Enable: 0 = Disable interrupt. 1 = Enables the I2C unit to interrupt upon detecting a Master Stop sent by the I2C unit."]
    #[inline(always)]
    pub fn set_msdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA Transaction Done Interrupt Enable 0 = DMA Transaction done interrupt is not enabled. 1 = DMA Transaction done interrupt is enabled."]
    #[inline(always)]
    pub const fn dmadoneie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transaction Done Interrupt Enable 0 = DMA Transaction done interrupt is not enabled. 1 = DMA Transaction done interrupt is enabled."]
    #[inline(always)]
    pub fn set_dmadoneie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "FIFO Overflow Interrupt Enable 0 = FIFO Overflow interrupt is not enabled 1 = FIFO Overflow interrupt is enabled"]
    #[inline(always)]
    pub const fn ofie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Interrupt Enable 0 = FIFO Overflow interrupt is not enabled 1 = FIFO Overflow interrupt is enabled"]
    #[inline(always)]
    pub fn set_ofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "FIFO Underflow Interrupt Enable 0 = FIFO Underflow interrupt is not enabled 1 = FIFO Underflow interrupt is enabled"]
    #[inline(always)]
    pub const fn ufie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Interrupt Enable 0 = FIFO Underflow interrupt is not enabled 1 = FIFO Underflow interrupt is enabled"]
    #[inline(always)]
    pub fn set_ufie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("ssdie", &self.ssdie())
            .field("aldie", &self.aldie())
            .field("teie", &self.teie())
            .field("rfie", &self.rfie())
            .field("sadie", &self.sadie())
            .field("bedie", &self.bedie())
            .field("msdie", &self.msdie())
            .field("dmadoneie", &self.dmadoneie())
            .field("ofie", &self.ofie())
            .field("ufie", &self.ufie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ier {
            ssdie: bool,
            aldie: bool,
            teie: bool,
            rfie: bool,
            sadie: bool,
            bedie: bool,
            msdie: bool,
            dmadoneie: bool,
            ofie: bool,
            ufie: bool,
        }
        let proxy = Ier {
            ssdie: self.ssdie(),
            aldie: self.aldie(),
            teie: self.teie(),
            rfie: self.rfie(),
            sadie: self.sadie(),
            bedie: self.bedie(),
            msdie: self.msdie(),
            dmadoneie: self.dmadoneie(),
            ofie: self.ofie(),
            ufie: self.ufie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Load Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcr(pub u32);
impl Lcr {
    #[doc = "Decrementer Load value for Standard Mode SCL (master mode) for both high and low phase. Data rate is generated as Ffclk/(SLV+max(SLV,CNT*2+6)+7+DNF) approximately. 100kbps data rate is generated by default if fclk is 48MHz. SLV also controls setup time and hold time for START and STOP condition in Standard Mode(master mode). Thdsta=Tsusta=Tsusto=Tfclk*SLV"]
    #[inline(always)]
    pub const fn slv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Decrementer Load value for Standard Mode SCL (master mode) for both high and low phase. Data rate is generated as Ffclk/(SLV+max(SLV,CNT*2+6)+7+DNF) approximately. 100kbps data rate is generated by default if fclk is 48MHz. SLV also controls setup time and hold time for START and STOP condition in Standard Mode(master mode). Thdsta=Tsusta=Tsusto=Tfclk*SLV"]
    #[inline(always)]
    pub fn set_slv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Decrementer Load value for Fast Mode (or Fast Mode Plus) SCL (master mode) for both high and low phase. Data rate is generated as Ffclk/(FLV+max(FLV,CNT*2+6)+7+DNF) approximately. 400kbps data rate is generated by default if fclk is 48MHz. FLV also controls setup time and hold time for START and STOP condition in Fast Mode(master mode). Thdsta=Tsusta=Tsusto=Tfclk*FLV"]
    #[inline(always)]
    pub const fn flv(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x01ff;
        val as u16
    }
    #[doc = "Decrementer Load value for Fast Mode (or Fast Mode Plus) SCL (master mode) for both high and low phase. Data rate is generated as Ffclk/(FLV+max(FLV,CNT*2+6)+7+DNF) approximately. 400kbps data rate is generated by default if fclk is 48MHz. FLV also controls setup time and hold time for START and STOP condition in Fast Mode(master mode). Thdsta=Tsusta=Tsusto=Tfclk*FLV"]
    #[inline(always)]
    pub fn set_flv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
    }
    #[doc = "Decrementer Load value for High Speed Mode SCL (master mode) for low phase. Tlow=Tfclk*(HLVL+3+DNF). Data rate is generated as 1/(Thigh+Tlow), or Ffclk/(HLVH+HLVL+7+2*DNF). 3.2Mbps data rate is generated by default if fclk is 48MHz. HLVL also controls setup time and hold time for START and STOP condition in High Speed Mode(master mode). Thdsta=Tsusta=Tsusto=Tfclk*(HLVL+1)"]
    #[inline(always)]
    pub const fn hlvl(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x01ff;
        val as u16
    }
    #[doc = "Decrementer Load value for High Speed Mode SCL (master mode) for low phase. Tlow=Tfclk*(HLVL+3+DNF). Data rate is generated as 1/(Thigh+Tlow), or Ffclk/(HLVH+HLVL+7+2*DNF). 3.2Mbps data rate is generated by default if fclk is 48MHz. HLVL also controls setup time and hold time for START and STOP condition in High Speed Mode(master mode). Thdsta=Tsusta=Tsusto=Tfclk*(HLVL+1)"]
    #[inline(always)]
    pub fn set_hlvl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 18usize)) | (((val as u32) & 0x01ff) << 18usize);
    }
    #[doc = "Decrementer Load value for High Speed Mode SCL (master mode) for high phase. Thigh=Tfclk*(HLVH+4+DNF)"]
    #[inline(always)]
    pub const fn hlvh(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "Decrementer Load value for High Speed Mode SCL (master mode) for high phase. Thigh=Tfclk*(HLVH+4+DNF)"]
    #[inline(always)]
    pub fn set_hlvh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for Lcr {
    #[inline(always)]
    fn default() -> Lcr {
        Lcr(0)
    }
}
impl core::fmt::Debug for Lcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lcr")
            .field("slv", &self.slv())
            .field("flv", &self.flv())
            .field("hlvl", &self.hlvl())
            .field("hlvh", &self.hlvh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lcr {
            slv: u16,
            flv: u16,
            hlvl: u16,
            hlvh: u8,
        }
        let proxy = Lcr {
            slv: self.slv(),
            flv: self.flv(),
            hlvl: self.hlvl(),
            hlvh: self.hlvh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bus Reset Cycle Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rccr(pub u32);
impl Rccr {
    #[doc = "The cycles of SCL during bus reset"]
    #[inline(always)]
    pub const fn rstcyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "The cycles of SCL during bus reset"]
    #[inline(always)]
    pub fn set_rstcyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Rccr {
    #[inline(always)]
    fn default() -> Rccr {
        Rccr(0)
    }
}
impl core::fmt::Debug for Rccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rccr")
            .field("rstcyc", &self.rstcyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rccr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rccr {
            rstcyc: u8,
        }
        let proxy = Rccr {
            rstcyc: self.rstcyc(),
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
#[doc = "Slave Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sar(pub u32);
impl Sar {
    #[doc = "The seven-bit address to which the I2C responds when in slave-receive mode"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "The seven-bit address to which the I2C responds when in slave-receive mode"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Sar {
    #[inline(always)]
    fn default() -> Sar {
        Sar(0)
    }
}
impl core::fmt::Debug for Sar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sar").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sar {
            addr: u8,
        }
        let proxy = Sar { addr: self.addr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Read/write Mode: 0 = The I2C is in master-transmit or slave-receive mode. 1 = The I2C is in master-receive or slave-transmit mode. This is the R/nW bit of the slave address. It is cleared automatically by hardware after a Stop state."]
    #[inline(always)]
    pub const fn rwm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read/write Mode: 0 = The I2C is in master-transmit or slave-receive mode. 1 = The I2C is in master-receive or slave-transmit mode. This is the R/nW bit of the slave address. It is cleared automatically by hardware after a Stop state."]
    #[inline(always)]
    pub fn set_rwm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ACK/NACK Status: 0 = The I2C received or sent an ACK on the bus. 1 = The I2C received or sent a NACK.on the bus. This bit is used in slave-transmit mode to determine when the byte transferred is the last one. This bit is updated after each byte and ACK/NACK information is received."]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ACK/NACK Status: 0 = The I2C received or sent an ACK on the bus. 1 = The I2C received or sent a NACK.on the bus. This bit is used in slave-transmit mode to determine when the byte transferred is the last one. This bit is updated after each byte and ACK/NACK information is received."]
    #[inline(always)]
    pub fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Unit Busy: 0 = I2C not busy. 1 = Set when local I2C is busy. This is defined as the time between the first Start and Stop."]
    #[inline(always)]
    pub const fn ub(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Unit Busy: 0 = I2C not busy. 1 = Set when local I2C is busy. This is defined as the time between the first Start and Stop."]
    #[inline(always)]
    pub fn set_ub(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C Bus Busy: 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the I2C bus is busy but local I2C is not involved in the transaction."]
    #[inline(always)]
    pub const fn ibb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Bus Busy: 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the I2C bus is busy but local I2C is not involved in the transaction."]
    #[inline(always)]
    pub fn set_ibb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Slave Stop Detected: 0 = No Stop detected. 1 = Set when the I2C detects a Stop while in slave-receive or slave-transmit mode. Cleared if write 1"]
    #[inline(always)]
    pub const fn ssd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Stop Detected: 0 = No Stop detected. 1 = Set when the I2C detects a Stop while in slave-receive or slave-transmit mode. Cleared if write 1"]
    #[inline(always)]
    pub fn set_ssd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Arbitration Loss Detected: Used during multi-master operation: 0 = Cleared when arbitration is won or never took place. 1 = Set when the I2C loses arbitration. Cleared if write 1"]
    #[inline(always)]
    pub const fn ald(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Loss Detected: Used during multi-master operation: 0 = Cleared when arbitration is won or never took place. 1 = Set when the I2C loses arbitration. Cleared if write 1"]
    #[inline(always)]
    pub fn set_ald(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DBR Transmit Empty: 0 = The data byte is still being transmitted. 1 = The I2C has finished transmitting a data byte on the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1"]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DBR Transmit Empty: 0 = The data byte is still being transmitted. 1 = The I2C has finished transmitting a data byte on the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1"]
    #[inline(always)]
    pub fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DBR Receive Full: 0 = The DBR has not received a new data byte or the I2C is idle. 1 = The DBR register received a new data byte from the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1"]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DBR Receive Full: 0 = The DBR has not received a new data byte or the I2C is idle. 1 = The DBR register received a new data byte from the I2C bus. An interrupt is signalled when enabled in the CR. Cleared if write 1"]
    #[inline(always)]
    pub fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Slave Address Detected: 0 = No slave address was detected. 1 = The I2C detected a seven-bit address that matches the general call address or SAR. An interrupt is signalled when enabled in the CR. Cleared if write 1"]
    #[inline(always)]
    pub const fn sad(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Address Detected: 0 = No slave address was detected. 1 = The I2C detected a seven-bit address that matches the general call address or SAR. An interrupt is signalled when enabled in the CR. Cleared if write 1"]
    #[inline(always)]
    pub fn set_sad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bus Error Detected: 0 = No error detected. 1 = The I2C sets this bit when it detects one of the following error conditions: As a master transmitter, no ACK was detected on the interface after a byte was sent. As a slave receiver, the I2C generates a NACK pulse. When an error occurs, I2C bus transactions continue. Software must guarantee that misplaced Start and Stop conditions do not occur. Cleared if write 1"]
    #[inline(always)]
    pub const fn bed(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Detected: 0 = No error detected. 1 = The I2C sets this bit when it detects one of the following error conditions: As a master transmitter, no ACK was detected on the interface after a byte was sent. As a slave receiver, the I2C generates a NACK pulse. When an error occurs, I2C bus transactions continue. Software must guarantee that misplaced Start and Stop conditions do not occur. Cleared if write 1"]
    #[inline(always)]
    pub fn set_bed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Early Bus Busy 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the unit detects that the SCL or SDA line is low without a START condition. Bit will remain set until the I2C unit detects the bus is idle by detecting a STOP condition. Bit will also be set whenever the IBB bit is set."]
    #[inline(always)]
    pub const fn ebb(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Early Bus Busy 0 = I2C bus is idle or the I2C is using the bus (that is, unit busy). 1 = Set when the unit detects that the SCL or SDA line is low without a START condition. Bit will remain set until the I2C unit detects the bus is idle by detecting a STOP condition. Bit will also be set whenever the IBB bit is set."]
    #[inline(always)]
    pub fn set_ebb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Master Stop Detected: 0 = No Master Stop Detected. 1 = This bit is set by the I2C unit when all of the following are true: This bit is enabled (CR\\[MSDE\\] = 1); I2C unit is configured as a master; I2C transmits a STOP signal"]
    #[inline(always)]
    pub const fn msd(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Master Stop Detected: 0 = No Master Stop Detected. 1 = This bit is set by the I2C unit when all of the following are true: This bit is enabled (CR\\[MSDE\\] = 1); I2C unit is configured as a master; I2C transmits a STOP signal"]
    #[inline(always)]
    pub fn set_msd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA Transaction Done. Asserted when both APB and I2C bus have finished transfer. Cleared if write 1"]
    #[inline(always)]
    pub const fn dmadone(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transaction Done. Asserted when both APB and I2C bus have finished transfer. Cleared if write 1"]
    #[inline(always)]
    pub fn set_dmadone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "FIFO Overflow Flag. Asserted when FIFO is full and a PUSH request generated without a POP. Cleared if write 1"]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Flag. Asserted when FIFO is full and a PUSH request generated without a POP. Cleared if write 1"]
    #[inline(always)]
    pub fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "FIFO Underflow Flag. Asserted when FIFO is empty and a POP request generated without a PUSH. Cleared if write 1"]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Flag. Asserted when FIFO is empty and a POP request generated without a PUSH. Cleared if write 1"]
    #[inline(always)]
    pub fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("rwm", &self.rwm())
            .field("nack", &self.nack())
            .field("ub", &self.ub())
            .field("ibb", &self.ibb())
            .field("ssd", &self.ssd())
            .field("ald", &self.ald())
            .field("te", &self.te())
            .field("rf", &self.rf())
            .field("sad", &self.sad())
            .field("bed", &self.bed())
            .field("ebb", &self.ebb())
            .field("msd", &self.msd())
            .field("dmadone", &self.dmadone())
            .field("of", &self.of())
            .field("uf", &self.uf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sr {
            rwm: bool,
            nack: bool,
            ub: bool,
            ibb: bool,
            ssd: bool,
            ald: bool,
            te: bool,
            rf: bool,
            sad: bool,
            bed: bool,
            ebb: bool,
            msd: bool,
            dmadone: bool,
            of: bool,
            uf: bool,
        }
        let proxy = Sr {
            rwm: self.rwm(),
            nack: self.nack(),
            ub: self.ub(),
            ibb: self.ibb(),
            ssd: self.ssd(),
            ald: self.ald(),
            te: self.te(),
            rf: self.rf(),
            sad: self.sad(),
            bed: self.bed(),
            ebb: self.ebb(),
            msd: self.msd(),
            dmadone: self.dmadone(),
            of: self.of(),
            uf: self.uf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Transfer Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Transfer Byte: Used to send or receive a byte on the I2C bus: 0 = Cleared by I2C when the byte is sent/received. 1 = Send/receive a byte. CPU can monitor this bit to determine when the byte transfer has completed. In master or slave mode, after each byte transfer including acknowledge pulse, the I2C holds the SCL line low (inserting wait states) until TB is set."]
    #[inline(always)]
    pub const fn tb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Byte: Used to send or receive a byte on the I2C bus: 0 = Cleared by I2C when the byte is sent/received. 1 = Send/receive a byte. CPU can monitor this bit to determine when the byte transfer has completed. In master or slave mode, after each byte transfer including acknowledge pulse, the I2C holds the SCL line low (inserting wait states) until TB is set."]
    #[inline(always)]
    pub fn set_tb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start: Used to initiate a Start condition to the I2C unit when in master mode. 0 = Do not send a Start pulse. 1 = Send a Start pulse."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Start: Used to initiate a Start condition to the I2C unit when in master mode. 0 = Do not send a Start pulse. 1 = Send a Start pulse."]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Stop: Used to initiate a Stop condition after transferring the next data byte on the I2C bus when in master mode. In master-receive mode, the NACK control bit must be set in conjunction with the STOP bit. 0 = Do not send a Stop. 1 = Send a Stop."]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Stop: Used to initiate a Stop condition after transferring the next data byte on the I2C bus when in master mode. In master-receive mode, the NACK control bit must be set in conjunction with the STOP bit. 0 = Do not send a Stop. 1 = Send a Stop."]
    #[inline(always)]
    pub fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "The positive/negative acknowledge control bit, defines the type of acknowledge pulse sent by the I2C when in master receive mode: 0 = Send a positive acknowledge (ACK) pulse after receiving a data byte. 1 = Send a negative acknowledge (NACK) pulse after receiving a data byte. The I2C automatically sends an ACK pulse when responding to its slave address or when responding in slave-receive mode, regardless of the NACK control-bit setting."]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "The positive/negative acknowledge control bit, defines the type of acknowledge pulse sent by the I2C when in master receive mode: 0 = Send a positive acknowledge (ACK) pulse after receiving a data byte. 1 = Send a negative acknowledge (NACK) pulse after receiving a data byte. The I2C automatically sends an ACK pulse when responding to its slave address or when responding in slave-receive mode, regardless of the NACK control-bit setting."]
    #[inline(always)]
    pub fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Master Abort: Used by the I2C in master mode to generate a Stop without transmitting another data byte: 0 = The I2C transmits Stop on if TCR\\[STOP\\] is set. 1 = The I2C sends Stop without data transmission. When in master-transmit mode, after transmitting a data byte, the TCR\\[TB\\] bit is cleared. When no more data bytes need to be sent, setting master abort bit sends the Stop. The TCR\\[TB\\] bit must remain clear. In master-receive mode, when a NAK is sent without a Stop (TCR\\[STOP\\] bit was not set) and CPU does not send a repeated Start, setting this bit sends the Stop. Once again, the TCR\\[TB\\] bit must remain clear. Master Abort can be done immediately after the address phase (Master Transmit mode only)."]
    #[inline(always)]
    pub const fn ma(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Master Abort: Used by the I2C in master mode to generate a Stop without transmitting another data byte: 0 = The I2C transmits Stop on if TCR\\[STOP\\] is set. 1 = The I2C sends Stop without data transmission. When in master-transmit mode, after transmitting a data byte, the TCR\\[TB\\] bit is cleared. When no more data bytes need to be sent, setting master abort bit sends the Stop. The TCR\\[TB\\] bit must remain clear. In master-receive mode, when a NAK is sent without a Stop (TCR\\[STOP\\] bit was not set) and CPU does not send a repeated Start, setting this bit sends the Stop. Once again, the TCR\\[TB\\] bit must remain clear. Master Abort can be done immediately after the address phase (Master Transmit mode only)."]
    #[inline(always)]
    pub fn set_ma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Request DMA TX. Will be cleared by HW automatically"]
    #[inline(always)]
    pub const fn txreq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Request DMA TX. Will be cleared by HW automatically"]
    #[inline(always)]
    pub fn set_txreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Request DMA RX. Will be cleared by HW automatically"]
    #[inline(always)]
    pub const fn rxreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Request DMA RX. Will be cleared by HW automatically"]
    #[inline(always)]
    pub fn set_rxreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Abort DMA operation. Will be cleared by HW automatically"]
    #[inline(always)]
    pub const fn abortdma(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Abort DMA operation. Will be cleared by HW automatically"]
    #[inline(always)]
    pub fn set_abortdma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("tb", &self.tb())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("nack", &self.nack())
            .field("ma", &self.ma())
            .field("txreq", &self.txreq())
            .field("rxreq", &self.rxreq())
            .field("abortdma", &self.abortdma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tcr {
            tb: bool,
            start: bool,
            stop: bool,
            nack: bool,
            ma: bool,
            txreq: bool,
            rxreq: bool,
            abortdma: bool,
        }
        let proxy = Tcr {
            tb: self.tb(),
            start: self.start(),
            stop: self.stop(),
            nack: self.nack(),
            ma: self.ma(),
            txreq: self.txreq(),
            rxreq: self.rxreq(),
            abortdma: self.abortdma(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Wait Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wcr(pub u32);
impl Wcr {
    #[doc = "Controls the counter values defining the setup and hold times in standard and fast mode Tvddat=Thddat=Tfclk*(CNT+2) Tsudat=max(Tlow-Thddat,Thddat) Lower counter values may violate setup and hold times."]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Controls the counter values defining the setup and hold times in standard and fast mode Tvddat=Thddat=Tfclk*(CNT+2) Tsudat=max(Tlow-Thddat,Thddat) Lower counter values may violate setup and hold times."]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Wcr {
    #[inline(always)]
    fn default() -> Wcr {
        Wcr(0)
    }
}
impl core::fmt::Debug for Wcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wcr").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Wcr {
            cnt: u8,
        }
        let proxy = Wcr { cnt: self.cnt() };
        defmt::write!(f, "{}", proxy)
    }
}
