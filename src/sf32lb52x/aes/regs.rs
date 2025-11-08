#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AesSetting(pub u32);
impl AesSetting {
    #[doc = "AES Mode: 3'h0: ECB 3'h1: CTR 3'h2: CBC Others: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn aes_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "AES Mode: 3'h0: ECB 3'h1: CTR 3'h2: CBC Others: Reserved"]
    #[inline(always)]
    pub const fn set_aes_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "AES Length: 2'h0: 128-bit 2'h1: 192-bit 2'h2: 256-bit 2'h3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn aes_length(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "AES Length: 2'h0: 128-bit 2'h1: 192-bit 2'h2: 256-bit 2'h3: Reserved"]
    #[inline(always)]
    pub const fn set_aes_length(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "1'h0: select key from AES_ACC key registers 1'h1: use internal root key"]
    #[must_use]
    #[inline(always)]
    pub const fn key_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "1'h0: select key from AES_ACC key registers 1'h1: use internal root key"]
    #[inline(always)]
    pub const fn set_key_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "1'h0: AES 1'h1: SM4"]
    #[must_use]
    #[inline(always)]
    pub const fn algo_standard(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1'h0: AES 1'h1: SM4"]
    #[inline(always)]
    pub const fn set_algo_standard(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1'h0: decryption 1'h1: encryption"]
    #[must_use]
    #[inline(always)]
    pub const fn aes_op_mode(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1'h0: decryption 1'h1: encryption"]
    #[inline(always)]
    pub const fn set_aes_op_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1'h0: normal operation 1'h1: bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn aes_bypass(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1'h0: normal operation 1'h1: bypass"]
    #[inline(always)]
    pub const fn set_aes_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for AesSetting {
    #[inline(always)]
    fn default() -> AesSetting {
        AesSetting(0)
    }
}
impl core::fmt::Debug for AesSetting {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AesSetting")
            .field("aes_mode", &self.aes_mode())
            .field("aes_length", &self.aes_length())
            .field("key_sel", &self.key_sel())
            .field("algo_standard", &self.algo_standard())
            .field("aes_op_mode", &self.aes_op_mode())
            .field("aes_bypass", &self.aes_bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AesSetting {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AesSetting {{ aes_mode: {=u8:?}, aes_length: {=u8:?}, key_sel: {=bool:?}, algo_standard: {=bool:?}, aes_op_mode: {=bool:?}, aes_bypass: {=bool:?} }}" , self . aes_mode () , self . aes_length () , self . key_sel () , self . algo_standard () , self . aes_op_mode () , self . aes_bypass ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Command(pub u32);
impl Command {
    #[doc = "write 1 to trigger the AES_ACC block"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to trigger the AES_ACC block"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AES_ACC soft reset, 1'h1: reset the AES_ACC block"]
    #[must_use]
    #[inline(always)]
    pub const fn aes_acc_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC soft reset, 1'h1: reset the AES_ACC block"]
    #[inline(always)]
    pub const fn set_aes_acc_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to trigger the HASH_ACC block"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_start(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to trigger the HASH_ACC block"]
    #[inline(always)]
    pub const fn set_hash_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HASH_ACC soft reset, 1'h1: reset the HASH_ACC block"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_reset(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC soft reset, 1'h1: reset the HASH_ACC block"]
    #[inline(always)]
    pub const fn set_hash_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "auto clock gating"]
    #[must_use]
    #[inline(always)]
    pub const fn auto_gate(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "auto clock gating"]
    #[inline(always)]
    pub const fn set_auto_gate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Command {
    #[inline(always)]
    fn default() -> Command {
        Command(0)
    }
}
impl core::fmt::Debug for Command {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Command")
            .field("start", &self.start())
            .field("aes_acc_reset", &self.aes_acc_reset())
            .field("hash_start", &self.hash_start())
            .field("hash_reset", &self.hash_reset())
            .field("auto_gate", &self.auto_gate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Command {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Command {{ start: {=bool:?}, aes_acc_reset: {=bool:?}, hash_start: {=bool:?}, hash_reset: {=bool:?}, auto_gate: {=bool:?} }}" , self . start () , self . aes_acc_reset () , self . hash_start () , self . hash_reset () , self . auto_gate ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaData(pub u32);
impl DmaData {
    #[doc = "AES_ACC data block size, AES_ACC only support block aligned transaction. Each block contains 16 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "AES_ACC data block size, AES_ACC only support block aligned transaction. Each block contains 16 bytes."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for DmaData {
    #[inline(always)]
    fn default() -> DmaData {
        DmaData(0)
    }
}
impl core::fmt::Debug for DmaData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaData")
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaData {{ size: {=u32:?} }}", self.size())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaIn(pub u32);
impl DmaIn {
    #[doc = "AES_ACC input data address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES_ACC input data address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaIn {
    #[inline(always)]
    fn default() -> DmaIn {
        DmaIn(0)
    }
}
impl core::fmt::Debug for DmaIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaIn").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaIn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaIn {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaOut(pub u32);
impl DmaOut {
    #[doc = "AES_ACC output data address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES_ACC output data address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaOut {
    #[inline(always)]
    fn default() -> DmaOut {
        DmaOut(0)
    }
}
impl core::fmt::Debug for DmaOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaOut")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaOut {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtKeyW0(pub u32);
impl ExtKeyW0 {
    #[doc = "External Key Word0"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External Key Word0"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ExtKeyW0 {
    #[inline(always)]
    fn default() -> ExtKeyW0 {
        ExtKeyW0(0)
    }
}
impl core::fmt::Debug for ExtKeyW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtKeyW0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtKeyW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtKeyW0 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtKeyW1(pub u32);
impl ExtKeyW1 {
    #[doc = "External Key Word1"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External Key Word1"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ExtKeyW1 {
    #[inline(always)]
    fn default() -> ExtKeyW1 {
        ExtKeyW1(0)
    }
}
impl core::fmt::Debug for ExtKeyW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtKeyW1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtKeyW1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtKeyW1 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtKeyW2(pub u32);
impl ExtKeyW2 {
    #[doc = "External Key Word2"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External Key Word2"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ExtKeyW2 {
    #[inline(always)]
    fn default() -> ExtKeyW2 {
        ExtKeyW2(0)
    }
}
impl core::fmt::Debug for ExtKeyW2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtKeyW2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtKeyW2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtKeyW2 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtKeyW3(pub u32);
impl ExtKeyW3 {
    #[doc = "External Key Word3"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External Key Word3"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ExtKeyW3 {
    #[inline(always)]
    fn default() -> ExtKeyW3 {
        ExtKeyW3(0)
    }
}
impl core::fmt::Debug for ExtKeyW3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtKeyW3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtKeyW3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtKeyW3 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtKeyW4(pub u32);
impl ExtKeyW4 {
    #[doc = "External Key Word4"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External Key Word4"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ExtKeyW4 {
    #[inline(always)]
    fn default() -> ExtKeyW4 {
        ExtKeyW4(0)
    }
}
impl core::fmt::Debug for ExtKeyW4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtKeyW4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtKeyW4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtKeyW4 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtKeyW5(pub u32);
impl ExtKeyW5 {
    #[doc = "External Key Word5"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External Key Word5"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ExtKeyW5 {
    #[inline(always)]
    fn default() -> ExtKeyW5 {
        ExtKeyW5(0)
    }
}
impl core::fmt::Debug for ExtKeyW5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtKeyW5")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtKeyW5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtKeyW5 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtKeyW6(pub u32);
impl ExtKeyW6 {
    #[doc = "External Key Word6"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External Key Word6"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ExtKeyW6 {
    #[inline(always)]
    fn default() -> ExtKeyW6 {
        ExtKeyW6(0)
    }
}
impl core::fmt::Debug for ExtKeyW6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtKeyW6")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtKeyW6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtKeyW6 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtKeyW7(pub u32);
impl ExtKeyW7 {
    #[doc = "External Key Word7"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External Key Word7"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ExtKeyW7 {
    #[inline(always)]
    fn default() -> ExtKeyW7 {
        ExtKeyW7(0)
    }
}
impl core::fmt::Debug for ExtKeyW7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtKeyW7")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtKeyW7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtKeyW7 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashDmaData(pub u32);
impl HashDmaData {
    #[doc = "HASH input data byte size."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH input data byte size."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashDmaData {
    #[inline(always)]
    fn default() -> HashDmaData {
        HashDmaData(0)
    }
}
impl core::fmt::Debug for HashDmaData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashDmaData")
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashDmaData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashDmaData {{ size: {=u32:?} }}", self.size())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashDmaIn(pub u32);
impl HashDmaIn {
    #[doc = "input data address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "input data address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashDmaIn {
    #[inline(always)]
    fn default() -> HashDmaIn {
        HashDmaIn(0)
    }
}
impl core::fmt::Debug for HashDmaIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashDmaIn")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashDmaIn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashDmaIn {{ addr: {=u32:?} }}", self.addr())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashIvH0(pub u32);
impl HashIvH0 {
    #[doc = "HASH IV H0"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH IV H0"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashIvH0 {
    #[inline(always)]
    fn default() -> HashIvH0 {
        HashIvH0(0)
    }
}
impl core::fmt::Debug for HashIvH0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashIvH0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashIvH0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashIvH0 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashIvH1(pub u32);
impl HashIvH1 {
    #[doc = "HASH IV H1"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH IV H1"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashIvH1 {
    #[inline(always)]
    fn default() -> HashIvH1 {
        HashIvH1(0)
    }
}
impl core::fmt::Debug for HashIvH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashIvH1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashIvH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashIvH1 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashIvH2(pub u32);
impl HashIvH2 {
    #[doc = "HASH IV H2"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH IV H2"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashIvH2 {
    #[inline(always)]
    fn default() -> HashIvH2 {
        HashIvH2(0)
    }
}
impl core::fmt::Debug for HashIvH2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashIvH2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashIvH2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashIvH2 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashIvH3(pub u32);
impl HashIvH3 {
    #[doc = "HASH IV H3"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH IV H3"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashIvH3 {
    #[inline(always)]
    fn default() -> HashIvH3 {
        HashIvH3(0)
    }
}
impl core::fmt::Debug for HashIvH3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashIvH3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashIvH3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashIvH3 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashIvH4(pub u32);
impl HashIvH4 {
    #[doc = "HASH IV H4"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH IV H4"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashIvH4 {
    #[inline(always)]
    fn default() -> HashIvH4 {
        HashIvH4(0)
    }
}
impl core::fmt::Debug for HashIvH4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashIvH4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashIvH4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashIvH4 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashIvH5(pub u32);
impl HashIvH5 {
    #[doc = "HASH IV H5"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH IV H5"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashIvH5 {
    #[inline(always)]
    fn default() -> HashIvH5 {
        HashIvH5(0)
    }
}
impl core::fmt::Debug for HashIvH5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashIvH5")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashIvH5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashIvH5 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashIvH6(pub u32);
impl HashIvH6 {
    #[doc = "HASH IV H6"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH IV H6"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashIvH6 {
    #[inline(always)]
    fn default() -> HashIvH6 {
        HashIvH6(0)
    }
}
impl core::fmt::Debug for HashIvH6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashIvH6")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashIvH6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashIvH6 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashIvH7(pub u32);
impl HashIvH7 {
    #[doc = "HASH IV H7"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH IV H7"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashIvH7 {
    #[inline(always)]
    fn default() -> HashIvH7 {
        HashIvH7(0)
    }
}
impl core::fmt::Debug for HashIvH7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashIvH7")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashIvH7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashIvH7 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashLenH(pub u32);
impl HashLenH {
    #[doc = "HASH load length h"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "HASH load length h"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for HashLenH {
    #[inline(always)]
    fn default() -> HashLenH {
        HashLenH(0)
    }
}
impl core::fmt::Debug for HashLenH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashLenH")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashLenH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashLenH {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashLenL(pub u32);
impl HashLenL {
    #[doc = "HASH load length l"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH load length l"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashLenL {
    #[inline(always)]
    fn default() -> HashLenL {
        HashLenL(0)
    }
}
impl core::fmt::Debug for HashLenL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashLenL")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashLenL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashLenL {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultH0(pub u32);
impl HashResultH0 {
    #[doc = "HASH result H0"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result H0"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultH0 {
    #[inline(always)]
    fn default() -> HashResultH0 {
        HashResultH0(0)
    }
}
impl core::fmt::Debug for HashResultH0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultH0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultH0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultH0 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultH1(pub u32);
impl HashResultH1 {
    #[doc = "HASH result H1"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result H1"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultH1 {
    #[inline(always)]
    fn default() -> HashResultH1 {
        HashResultH1(0)
    }
}
impl core::fmt::Debug for HashResultH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultH1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultH1 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultH2(pub u32);
impl HashResultH2 {
    #[doc = "HASH result H2"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result H2"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultH2 {
    #[inline(always)]
    fn default() -> HashResultH2 {
        HashResultH2(0)
    }
}
impl core::fmt::Debug for HashResultH2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultH2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultH2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultH2 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultH3(pub u32);
impl HashResultH3 {
    #[doc = "HASH result H3"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result H3"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultH3 {
    #[inline(always)]
    fn default() -> HashResultH3 {
        HashResultH3(0)
    }
}
impl core::fmt::Debug for HashResultH3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultH3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultH3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultH3 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultH4(pub u32);
impl HashResultH4 {
    #[doc = "HASH result H4"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result H4"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultH4 {
    #[inline(always)]
    fn default() -> HashResultH4 {
        HashResultH4(0)
    }
}
impl core::fmt::Debug for HashResultH4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultH4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultH4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultH4 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultH5(pub u32);
impl HashResultH5 {
    #[doc = "HASH result H5"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result H5"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultH5 {
    #[inline(always)]
    fn default() -> HashResultH5 {
        HashResultH5(0)
    }
}
impl core::fmt::Debug for HashResultH5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultH5")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultH5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultH5 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultH6(pub u32);
impl HashResultH6 {
    #[doc = "HASH result H6"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result H6"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultH6 {
    #[inline(always)]
    fn default() -> HashResultH6 {
        HashResultH6(0)
    }
}
impl core::fmt::Debug for HashResultH6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultH6")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultH6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultH6 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultH7(pub u32);
impl HashResultH7 {
    #[doc = "HASH result H7"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result H7"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultH7 {
    #[inline(always)]
    fn default() -> HashResultH7 {
        HashResultH7(0)
    }
}
impl core::fmt::Debug for HashResultH7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultH7")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultH7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultH7 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultLenH(pub u32);
impl HashResultLenH {
    #[doc = "HASH result length h"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "HASH result length h"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for HashResultLenH {
    #[inline(always)]
    fn default() -> HashResultLenH {
        HashResultLenH(0)
    }
}
impl core::fmt::Debug for HashResultLenH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultLenH")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultLenH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultLenH {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashResultLenL(pub u32);
impl HashResultLenL {
    #[doc = "HASH result length l"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HASH result length l"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HashResultLenL {
    #[inline(always)]
    fn default() -> HashResultLenL {
        HashResultLenL(0)
    }
}
impl core::fmt::Debug for HashResultLenL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashResultLenL")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashResultLenL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HashResultLenL {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HashSetting(pub u32);
impl HashSetting {
    #[doc = "HASH Mode: 3'h0: SHA-1 3'h1: SHA-224 3'h2: SHA-256 3'h3: SM3 Others: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "HASH Mode: 3'h0: SHA-1 3'h1: SHA-224 3'h2: SHA-256 3'h3: SM3 Others: Reserved"]
    #[inline(always)]
    pub const fn set_hash_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "HASH padding enable. Set 1 to do padding after data transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn do_padding(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HASH padding enable. Set 1 to do padding after data transfer."]
    #[inline(always)]
    pub const fn set_do_padding(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HASH byte swap option. Set 1 to swap byte order when read data from memory."]
    #[must_use]
    #[inline(always)]
    pub const fn byte_swap(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HASH byte swap option. Set 1 to swap byte order when read data from memory."]
    #[inline(always)]
    pub const fn set_byte_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "HASH default iv select. 1'h0: default iv according to hash mode 1'h1: default iv from HASH_IV_H* registers"]
    #[must_use]
    #[inline(always)]
    pub const fn dft_iv_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HASH default iv select. 1'h0: default iv according to hash mode 1'h1: default iv from HASH_IV_H* registers"]
    #[inline(always)]
    pub const fn set_dft_iv_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "hash result endian setting: 1'h0: little endian 1'h1: big endian"]
    #[must_use]
    #[inline(always)]
    pub const fn result_endian(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "hash result endian setting: 1'h0: little endian 1'h1: big endian"]
    #[inline(always)]
    pub const fn set_result_endian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "write 1 to load hash iv"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_iv_load(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to load hash iv"]
    #[inline(always)]
    pub const fn set_hash_iv_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "write 1 to load hash length"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_len_load(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to load hash length"]
    #[inline(always)]
    pub const fn set_hash_len_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for HashSetting {
    #[inline(always)]
    fn default() -> HashSetting {
        HashSetting(0)
    }
}
impl core::fmt::Debug for HashSetting {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HashSetting")
            .field("hash_mode", &self.hash_mode())
            .field("do_padding", &self.do_padding())
            .field("byte_swap", &self.byte_swap())
            .field("dft_iv_sel", &self.dft_iv_sel())
            .field("result_endian", &self.result_endian())
            .field("hash_iv_load", &self.hash_iv_load())
            .field("hash_len_load", &self.hash_len_load())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HashSetting {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "HashSetting {{ hash_mode: {=u8:?}, do_padding: {=bool:?}, byte_swap: {=bool:?}, dft_iv_sel: {=bool:?}, result_endian: {=bool:?}, hash_iv_load: {=bool:?}, hash_len_load: {=bool:?} }}" , self . hash_mode () , self . do_padding () , self . byte_swap () , self . dft_iv_sel () , self . result_endian () , self . hash_iv_load () , self . hash_len_load ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub u32);
impl Irq {
    #[doc = "AES_ACC done status"]
    #[must_use]
    #[inline(always)]
    pub const fn done_stat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC done status"]
    #[inline(always)]
    pub const fn set_done_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AES_ACC bus error status"]
    #[must_use]
    #[inline(always)]
    pub const fn bus_err_stat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC bus error status"]
    #[inline(always)]
    pub const fn set_bus_err_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AES_ACC setup error status"]
    #[must_use]
    #[inline(always)]
    pub const fn setup_err_stat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC setup error status"]
    #[inline(always)]
    pub const fn set_setup_err_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HASH_ACC done status"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_done_stat(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC done status"]
    #[inline(always)]
    pub const fn set_hash_done_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HASH_ACC bus error status"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_bus_err_stat(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC bus error status"]
    #[inline(always)]
    pub const fn set_hash_bus_err_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "HASH_ACC padding error status"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_pad_err_stat(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC padding error status"]
    #[inline(always)]
    pub const fn set_hash_pad_err_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "AES_ACC done raw status"]
    #[must_use]
    #[inline(always)]
    pub const fn done_raw_stat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC done raw status"]
    #[inline(always)]
    pub const fn set_done_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "AES_ACC bus error raw status"]
    #[must_use]
    #[inline(always)]
    pub const fn bus_err_raw_stat(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC bus error raw status"]
    #[inline(always)]
    pub const fn set_bus_err_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "AES_ACC setup error raw status"]
    #[must_use]
    #[inline(always)]
    pub const fn setup_err_raw_stat(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC setup error raw status"]
    #[inline(always)]
    pub const fn set_setup_err_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "HASH_ACC done raw status"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_done_raw_stat(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC done raw status"]
    #[inline(always)]
    pub const fn set_hash_done_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "HASH_ACC bus error raw status"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_bus_err_raw_stat(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC bus error raw status"]
    #[inline(always)]
    pub const fn set_hash_bus_err_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "HASH_ACC padding error raw status"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_pad_err_raw_stat(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC padding error raw status"]
    #[inline(always)]
    pub const fn set_hash_pad_err_raw_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Irq {
    #[inline(always)]
    fn default() -> Irq {
        Irq(0)
    }
}
impl core::fmt::Debug for Irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irq")
            .field("done_stat", &self.done_stat())
            .field("bus_err_stat", &self.bus_err_stat())
            .field("setup_err_stat", &self.setup_err_stat())
            .field("hash_done_stat", &self.hash_done_stat())
            .field("hash_bus_err_stat", &self.hash_bus_err_stat())
            .field("hash_pad_err_stat", &self.hash_pad_err_stat())
            .field("done_raw_stat", &self.done_raw_stat())
            .field("bus_err_raw_stat", &self.bus_err_raw_stat())
            .field("setup_err_raw_stat", &self.setup_err_raw_stat())
            .field("hash_done_raw_stat", &self.hash_done_raw_stat())
            .field("hash_bus_err_raw_stat", &self.hash_bus_err_raw_stat())
            .field("hash_pad_err_raw_stat", &self.hash_pad_err_raw_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Irq {{ done_stat: {=bool:?}, bus_err_stat: {=bool:?}, setup_err_stat: {=bool:?}, hash_done_stat: {=bool:?}, hash_bus_err_stat: {=bool:?}, hash_pad_err_stat: {=bool:?}, done_raw_stat: {=bool:?}, bus_err_raw_stat: {=bool:?}, setup_err_raw_stat: {=bool:?}, hash_done_raw_stat: {=bool:?}, hash_bus_err_raw_stat: {=bool:?}, hash_pad_err_raw_stat: {=bool:?} }}" , self . done_stat () , self . bus_err_stat () , self . setup_err_stat () , self . hash_done_stat () , self . hash_bus_err_stat () , self . hash_pad_err_stat () , self . done_raw_stat () , self . bus_err_raw_stat () , self . setup_err_raw_stat () , self . hash_done_raw_stat () , self . hash_bus_err_raw_stat () , self . hash_pad_err_raw_stat ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvW0(pub u32);
impl IvW0 {
    #[doc = "Initial Vector Word0"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector Word0"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvW0 {
    #[inline(always)]
    fn default() -> IvW0 {
        IvW0(0)
    }
}
impl core::fmt::Debug for IvW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvW0").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvW0 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvW1(pub u32);
impl IvW1 {
    #[doc = "Initial Vector Word1"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector Word1"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvW1 {
    #[inline(always)]
    fn default() -> IvW1 {
        IvW1(0)
    }
}
impl core::fmt::Debug for IvW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvW1").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvW1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvW1 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvW2(pub u32);
impl IvW2 {
    #[doc = "Initial Vector Word2"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector Word2"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvW2 {
    #[inline(always)]
    fn default() -> IvW2 {
        IvW2(0)
    }
}
impl core::fmt::Debug for IvW2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvW2").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvW2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvW2 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvW3(pub u32);
impl IvW3 {
    #[doc = "Initial Vector Word3"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector Word3"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvW3 {
    #[inline(always)]
    fn default() -> IvW3 {
        IvW3(0)
    }
}
impl core::fmt::Debug for IvW3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvW3").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvW3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvW3 {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setting(pub u32);
impl Setting {
    #[doc = "AES_ACC done interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn done_irq_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC done interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_done_irq_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AES_ACC bus error interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn bus_err_irq_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC bus error interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_bus_err_irq_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AES_ACC setup error interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn setup_err_irq_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC setup error interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_setup_err_irq_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HASH_ACC done interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_done_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC done interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_hash_done_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HASH_ACC bus error interrpt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_bus_err_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC bus error interrpt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_hash_bus_err_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "HASH_ACC padding error interrupt mask, 0: mask the interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_pad_err_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC padding error interrupt mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn set_hash_pad_err_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Setting {
    #[inline(always)]
    fn default() -> Setting {
        Setting(0)
    }
}
impl core::fmt::Debug for Setting {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setting")
            .field("done_irq_mask", &self.done_irq_mask())
            .field("bus_err_irq_mask", &self.bus_err_irq_mask())
            .field("setup_err_irq_mask", &self.setup_err_irq_mask())
            .field("hash_done_mask", &self.hash_done_mask())
            .field("hash_bus_err_mask", &self.hash_bus_err_mask())
            .field("hash_pad_err_mask", &self.hash_pad_err_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setting {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Setting {{ done_irq_mask: {=bool:?}, bus_err_irq_mask: {=bool:?}, setup_err_irq_mask: {=bool:?}, hash_done_mask: {=bool:?}, hash_bus_err_mask: {=bool:?}, hash_pad_err_mask: {=bool:?} }}" , self . done_irq_mask () , self . bus_err_irq_mask () , self . setup_err_irq_mask () , self . hash_done_mask () , self . hash_bus_err_mask () , self . hash_pad_err_mask ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "AES_ACC block is busy"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AES_ACC block is busy"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "flash key valid indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_key_valid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "flash key valid indicator"]
    #[inline(always)]
    pub const fn set_flash_key_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HASH_ACC block is busy"]
    #[must_use]
    #[inline(always)]
    pub const fn hash_busy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HASH_ACC block is busy"]
    #[inline(always)]
    pub const fn set_hash_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("busy", &self.busy())
            .field("flash_key_valid", &self.flash_key_valid())
            .field("hash_busy", &self.hash_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ busy: {=bool:?}, flash_key_valid: {=bool:?}, hash_busy: {=bool:?} }}",
            self.busy(),
            self.flash_key_valid(),
            self.hash_busy()
        )
    }
}
