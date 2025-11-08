#[doc = "补丁通道寄存器"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub u32);
impl Ch {
    const ADDR_MASK: u32 = 0x1ffff << 2;
    #[doc = "补丁入口地址字段"]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        (self.0 & Self::ADDR_MASK) >> 2
    }
    #[doc = "设置补丁入口地址字段"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !Self::ADDR_MASK) | ((val & 0x1ffff) << 2);
    }
}
impl Default for Ch {
    #[inline(always)]
    fn default() -> Ch {
        Ch(0)
    }
}
impl core::fmt::Debug for Ch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ch")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ch {{ addr: {=u32:?} }}", self.addr());
    }
}
macro_rules! simple_reg {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct $name(pub u32);
        impl $name {
            #[inline(always)]
            pub const fn bits(&self) -> u32 {
                self.0
            }
            #[inline(always)]
            pub fn set_bits(&mut self, val: u32) {
                self.0 = val;
            }
        }
        impl Default for $name {
            #[inline(always)]
            fn default() -> $name {
                $name(0)
            }
        }
        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("bits", &format_args!("0x{:08x}", self.0))
                    .finish()
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for $name {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(
                    f,
                    "Reg {{ bits: {=u32:#010x} }}",
                    self.0
                );
            }
        }
    };
}
simple_reg!(Cer, "通道使能寄存器");
simple_reg!(Csr, "通道状态寄存器");
simple_reg!(Cdr, "通道数据寄存器");
simple_reg!(Ier, "中断使能寄存器");
simple_reg!(Isr, "中断状态寄存器");
simple_reg!(Icr, "中断清除寄存器");
simple_reg!(Ver, "模块版本寄存器");
