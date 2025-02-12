#[doc = "AEZIP ctrl"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AezipCtrl(pub u32);
impl AezipCtrl {
    #[doc = "AEZIP ctrl"]
    #[inline(always)]
    pub const fn aezip_ctrl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AEZIP ctrl"]
    #[inline(always)]
    pub fn set_aezip_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for AezipCtrl {
    #[inline(always)]
    fn default() -> AezipCtrl {
        AezipCtrl(0)
    }
}
impl core::fmt::Debug for AezipCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AezipCtrl")
            .field("aezip_ctrl", &self.aezip_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AezipCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AezipCtrl {
            aezip_ctrl: bool,
        }
        let proxy = AezipCtrl {
            aezip_ctrl: self.aezip_ctrl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip index cache clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheClr(pub u32);
impl CacheClr {
    #[doc = "no used"]
    #[inline(always)]
    pub const fn cache_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no used"]
    #[inline(always)]
    pub fn set_cache_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CacheClr {
    #[inline(always)]
    fn default() -> CacheClr {
        CacheClr(0)
    }
}
impl core::fmt::Debug for CacheClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CacheClr")
            .field("cache_clr", &self.cache_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CacheClr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CacheClr {
            cache_clr: bool,
        }
        let proxy = CacheClr {
            cache_clr: self.cache_clr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData0(pub u32);
impl DbData0 {
    #[doc = "bit\\[31:24\\] bit_depth bit\\[23:16\\] color_type bit\\[15:0\\] block number"]
    #[inline(always)]
    pub const fn db_data0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[31:24\\] bit_depth bit\\[23:16\\] color_type bit\\[15:0\\] block number"]
    #[inline(always)]
    pub fn set_db_data0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData0 {
    #[inline(always)]
    fn default() -> DbData0 {
        DbData0(0)
    }
}
impl core::fmt::Debug for DbData0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData0")
            .field("db_data0", &self.db_data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData0 {
            db_data0: u32,
        }
        let proxy = DbData0 {
            db_data0: self.db_data0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData1(pub u32);
impl DbData1 {
    #[doc = "bit\\[31:16\\] width bit\\[15:0\\] height"]
    #[inline(always)]
    pub const fn db_data1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[31:16\\] width bit\\[15:0\\] height"]
    #[inline(always)]
    pub fn set_db_data1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData1 {
    #[inline(always)]
    fn default() -> DbData1 {
        DbData1(0)
    }
}
impl core::fmt::Debug for DbData1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData1")
            .field("db_data1", &self.db_data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData1 {
            db_data1: u32,
        }
        let proxy = DbData1 {
            db_data1: self.db_data1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData10(pub u32);
impl DbData10 {
    #[doc = "bit\\[15:8 dispos_op_cur bit\\[7:0\\] blend_op_cur"]
    #[inline(always)]
    pub const fn db_data10(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[15:8 dispos_op_cur bit\\[7:0\\] blend_op_cur"]
    #[inline(always)]
    pub fn set_db_data10(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData10 {
    #[inline(always)]
    fn default() -> DbData10 {
        DbData10(0)
    }
}
impl core::fmt::Debug for DbData10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData10")
            .field("db_data10", &self.db_data10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData10 {
            db_data10: u32,
        }
        let proxy = DbData10 {
            db_data10: self.db_data10(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData11(pub u32);
impl DbData11 {
    #[doc = "frame_cont_cur"]
    #[inline(always)]
    pub const fn db_data11(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "frame_cont_cur"]
    #[inline(always)]
    pub fn set_db_data11(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData11 {
    #[inline(always)]
    fn default() -> DbData11 {
        DbData11(0)
    }
}
impl core::fmt::Debug for DbData11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData11")
            .field("db_data11", &self.db_data11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData11 {
            db_data11: u32,
        }
        let proxy = DbData11 {
            db_data11: self.db_data11(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData12(pub u32);
impl DbData12 {
    #[doc = "paly_cont_cur"]
    #[inline(always)]
    pub const fn db_data12(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "paly_cont_cur"]
    #[inline(always)]
    pub fn set_db_data12(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData12 {
    #[inline(always)]
    fn default() -> DbData12 {
        DbData12(0)
    }
}
impl core::fmt::Debug for DbData12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData12")
            .field("db_data12", &self.db_data12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData12 {
            db_data12: u32,
        }
        let proxy = DbData12 {
            db_data12: self.db_data12(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData13(pub u32);
impl DbData13 {
    #[doc = "frame_size_cur"]
    #[inline(always)]
    pub const fn db_data13(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "frame_size_cur"]
    #[inline(always)]
    pub fn set_db_data13(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData13 {
    #[inline(always)]
    fn default() -> DbData13 {
        DbData13(0)
    }
}
impl core::fmt::Debug for DbData13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData13")
            .field("db_data13", &self.db_data13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData13 {
            db_data13: u32,
        }
        let proxy = DbData13 {
            db_data13: self.db_data13(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData2(pub u32);
impl DbData2 {
    #[doc = "bit\\[31:0\\] total_len"]
    #[inline(always)]
    pub const fn db_data2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[31:0\\] total_len"]
    #[inline(always)]
    pub fn set_db_data2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData2 {
    #[inline(always)]
    fn default() -> DbData2 {
        DbData2(0)
    }
}
impl core::fmt::Debug for DbData2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData2")
            .field("db_data2", &self.db_data2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData2 {
            db_data2: u32,
        }
        let proxy = DbData2 {
            db_data2: self.db_data2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData3(pub u32);
impl DbData3 {
    #[doc = "bit\\[31:24\\] ezip_type bit\\[23:20\\] bfinal bit\\[19:16\\] btype bit\\[11:8\\] ahb_state bit\\[7:4\\] ctrl_state bir\\[3:0\\] build_stste"]
    #[inline(always)]
    pub const fn db_data3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[31:24\\] ezip_type bit\\[23:20\\] bfinal bit\\[19:16\\] btype bit\\[11:8\\] ahb_state bit\\[7:4\\] ctrl_state bir\\[3:0\\] build_stste"]
    #[inline(always)]
    pub fn set_db_data3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData3 {
    #[inline(always)]
    fn default() -> DbData3 {
        DbData3(0)
    }
}
impl core::fmt::Debug for DbData3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData3")
            .field("db_data3", &self.db_data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData3 {
            db_data3: u32,
        }
        let proxy = DbData3 {
            db_data3: self.db_data3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData4(pub u32);
impl DbData4 {
    #[doc = "bit\\[9\\]:ezip_buf_full bit\\[8\\]:ezip_buf_empty bit\\[7\\]:dec_buf_full bit\\[6\\]:dec_buf_empty bit\\[5\\]:bypass_on bit\\[4\\]:dec_on bit\\[3\\]:ind3_on bit\\[2\\]:ind2_on bit\\[1\\]:ind1_on bit\\[0\\]:ezip_on"]
    #[inline(always)]
    pub const fn db_data4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[9\\]:ezip_buf_full bit\\[8\\]:ezip_buf_empty bit\\[7\\]:dec_buf_full bit\\[6\\]:dec_buf_empty bit\\[5\\]:bypass_on bit\\[4\\]:dec_on bit\\[3\\]:ind3_on bit\\[2\\]:ind2_on bit\\[1\\]:ind1_on bit\\[0\\]:ezip_on"]
    #[inline(always)]
    pub fn set_db_data4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData4 {
    #[inline(always)]
    fn default() -> DbData4 {
        DbData4(0)
    }
}
impl core::fmt::Debug for DbData4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData4")
            .field("db_data4", &self.db_data4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData4 {
            db_data4: u32,
        }
        let proxy = DbData4 {
            db_data4: self.db_data4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData5(pub u32);
impl DbData5 {
    #[doc = "bit\\[31:16\\] width_cnt_cur bit\\[15:0\\] height_cnt_cur"]
    #[inline(always)]
    pub const fn db_data5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[31:16\\] width_cnt_cur bit\\[15:0\\] height_cnt_cur"]
    #[inline(always)]
    pub fn set_db_data5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData5 {
    #[inline(always)]
    fn default() -> DbData5 {
        DbData5(0)
    }
}
impl core::fmt::Debug for DbData5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData5")
            .field("db_data5", &self.db_data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData5 {
            db_data5: u32,
        }
        let proxy = DbData5 {
            db_data5: self.db_data5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData6(pub u32);
impl DbData6 {
    #[doc = "seq_num"]
    #[inline(always)]
    pub const fn db_data6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "seq_num"]
    #[inline(always)]
    pub fn set_db_data6(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData6 {
    #[inline(always)]
    fn default() -> DbData6 {
        DbData6(0)
    }
}
impl core::fmt::Debug for DbData6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData6")
            .field("db_data6", &self.db_data6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData6 {
            db_data6: u32,
        }
        let proxy = DbData6 {
            db_data6: self.db_data6(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData7(pub u32);
impl DbData7 {
    #[doc = "bit\\[31:16\\] frame_width_cur bit\\[15:0\\] frame_height_cur"]
    #[inline(always)]
    pub const fn db_data7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[31:16\\] frame_width_cur bit\\[15:0\\] frame_height_cur"]
    #[inline(always)]
    pub fn set_db_data7(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData7 {
    #[inline(always)]
    fn default() -> DbData7 {
        DbData7(0)
    }
}
impl core::fmt::Debug for DbData7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData7")
            .field("db_data7", &self.db_data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData7 {
            db_data7: u32,
        }
        let proxy = DbData7 {
            db_data7: self.db_data7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData8(pub u32);
impl DbData8 {
    #[doc = "bit\\[31:16 \\]frame_offsetx_cur bit\\[15:0\\] frame_offsety_cur"]
    #[inline(always)]
    pub const fn db_data8(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[31:16 \\]frame_offsetx_cur bit\\[15:0\\] frame_offsety_cur"]
    #[inline(always)]
    pub fn set_db_data8(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData8 {
    #[inline(always)]
    fn default() -> DbData8 {
        DbData8(0)
    }
}
impl core::fmt::Debug for DbData8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData8")
            .field("db_data8", &self.db_data8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData8 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData8 {
            db_data8: u32,
        }
        let proxy = DbData8 {
            db_data8: self.db_data8(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug data9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbData9(pub u32);
impl DbData9 {
    #[doc = "bit\\[31:16 \\]delay_num_cur bit\\[15:0\\] delay_den_cur"]
    #[inline(always)]
    pub const fn db_data9(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bit\\[31:16 \\]delay_num_cur bit\\[15:0\\] delay_den_cur"]
    #[inline(always)]
    pub fn set_db_data9(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbData9 {
    #[inline(always)]
    fn default() -> DbData9 {
        DbData9(0)
    }
}
impl core::fmt::Debug for DbData9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbData9")
            .field("db_data9", &self.db_data9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbData9 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbData9 {
            db_data9: u32,
        }
        let proxy = DbData9 {
            db_data9: self.db_data9(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder debug sel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbSel(pub u32);
impl DbSel {
    #[doc = "bit\\[15\\] 0:line_first 1:out_buf_en\\[1\\] bit\\[14\\] 0:rd_head3 1:out_buf_en\\[0\\] bit\\[13\\] 0:rd_head2 1:inbuf_empty bit\\[12\\] 0:rd_heas1 1:inbuf_half_empty bit\\[11\\] 0:blk_restart 1: inbuf_full bit\\[10\\] 0:ezip_buf_end 1:ezip_pixel_end bit\\[9\\] 0:ezip_buf_full 1:0 bit\\[8\\] 0:ezip_buf_empty 1:0 bit\\[7\\] 0:dec_buf_empty 1:0 bit\\[6\\] 0:dec_buf_full 1:para_ok bit\\[5\\] 0:dec_on 1:ezip_fuf_push bit\\[4\\] 0:ind3_on 1:copy_on bit\\[3\\] 0:ind2_on 1:bypass_on bit\\[2\\] 0:ind1_on 1:blk_clr bit\\[1\\] 0:ezip_on 1:para_val bit\\[0\\] 0:ezip_int 1:para_req"]
    #[inline(always)]
    pub const fn db_sel(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "bit\\[15\\] 0:line_first 1:out_buf_en\\[1\\] bit\\[14\\] 0:rd_head3 1:out_buf_en\\[0\\] bit\\[13\\] 0:rd_head2 1:inbuf_empty bit\\[12\\] 0:rd_heas1 1:inbuf_half_empty bit\\[11\\] 0:blk_restart 1: inbuf_full bit\\[10\\] 0:ezip_buf_end 1:ezip_pixel_end bit\\[9\\] 0:ezip_buf_full 1:0 bit\\[8\\] 0:ezip_buf_empty 1:0 bit\\[7\\] 0:dec_buf_empty 1:0 bit\\[6\\] 0:dec_buf_full 1:para_ok bit\\[5\\] 0:dec_on 1:ezip_fuf_push bit\\[4\\] 0:ind3_on 1:copy_on bit\\[3\\] 0:ind2_on 1:bypass_on bit\\[2\\] 0:ind1_on 1:blk_clr bit\\[1\\] 0:ezip_on 1:para_val bit\\[0\\] 0:ezip_int 1:para_req"]
    #[inline(always)]
    pub fn set_db_sel(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DbSel {
    #[inline(always)]
    fn default() -> DbSel {
        DbSel(0)
    }
}
impl core::fmt::Debug for DbSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbSel")
            .field("db_sel", &self.db_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbSel {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbSel {
            db_sel: u16,
        }
        let proxy = DbSel {
            db_sel: self.db_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder destination address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DstAddr(pub u32);
impl DstAddr {
    #[doc = "ezip decoder destination address(ahb_out mode)"]
    #[inline(always)]
    pub const fn dst_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ezip decoder destination address(ahb_out mode)"]
    #[inline(always)]
    pub fn set_dst_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DstAddr {
    #[inline(always)]
    fn default() -> DstAddr {
        DstAddr(0)
    }
}
impl core::fmt::Debug for DstAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DstAddr")
            .field("dst_addr", &self.dst_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DstAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DstAddr {
            dst_addr: u32,
        }
        let proxy = DstAddr {
            dst_addr: self.dst_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder end point"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EndPoint(pub u32);
impl EndPoint {
    #[doc = "ezip end row"]
    #[inline(always)]
    pub const fn end_row(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "ezip end row"]
    #[inline(always)]
    pub fn set_end_row(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "ezip end col"]
    #[inline(always)]
    pub const fn end_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "ezip end col"]
    #[inline(always)]
    pub fn set_end_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EndPoint {
    #[inline(always)]
    fn default() -> EndPoint {
        EndPoint(0)
    }
}
impl core::fmt::Debug for EndPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EndPoint")
            .field("end_row", &self.end_row())
            .field("end_col", &self.end_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EndPoint {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EndPoint {
            end_row: u16,
            end_col: u16,
        }
        let proxy = EndPoint {
            end_row: self.end_row(),
            end_col: self.end_col(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip/aezip_frame decoder ctrl"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EzipCtrl(pub u32);
impl EzipCtrl {
    #[doc = "1:start or run 0:stop or end"]
    #[inline(always)]
    pub const fn ezip_ctrl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1:start or run 0:stop or end"]
    #[inline(always)]
    pub fn set_ezip_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for EzipCtrl {
    #[inline(always)]
    fn default() -> EzipCtrl {
        EzipCtrl(0)
    }
}
impl core::fmt::Debug for EzipCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EzipCtrl")
            .field("ezip_ctrl", &self.ezip_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EzipCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EzipCtrl {
            ezip_ctrl: bool,
        }
        let proxy = EzipCtrl {
            ezip_ctrl: self.ezip_ctrl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EzipPara(pub u32);
impl EzipPara {
    #[doc = "only used in ezip decoder mode. must select ahb in gzip/lz4 decoder mode. 0:epic 1:ahb"]
    #[inline(always)]
    pub const fn out_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "only used in ezip decoder mode. must select ahb in gzip/lz4 decoder mode. 0:epic 1:ahb"]
    #[inline(always)]
    pub fn set_out_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0:ezip or aezip 1:gzip 2:Lz4"]
    #[inline(always)]
    pub const fn mod_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "0:ezip or aezip 1:gzip 2:Lz4"]
    #[inline(always)]
    pub fn set_mod_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "no used"]
    #[inline(always)]
    pub const fn cache_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "no used"]
    #[inline(always)]
    pub fn set_cache_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "don't support ezip type2\\type4. 0:ahb 1:fifo"]
    #[inline(always)]
    pub const fn in_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "don't support ezip type2\\type4. 0:ahb 1:fifo"]
    #[inline(always)]
    pub fn set_in_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0:QSPI4 1:QSPI3"]
    #[inline(always)]
    pub const fn spi_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0:QSPI4 1:QSPI3"]
    #[inline(always)]
    pub fn set_spi_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for EzipPara {
    #[inline(always)]
    fn default() -> EzipPara {
        EzipPara(0)
    }
}
impl core::fmt::Debug for EzipPara {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EzipPara")
            .field("out_sel", &self.out_sel())
            .field("mod_sel", &self.mod_sel())
            .field("cache_en", &self.cache_en())
            .field("in_sel", &self.in_sel())
            .field("spi_sel", &self.spi_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EzipPara {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EzipPara {
            out_sel: bool,
            mod_sel: u8,
            cache_en: bool,
            in_sel: bool,
            spi_sel: bool,
        }
        let proxy = EzipPara {
            out_sel: self.out_sel(),
            mod_sel: self.mod_sel(),
            cache_en: self.cache_en(),
            in_sel: self.in_sel(),
            spi_sel: self.spi_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip frame area"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrameArea(pub u32);
impl FrameArea {
    #[doc = "AEZIP frame height"]
    #[inline(always)]
    pub const fn frame_height(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AEZIP frame height"]
    #[inline(always)]
    pub fn set_frame_height(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "AEZIP frame width"]
    #[inline(always)]
    pub const fn frame_width(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "AEZIP frame width"]
    #[inline(always)]
    pub fn set_frame_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FrameArea {
    #[inline(always)]
    fn default() -> FrameArea {
        FrameArea(0)
    }
}
impl core::fmt::Debug for FrameArea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrameArea")
            .field("frame_height", &self.frame_height())
            .field("frame_width", &self.frame_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrameArea {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrameArea {
            frame_height: u16,
            frame_width: u16,
        }
        let proxy = FrameArea {
            frame_height: self.frame_height(),
            frame_width: self.frame_width(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip frame delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrameDelay(pub u32);
impl FrameDelay {
    #[doc = "AEZIP frame delay fraction denominator"]
    #[inline(always)]
    pub const fn delay_den(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AEZIP frame delay fraction denominator"]
    #[inline(always)]
    pub fn set_delay_den(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "AEZIP frame delay fraction numerator"]
    #[inline(always)]
    pub const fn delay_num(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "AEZIP frame delay fraction numerator"]
    #[inline(always)]
    pub fn set_delay_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FrameDelay {
    #[inline(always)]
    fn default() -> FrameDelay {
        FrameDelay(0)
    }
}
impl core::fmt::Debug for FrameDelay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrameDelay")
            .field("delay_den", &self.delay_den())
            .field("delay_num", &self.delay_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrameDelay {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrameDelay {
            delay_den: u16,
            delay_num: u16,
        }
        let proxy = FrameDelay {
            delay_den: self.delay_den(),
            delay_num: self.delay_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip number of frames"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrameNum(pub u32);
impl FrameNum {
    #[doc = "number of frames"]
    #[inline(always)]
    pub const fn frame_num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "number of frames"]
    #[inline(always)]
    pub fn set_frame_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FrameNum {
    #[inline(always)]
    fn default() -> FrameNum {
        FrameNum(0)
    }
}
impl core::fmt::Debug for FrameNum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrameNum")
            .field("frame_num", &self.frame_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrameNum {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrameNum {
            frame_num: u32,
        }
        let proxy = FrameNum {
            frame_num: self.frame_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip frame area"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrameOffset(pub u32);
impl FrameOffset {
    #[doc = "AEZIP frame offset row"]
    #[inline(always)]
    pub const fn offest_row(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AEZIP frame offset row"]
    #[inline(always)]
    pub fn set_offest_row(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "AEZIP frame offset col"]
    #[inline(always)]
    pub const fn offset_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "AEZIP frame offset col"]
    #[inline(always)]
    pub fn set_offset_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FrameOffset {
    #[inline(always)]
    fn default() -> FrameOffset {
        FrameOffset(0)
    }
}
impl core::fmt::Debug for FrameOffset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrameOffset")
            .field("offest_row", &self.offest_row())
            .field("offset_col", &self.offset_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrameOffset {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrameOffset {
            offest_row: u16,
            offset_col: u16,
        }
        let proxy = FrameOffset {
            offest_row: self.offest_row(),
            offset_col: self.offset_col(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip frame size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrameSize(pub u32);
impl FrameSize {
    #[doc = "frame size"]
    #[inline(always)]
    pub const fn frame_size(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "frame size"]
    #[inline(always)]
    pub fn set_frame_size(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FrameSize {
    #[inline(always)]
    fn default() -> FrameSize {
        FrameSize(0)
    }
}
impl core::fmt::Debug for FrameSize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrameSize")
            .field("frame_size", &self.frame_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrameSize {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrameSize {
            frame_size: u32,
        }
        let proxy = FrameSize {
            frame_size: self.frame_size(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip start number of frames"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrameStart(pub u32);
impl FrameStart {
    #[doc = "start number of frames,count from 1"]
    #[inline(always)]
    pub const fn frame_start(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "start number of frames,count from 1"]
    #[inline(always)]
    pub fn set_frame_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FrameStart {
    #[inline(always)]
    fn default() -> FrameStart {
        FrameStart(0)
    }
}
impl core::fmt::Debug for FrameStart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrameStart")
            .field("frame_start", &self.frame_start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrameStart {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrameStart {
            frame_start: u32,
        }
        let proxy = FrameStart {
            frame_start: self.frame_start(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrameType(pub u32);
impl FrameType {
    #[doc = "AEZIP type of frame area renndering for this frame"]
    #[inline(always)]
    pub const fn blend_op(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AEZIP type of frame area renndering for this frame"]
    #[inline(always)]
    pub fn set_blend_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AEZIP type of frame area disposal to be done after rendering this frame"]
    #[inline(always)]
    pub const fn dispose_op(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "AEZIP type of frame area disposal to be done after rendering this frame"]
    #[inline(always)]
    pub fn set_dispose_op(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FrameType {
    #[inline(always)]
    fn default() -> FrameType {
        FrameType(0)
    }
}
impl core::fmt::Debug for FrameType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrameType")
            .field("blend_op", &self.blend_op())
            .field("dispose_op", &self.dispose_op())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrameType {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrameType {
            blend_op: u8,
            dispose_op: u8,
        }
        let proxy = FrameType {
            blend_op: self.blend_op(),
            dispose_op: self.dispose_op(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GreyPara(pub u32);
impl GreyPara {
    #[doc = "fill color parameter, when send grey data to epic . \\[23:16\\]-R,\\[15:8\\]-G,\\[7:0\\]-B"]
    #[inline(always)]
    pub const fn grey_para(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "fill color parameter, when send grey data to epic . \\[23:16\\]-R,\\[15:8\\]-G,\\[7:0\\]-B"]
    #[inline(always)]
    pub fn set_grey_para(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GreyPara {
    #[inline(always)]
    fn default() -> GreyPara {
        GreyPara(0)
    }
}
impl core::fmt::Debug for GreyPara {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GreyPara")
            .field("grey_para", &self.grey_para())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GreyPara {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GreyPara {
            grey_para: u32,
        }
        let proxy = GreyPara {
            grey_para: self.grey_para(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder _int_en"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntEn(pub u32);
impl IntEn {
    #[doc = "ezip_end _int_en"]
    #[inline(always)]
    pub const fn end_int_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ezip_end _int_en"]
    #[inline(always)]
    pub fn set_end_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "row_int_en"]
    #[inline(always)]
    pub const fn row_int_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "row_int_en"]
    #[inline(always)]
    pub fn set_row_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "row_err_en"]
    #[inline(always)]
    pub const fn row_err_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "row_err_en"]
    #[inline(always)]
    pub fn set_row_err_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "btype_err_en"]
    #[inline(always)]
    pub const fn btype_err_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "btype_err_en"]
    #[inline(always)]
    pub fn set_btype_err_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ezip_type_err_en"]
    #[inline(always)]
    pub const fn etype_err_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ezip_type_err_en"]
    #[inline(always)]
    pub fn set_etype_err_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "aezip_frame_int_en"]
    #[inline(always)]
    pub const fn aezip_int_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "aezip_frame_int_en"]
    #[inline(always)]
    pub fn set_aezip_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntEn {
    #[inline(always)]
    fn default() -> IntEn {
        IntEn(0)
    }
}
impl core::fmt::Debug for IntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntEn")
            .field("end_int_en", &self.end_int_en())
            .field("row_int_en", &self.row_int_en())
            .field("row_err_en", &self.row_err_en())
            .field("btype_err_en", &self.btype_err_en())
            .field("etype_err_en", &self.etype_err_en())
            .field("aezip_int_en", &self.aezip_int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntEn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntEn {
            end_int_en: bool,
            row_int_en: bool,
            row_err_en: bool,
            btype_err_en: bool,
            etype_err_en: bool,
            aezip_int_en: bool,
        }
        let proxy = IntEn {
            end_int_en: self.end_int_en(),
            row_int_en: self.row_int_en(),
            row_err_en: self.row_err_en(),
            btype_err_en: self.btype_err_en(),
            etype_err_en: self.etype_err_en(),
            aezip_int_en: self.aezip_int_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder int mask state"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntMask(pub u32);
impl IntMask {
    #[doc = "ezip_end _int mask sta/aezip_frame_int_mask_Sta"]
    #[inline(always)]
    pub const fn end_int_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ezip_end _int mask sta/aezip_frame_int_mask_Sta"]
    #[inline(always)]
    pub fn set_end_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "arrive row sign mask sta"]
    #[inline(always)]
    pub const fn row_int_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "arrive row sign mask sta"]
    #[inline(always)]
    pub fn set_row_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "overflow height mask sta"]
    #[inline(always)]
    pub const fn row_err_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "overflow height mask sta"]
    #[inline(always)]
    pub fn set_row_err_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "btype_err_mask sta"]
    #[inline(always)]
    pub const fn btype_err_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "btype_err_mask sta"]
    #[inline(always)]
    pub fn set_btype_err_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ezip_type_err_mask sta"]
    #[inline(always)]
    pub const fn etype_err_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ezip_type_err_mask sta"]
    #[inline(always)]
    pub fn set_etype_err_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "aezip_end_int_mask sta"]
    #[inline(always)]
    pub const fn aezip_int_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "aezip_end_int_mask sta"]
    #[inline(always)]
    pub fn set_aezip_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntMask {
    #[inline(always)]
    fn default() -> IntMask {
        IntMask(0)
    }
}
impl core::fmt::Debug for IntMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntMask")
            .field("end_int_mask", &self.end_int_mask())
            .field("row_int_mask", &self.row_int_mask())
            .field("row_err_mask", &self.row_err_mask())
            .field("btype_err_mask", &self.btype_err_mask())
            .field("etype_err_mask", &self.etype_err_mask())
            .field("aezip_int_mask", &self.aezip_int_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntMask {
            end_int_mask: bool,
            row_int_mask: bool,
            row_err_mask: bool,
            btype_err_mask: bool,
            etype_err_mask: bool,
            aezip_int_mask: bool,
        }
        let proxy = IntMask {
            end_int_mask: self.end_int_mask(),
            row_int_mask: self.row_int_mask(),
            row_err_mask: self.row_err_mask(),
            btype_err_mask: self.btype_err_mask(),
            etype_err_mask: self.etype_err_mask(),
            aezip_int_mask: self.aezip_int_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder _int_sta"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSta(pub u32);
impl IntSta {
    #[doc = "ezip_end _int_sta/aezip_frame_int_sta"]
    #[inline(always)]
    pub const fn end_int_sta(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ezip_end _int_sta/aezip_frame_int_sta"]
    #[inline(always)]
    pub fn set_end_int_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "arrive row sign sta"]
    #[inline(always)]
    pub const fn row_int_sta(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "arrive row sign sta"]
    #[inline(always)]
    pub fn set_row_int_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "overflow height sta"]
    #[inline(always)]
    pub const fn row_err_sta(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "overflow height sta"]
    #[inline(always)]
    pub fn set_row_err_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "btype_err_sta"]
    #[inline(always)]
    pub const fn btype_err_sta(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "btype_err_sta"]
    #[inline(always)]
    pub fn set_btype_err_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ezip_type_err_sta"]
    #[inline(always)]
    pub const fn etype_err_sta(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ezip_type_err_sta"]
    #[inline(always)]
    pub fn set_etype_err_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "aezip_end_int_sta"]
    #[inline(always)]
    pub const fn aezip_int_sta(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "aezip_end_int_sta"]
    #[inline(always)]
    pub fn set_aezip_int_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntSta {
    #[inline(always)]
    fn default() -> IntSta {
        IntSta(0)
    }
}
impl core::fmt::Debug for IntSta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSta")
            .field("end_int_sta", &self.end_int_sta())
            .field("row_int_sta", &self.row_int_sta())
            .field("row_err_sta", &self.row_err_sta())
            .field("btype_err_sta", &self.btype_err_sta())
            .field("etype_err_sta", &self.etype_err_sta())
            .field("aezip_int_sta", &self.aezip_int_sta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntSta {
            end_int_sta: bool,
            row_int_sta: bool,
            row_err_sta: bool,
            btype_err_sta: bool,
            etype_err_sta: bool,
            aezip_int_sta: bool,
        }
        let proxy = IntSta {
            end_int_sta: self.end_int_sta(),
            row_int_sta: self.row_int_sta(),
            row_err_sta: self.row_err_sta(),
            btype_err_sta: self.btype_err_sta(),
            etype_err_sta: self.etype_err_sta(),
            aezip_int_sta: self.aezip_int_sta(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder release bus parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NapPara(pub u32);
impl NapPara {
    #[doc = "ezip decoder release bus time 0000: not nap 0001: 16 cycle 0010: 32 cycle 0100: 64 cycle 1000: 128 cycle other: not nap"]
    #[inline(always)]
    pub const fn nap_tim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "ezip decoder release bus time 0000: not nap 0001: 16 cycle 0010: 32 cycle 0100: 64 cycle 1000: 128 cycle other: not nap"]
    #[inline(always)]
    pub fn set_nap_tim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "ezip decoder burst number 0000: 16 0001: 32 0010: 64 0100: 128 1000: 256 other: 16"]
    #[inline(always)]
    pub const fn burst_num(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "ezip decoder burst number 0000: 16 0001: 32 0010: 64 0100: 128 1000: 256 other: 16"]
    #[inline(always)]
    pub fn set_burst_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for NapPara {
    #[inline(always)]
    fn default() -> NapPara {
        NapPara(0)
    }
}
impl core::fmt::Debug for NapPara {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NapPara")
            .field("nap_tim", &self.nap_tim())
            .field("burst_num", &self.burst_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NapPara {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NapPara {
            nap_tim: u8,
            burst_num: u8,
        }
        let proxy = NapPara {
            nap_tim: self.nap_tim(),
            burst_num: self.burst_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip number of times to loop this AEZIP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PlayNum(pub u32);
impl PlayNum {
    #[doc = "number of times to loop this AEZIP,0 indicates infinite looping"]
    #[inline(always)]
    pub const fn play_num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "number of times to loop this AEZIP,0 indicates infinite looping"]
    #[inline(always)]
    pub fn set_play_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PlayNum {
    #[inline(always)]
    fn default() -> PlayNum {
        PlayNum(0)
    }
}
impl core::fmt::Debug for PlayNum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PlayNum")
            .field("play_num", &self.play_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PlayNum {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PlayNum {
            play_num: u32,
        }
        let proxy = PlayNum {
            play_num: self.play_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip start number of play"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PlayStart(pub u32);
impl PlayStart {
    #[doc = "start number of times to loop this AEZIP,,count from 1"]
    #[inline(always)]
    pub const fn play_start(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "start number of times to loop this AEZIP,,count from 1"]
    #[inline(always)]
    pub fn set_play_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PlayStart {
    #[inline(always)]
    fn default() -> PlayStart {
        PlayStart(0)
    }
}
impl core::fmt::Debug for PlayStart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PlayStart")
            .field("play_start", &self.play_start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PlayStart {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PlayStart {
            play_start: u32,
        }
        let proxy = PlayStart {
            play_start: self.play_start(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder row sign"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RowSign(pub u32);
impl RowSign {
    #[doc = "arrived row sign,ezip can generate a interrupt"]
    #[inline(always)]
    pub const fn row_sign(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "arrived row sign,ezip can generate a interrupt"]
    #[inline(always)]
    pub fn set_row_sign(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RowSign {
    #[inline(always)]
    fn default() -> RowSign {
        RowSign(0)
    }
}
impl core::fmt::Debug for RowSign {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RowSign")
            .field("row_sign", &self.row_sign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RowSign {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RowSign {
            row_sign: u16,
        }
        let proxy = RowSign {
            row_sign: self.row_sign(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Aezip sequence number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqNum(pub u32);
impl SeqNum {
    #[doc = "sequence number of the animation chunk,starting from 0"]
    #[inline(always)]
    pub const fn seq_num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "sequence number of the animation chunk,starting from 0"]
    #[inline(always)]
    pub fn set_seq_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SeqNum {
    #[inline(always)]
    fn default() -> SeqNum {
        SeqNum(0)
    }
}
impl core::fmt::Debug for SeqNum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SeqNum")
            .field("seq_num", &self.seq_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SeqNum {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SeqNum {
            seq_num: u32,
        }
        let proxy = SeqNum {
            seq_num: self.seq_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder source address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrcAddr(pub u32);
impl SrcAddr {
    #[doc = "ezip decoder source address"]
    #[inline(always)]
    pub const fn src_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ezip decoder source address"]
    #[inline(always)]
    pub fn set_src_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SrcAddr {
    #[inline(always)]
    fn default() -> SrcAddr {
        SrcAddr(0)
    }
}
impl core::fmt::Debug for SrcAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SrcAddr")
            .field("src_addr", &self.src_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SrcAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SrcAddr {
            src_addr: u32,
        }
        let proxy = SrcAddr {
            src_addr: self.src_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip source data length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrcLen(pub u32);
impl SrcLen {
    #[doc = "source data byte length only in source data fifo mode"]
    #[inline(always)]
    pub const fn src_len(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "source data byte length only in source data fifo mode"]
    #[inline(always)]
    pub fn set_src_len(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SrcLen {
    #[inline(always)]
    fn default() -> SrcLen {
        SrcLen(0)
    }
}
impl core::fmt::Debug for SrcLen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SrcLen")
            .field("src_len", &self.src_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SrcLen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SrcLen {
            src_len: u32,
        }
        let proxy = SrcLen {
            src_len: self.src_len(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ezip decoder start point"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StartPoint(pub u32);
impl StartPoint {
    #[doc = "ezip start row,count from 0"]
    #[inline(always)]
    pub const fn start_row(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "ezip start row,count from 0"]
    #[inline(always)]
    pub fn set_start_row(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "ezip start col,count from 0"]
    #[inline(always)]
    pub const fn start_col(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "ezip start col,count from 0"]
    #[inline(always)]
    pub fn set_start_col(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for StartPoint {
    #[inline(always)]
    fn default() -> StartPoint {
        StartPoint(0)
    }
}
impl core::fmt::Debug for StartPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StartPoint")
            .field("start_row", &self.start_row())
            .field("start_col", &self.start_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StartPoint {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct StartPoint {
            start_row: u16,
            start_col: u16,
        }
        let proxy = StartPoint {
            start_row: self.start_row(),
            start_col: self.start_col(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
