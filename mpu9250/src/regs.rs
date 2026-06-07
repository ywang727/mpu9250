use bitfield::bitfield;

pub const REG_SELF_TEST_GYRO_START: u8 = 0x0;
pub const REG_SELF_TEST_GYRO_LEN: u8 = 3;

pub const REG_SELF_TEST_ACCEL_START: u8 = 0xd;
pub const REG_SELF_TEST_ACCEL_LEN: u8 = 3;

pub const REG_GRAVITY_OFFSET_START: u8 = 0x13;
pub const REG_GRAVITY_OFFSET_LEN: u8 = 6;

pub const REG_SMPLRT_DIV: u8 = 0x19;
pub const REG_CONFIG: u8 = 0x1a;
pub const REG_GYRO_CONFIG: u8 = 0x1b;

pub const REG_ACCEL_CONFIG_1: u8 = 0x1c;
pub const REG_ACCEL_CONFIG_2: u8 = 0x1d;

pub const REG_LP_ACCEL_ODR: u8 = 0x1e;
pub const REG_WOM_THR: u8 = 0x1f;
pub const REG_FIFO_EN: u8 = 0x23;
pub const REG_I2C_MST_CTRL: u8 = 0x24;
pub const REG_I2C_SLV0_ADDR: u8 = 0x25;
pub const REG_I2C_SLV0_REG: u8 = 0x26;
pub const REG_I2C_SLV0_CTRL: u8 = 0x27;
pub const REG_I2C_SLV1_ADDR: u8 = 0x28;
pub const REG_I2C_SLV1_REG: u8 = 0x29;
pub const REG_I2C_SLV1_CTRL: u8 = 0x2a;
pub const REG_I2C_SLV2_ADDR: u8 = 0x2b;
pub const REG_I2C_SLV2_REG: u8 = 0x2c;
pub const REG_I2C_SLV2_CTRL: u8 = 0x2d;
pub const REG_I2C_SLV3_ADDR: u8 = 0x2e;
pub const REG_I2C_SLV3_REG: u8 = 0x2f;
pub const REG_I2C_SLV3_CTRL: u8 = 0x30;

pub const REG_I2C_SLV4_ADDR: u8 = 0x31;
pub const REG_I2C_SLV4_REG: u8 = 0x32;
pub const REG_I2C_SLV4_DO: u8 = 0x33;
pub const REG_I2C_SLV4_CTRL: u8 = 0x34;
pub const REG_I2C_SLV4_DI: u8 = 0x35;
pub const REG_I2C_MST_STATUS: u8 = 0x36;

pub const REG_INT_PIN_CFG: u8 = 0x37;
pub const REG_INT_ENABLE: u8 = 0x38;
pub const REG_INT_STATUS: u8 = 0x3a;
pub const REG_ACCEL_OUT_START: u8 = 0x3b;
pub const REG_ACCEL_OUT_LEN: u8 = 0x06;
pub const REG_TEMP_OUT_START: u8 = 0x41;
pub const REG_TEMP_OUT_LEN: u8 = 0x02;
pub const REG_GYRO_OUT_START: u8 = 0x43;
pub const REG_GYRO_OUT_LEN: u8 = 0x06;

pub const REG_EXT_SENSOR_DATA_START: u8 = 0x49;
pub const REG_EXT_SENSOR_DATA_LEN: u8 = 24;

pub const REG_I2C_SLV0_DO: u8 = 0x63;
pub const REG_I2C_SLV1_DO: u8 = 0x64;
pub const REG_I2C_SLV2_DO: u8 = 0x65;
pub const REG_I2C_SLV3_DO: u8 = 0x66;

pub const REG_I2C_MST_DELAY_CTRL: u8 = 0x67;
pub const REG_SIGNAL_PATH_RESET: u8 = 0x68;
pub const REG_MOT_DETECT_CTRL: u8 = 0x69;
pub const REG_USER_CTRL: u8 = 0x6a;
pub const REG_PWR_MGMT_1: u8 = 0x6b;
pub const REG_PWR_MGMT_2: u8 = 0x6c;

pub const REG_FIFO_COUNT_START: u8 = 0x72;
pub const REG_FIFO_COUNT_LEN: u8 = 2;
pub const REG_FIFO_R_W: u8 = 0x74;

pub const REG_WHO_AM_I: u8 = 0x75;

pub const REG_ACCEL_OFFSET_START: u8 = 0x77;
pub const REG_ACCEL_OFFSET_LEN: u8 = 2;

pub const REG_XA_OFFSET_START: u8 = 0x77;
pub const REG_XA_OFFSET_LEN: u8 = 2;
pub const REG_YA_OFFSET_START: u8 = 0x7A;
pub const REG_YA_OFFSET_LEN: u8 = 2;
pub const REG_ZA_OFFSET_START: u8 = 0x7D;
pub const REG_ZA_OFFSET_LEN: u8 = 2;

//=====================================================================

pub trait Register: Copy {
    const ADDRESS: u8;
    fn from_u8(val: u8) -> Self;
    fn to_u8(self) -> u8;
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct SelfTestGyro(pub [u8; 3]); // 0-2
impl SelfTestGyro {
    pub fn self_test_gyro(&self) -> (i8, i8, i8) {
        (self.0[0] as i8, self.0[1] as i8, self.0[2] as i8)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct SelfTestAccel(pub [u8; 3]); // 0xD-0xf
impl SelfTestAccel {
    pub fn self_test_accel(&self) -> (i8, i8, i8) {
        (self.0[0] as i8, self.0[1] as i8, self.0[2] as i8)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct GravityOffset(pub [u8; 6]); // 0x19-0x24
impl GravityOffset {
    pub fn gravity_offset(&self) -> (i16, i16, i16) {
        (
            (self.0[0] as i16) << 8 | (self.0[1] as i16),
            (self.0[2] as i16) << 8 | (self.0[3] as i16),
            (self.0[4] as i16) << 8 | (self.0[5] as i16),
        )
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct Config(u8);
    impl Debug;
    pub bool, fifo_mode, set_fifo_mode: 6;
    pub u8, ext_sync_set, set_ext_sync_set: 5, 3;
    pub u8, dlpf_cfg, set_dlpf_cfg: 2, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct GyroConfig(u8);
    impl Debug;
    pub bool, xgyro_ct_en, set_xgyro_ct_en: 7;
    pub bool, ygyro_ct_en, set_ygyro_ct_en: 6;
    pub bool, zgyro_ct_en, set_zgyro_ct_en: 5;
    pub u8, gyro_fs_sel, set_gyro_fs_sel: 4, 3;
    pub u8, fchoice_b, set_fchoice_b: 1, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccelConfig1(u8);
    impl Debug;
    pub bool, ax_st_en, set_ax_st_en: 7;
    pub bool, ay_st_en, set_ay_st_en: 6;
    pub bool, az_st_en, set_az_st_en: 5;
    pub u8, accel_fs_sel, set_accel_fs_sel: 4, 3;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccelConfig2(u8);
    impl Debug;
    pub bool, accel_fchoice_b, set_accel_fchoice_b: 3;
    pub u8, a_dlpf_cfg, set_a_dlpf_cfg: 2, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct LpAccelOdr(u8);
    impl Debug;
    pub u8, lposc_clksel, set_lposc_clksel: 3, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct WomThr(u8);
    impl Debug;
    pub u8, wom_threshold, set_wom_threshold: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FifoEn(u8);
    impl Debug;
    pub bool, temp_fifo_en, set_temp_fifo_en: 7;
    pub bool, gyro_xout, set_gyro_xout: 6;
    pub bool, gyro_yout, set_gyro_yout: 5;
    pub bool, gyro_zout, set_gyro_zout: 4;
    pub bool, accel, set_accel: 3;
    pub bool, slv2, set_slv2: 2;
    pub bool, slv1, set_slv1: 1;
    pub bool, slv0, set_slv0: 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CMstCtrl(u8);
    impl Debug;
    pub bool, mult_mst_en, set_mult_mst_en: 7;
    pub bool, wait_for_es, set_wait_for_es: 6;
    pub bool, slv_3_fifo_en, set_slv_3_fifo_en: 5;
    pub bool, i2c_mst_p_nsr, set_i2c_mst_p_nsr: 4;
    pub u8, i2c_mst_clk, set_i2c_mst_clk: 3, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv0Addr(u8);
    impl Debug;
    pub bool, i2c_slv0_rnw, set_i2c_slv0_rnw: 7;
    pub u8, i2c_id_0, set_i2c_id_0: 6, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv0Reg(u8);
    impl Debug;
    pub u8, i2c_slv0_reg, set_i2c_slv0_reg: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv0Ctrl(u8);
    impl Debug;
    pub bool, i2c_slv0_en, set_i2c_slv0_en: 7;
    pub bool, i2c_slv0_byte_sw, set_i2c_slv0_byte_sw: 6;
    pub bool, i2c_slv0_reg_dis, set_i2c_slv0_reg_dis: 5;
    pub bool, i2c_slv0_grp, set_i2c_slv0_grp: 4;
    pub u8, i2c_slv0_leng, set_i2c_slv0_leng: 3, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv1Addr(u8);
    impl Debug;
    pub bool, i2c_slv1_rnw, set_i2c_slv1_rnw: 7;
    pub u8, i2c_id_1, set_i2c_id_1: 6, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv1Reg(u8);
    impl Debug;
    pub u8, i2c_slv1_reg, set_i2c_slv1_reg: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv1Ctrl(u8);
    impl Debug;
    pub bool, i2c_slv1_en, set_i2c_slv1_en: 7;
    pub bool, i2c_slv1_byte_sw, set_i2c_slv1_byte_sw: 6;
    pub bool, i2c_slv1_reg_dis, set_i2c_slv1_reg_dis: 5;
    pub bool, i2c_slv1_grp, set_i2c_slv1_grp: 4;
    pub u8, i2c_slv1_leng, set_i2c_slv1_leng: 3, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv2Addr(u8);
    impl Debug;
    pub bool, i2c_slv2_rnw, set_i2c_slv2_rnw: 7;
    pub u8, i2c_id_2, set_i2c_id_2: 6, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv2Reg(u8);
    impl Debug;
    pub u8, i2c_slv2_reg, set_i2c_slv2_reg: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv2Ctrl(u8);
    impl Debug;
    pub bool, i2c_slv2_en, set_i2c_slv2_en: 7;
    pub bool, i2c_slv2_byte_sw, set_i2c_slv2_byte_sw: 6;
    pub bool, i2c_slv2_reg_dis, set_i2c_slv2_reg_dis: 5;
    pub bool, i2c_slv2_grp, set_i2c_slv2_grp: 4;
    pub u8, i2c_slv2_leng, set_i2c_slv2_leng: 3, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv3Addr(u8);
    impl Debug;
    pub bool, i2c_slv3_rnw, set_i2c_slv3_rnw: 7;
    pub u8, i2c_id_3, set_i2c_id_3: 6, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv3Reg(u8);
    impl Debug;
    pub u8, i2c_slv3_reg, set_i2c_slv3_reg: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv3Ctrl(u8);
    impl Debug;
    pub bool, i2c_slv3_en, set_i2c_slv3_en: 7;
    pub bool, i2c_slv3_byte_sw, set_i2c_slv3_byte_sw: 6;
    pub bool, i2c_slv3_reg_dis, set_i2c_slv3_reg_dis: 5;
    pub bool, i2c_slv3_grp, set_i2c_slv3_grp: 4;
    pub u8, i2c_slv3_leng, set_i2c_slv3_leng: 3, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv4Addr(u8);
    impl Debug;
    pub bool, i2c_slv4_rnw, set_i2c_slv4_rnw: 7;
    pub u8, i2c_id_4, set_i2c_id_4: 6, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv4Reg(u8);
    impl Debug;
    pub u8, i2c_slv4_reg, set_i2c_slv4_reg: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CSlv4Ctrl(u8);
    impl Debug;
    pub bool, i2c_slv4_en, set_i2c_slv4_en: 7;
    pub bool, slv4_done_int_en, set_slv4_done_int_en: 6;
    pub bool, i2c_slv4_reg_dis, set_i2c_slv4_reg_dis: 5;
    pub u8, i2c_mst_dly, set_i2c_mst_dly: 4, 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CMstStatus(u8);
    impl Debug;
    pub bool, pass_through, _: 7;
    pub bool, i2c_slv4_done, _: 6;
    pub bool, i2c_lost_arb, _: 5;
    pub bool, i2c_slv4_nack, _: 4;
    pub bool, i2c_slv3_nack, _: 3;
    pub bool, i2c_slv2_nack, _: 2;
    pub bool, i2c_slv1_nack, _: 1;
    pub bool, i2c_slv0_nack, _: 0;
}
bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct IntPinCfg(u8);
    impl Debug;
    pub bool, actl, set_actl: 7;
    pub bool, open, set_open: 6;
    pub bool, latch_int_en, set_latch_int_en: 5;
    pub bool, int_anyrd_2clear, set_int_anyrd_2clear: 4;
    pub bool, actl_fsync, set_actl_fsync: 3;
    pub bool, fsync_int_mode_en, set_fsync_int_mode_en: 2;
    pub bool, bypass_en, set_bypass_en: 1;
}
bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct IntEnable(u8);
    impl Debug;
    pub bool, wom_en, set_wom_en: 6;
    pub bool, fifo_oflow_en, set_fifo_oflow_en: 4;
    pub bool, fsync_int_en, set_fsync_int_en: 3;
    pub bool, raw_rdy_en, set_raw_rdy_en: 0;
}
bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct IntStatus(u8);
    impl Debug;
    pub bool, wom_int, _: 6;
    pub bool, fifo_oflow_int, _: 4;
    pub bool, fsync_int, _: 3;
    pub bool, raw_data_rdy_int, _: 0;
}
//=====================================================================
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct AgtDataOut(pub [u8; 14]);
impl AgtDataOut {
    pub fn agt_out(&self) -> (i16, i16, i16, i16, i16, i16, i16) {
        (
            (((self.0[0] as u16) << 8) | (self.0[1] as u16)) as i16,
            (((self.0[2] as u16) << 8) | (self.0[3] as u16)) as i16,
            (((self.0[4] as u16) << 8) | (self.0[5] as u16)) as i16,
            (((self.0[6] as u16) << 8) | (self.0[7] as u16)) as i16,
            (((self.0[8] as u16) << 8) | (self.0[9] as u16)) as i16,
            (((self.0[10] as u16) << 8) | (self.0[11] as u16)) as i16,
            (((self.0[12] as u16) << 8) | (self.0[13] as u16)) as i16,
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct AccelOut(pub [u8; 6]);
impl AccelOut {
    pub fn accel_out(&self) -> (i16, i16, i16) {
        (
            (((self.0[0] as u16) << 8) | (self.0[1] as u16)) as i16,
            (((self.0[2] as u16) << 8) | (self.0[3] as u16)) as i16,
            (((self.0[4] as u16) << 8) | (self.0[5] as u16)) as i16,
        )
    }
}
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct TempOut(pub [u8; 2]);
impl TempOut {
    pub fn temp_out(&self) -> i16 {
        (((self.0[0] as u16) << 8) | (self.0[1] as u16)) as i16
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct GyroOut(pub [u8; 6]);
impl GyroOut {
    pub fn gyro_out(&self) -> (i16, i16, i16) {
        (
            (((self.0[0] as u16) << 8) | (self.0[1] as u16)) as i16,
            (((self.0[2] as u16) << 8) | (self.0[3] as u16)) as i16,
            (((self.0[4] as u16) << 8) | (self.0[5] as u16)) as i16,
        )
    }
}

////
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct ExtSensData(pub [u8; 24]);
impl ExtSensData {
    pub fn ext_sens_data(&self) -> [u8; 24] {
        self.0
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct I2CMstDelayCtrl(u8);
    impl Debug;
    pub bool, delay_es_shadow, set_delay_es_shadow: 7;
    pub bool, i2c_slv4_dly_en, set_i2c_slv4_dly_en: 4;
    pub bool, i2c_slv3_dly_en, set_i2c_slv3_dly_en: 3;
    pub bool, i2c_slv2_dly_en, set_i2c_slv2_dly_en: 2;
    pub bool, i2c_slv1_dly_en, set_i2c_slv1_dly_en: 1;
    pub bool, i2c_slv0_dly_en, set_i2c_slv0_dly_en: 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SignalPathReset(u8);
    impl Debug;
    pub bool, gyro_rst, set_gyro_rst: 2;
    pub bool, accel_rst, set_accel_rst: 1;
    pub bool, temp_rst, set_temp_rst: 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MotDetectCtrl(u8);
    impl Debug;
    pub bool, accel_intel_en, set_accel_intel_en: 7;
    pub bool, accel_intel_mode, set_accel_intel_mode: 6;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct UserCtrl(u8);
    impl Debug;
    pub bool, fifo_en, set_fifo_en: 6;
    pub bool, i2c_mst_en, set_i2c_mst_en: 5;
    pub bool, i2c_if_dis, set_i2c_if_dis: 4;
    pub bool, fifo_rst, set_fifo_rst: 2;
    pub bool, i2c_mst_rst, set_i2c_mst_rst: 1;
    pub bool, sig_cond_rst, set_sig_cond_rst: 0;
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PwrMGMT1(u8);
    impl Debug;
    pub bool, h_reset, set_h_reset: 7;
    pub bool, sleep, set_sleep: 6;
    pub bool, cycle, set_cycle: 5;
    pub bool, gyro_standby, set_gyro_standby: 4;
    pub bool, pd_ptat, set_pd_ptat: 3;
    pub u8, from into ClkSel, clksel, set_clksel: 2, 0;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClkSel {
    Internal20M1 = 0,
    AutoSelect1 = 1,
    AutoSelect2 = 2,
    AutoSelect3 = 3,
    AutoSelect4 = 4,
    AutoSelect5 = 5,
    Internal20M2 = 6,
    Stop = 7,
}

impl Into<u8> for ClkSel {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for ClkSel {
    fn from(val: u8) -> Self {
        match val {
            0 => ClkSel::Internal20M1,
            1 => ClkSel::AutoSelect1,
            2 => ClkSel::AutoSelect2,
            3 => ClkSel::AutoSelect3,
            4 => ClkSel::AutoSelect4,
            5 => ClkSel::AutoSelect5,
            6 => ClkSel::Internal20M2,
            7 => ClkSel::Stop,
            _ => unreachable!("Invalid ClkSel value: {}", val),
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PwrMGMT2(u8);
    impl Debug;
    pub bool, dis_xa, set_dis_xa: 5;
    pub bool, dis_ya, set_dis_ya: 4;
    pub bool, dis_za, set_dis_za: 3;
    pub bool, dis_xg, set_dis_xg: 2;
    pub bool, dis_yg, set_dis_yg: 1;
    pub bool, dis_zg, set_dis_zg: 0;
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct FifoCount(pub [u8; 2]);
impl FifoCount {
    pub fn fifo_count(&self) -> u16 {
        ((self.0[0] as u16) & 0x1f) << 8 | self.0[1] as u16
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct AccelOffset(pub [u8; 6]);
impl AccelOffset {
    pub fn accel_offset(&self) -> (i16, i16, i16) {
        (
            (((self.0[0] as u16) << 8 | (self.0[1] as u16)) as i16) >> 1,
            (((self.0[2] as u16) << 8 | (self.0[3] as u16)) as i16) >> 1,
            (((self.0[4] as u16) << 8 | (self.0[5] as u16)) as i16) >> 1,
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct SmplrtDiv(pub u8);

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct FifoRW(pub u8);

macro_rules! impl_register {
    ($struct_name:ident, $addr_const:ident) => {
        impl Register for $struct_name {
            const ADDRESS: u8 = $addr_const;
            #[inline]
            fn from_u8(val: u8) -> Self {
                $struct_name(val)
            }
            #[inline]
            fn to_u8(self) -> u8 {
                self.0
            }
        }
    };
}

impl_register!(SmplrtDiv, REG_SMPLRT_DIV);
impl_register!(Config, REG_CONFIG);
impl_register!(GyroConfig, REG_GYRO_CONFIG);
impl_register!(AccelConfig1, REG_ACCEL_CONFIG_1);
impl_register!(AccelConfig2, REG_ACCEL_CONFIG_2);
impl_register!(LpAccelOdr, REG_LP_ACCEL_ODR);
impl_register!(WomThr, REG_WOM_THR);
impl_register!(FifoEn, REG_FIFO_EN);
impl_register!(I2CMstCtrl, REG_I2C_MST_CTRL);
impl_register!(I2CSlv0Addr, REG_I2C_SLV0_ADDR);
impl_register!(I2CSlv0Reg, REG_I2C_SLV0_REG);
impl_register!(I2CSlv0Ctrl, REG_I2C_SLV0_CTRL);
impl_register!(I2CSlv1Addr, REG_I2C_SLV1_ADDR);
impl_register!(I2CSlv1Reg, REG_I2C_SLV1_REG);
impl_register!(I2CSlv1Ctrl, REG_I2C_SLV1_CTRL);
impl_register!(I2CSlv2Addr, REG_I2C_SLV2_ADDR);
impl_register!(I2CSlv2Reg, REG_I2C_SLV2_REG);
impl_register!(I2CSlv2Ctrl, REG_I2C_SLV2_CTRL);
impl_register!(I2CSlv3Addr, REG_I2C_SLV3_ADDR);
impl_register!(I2CSlv3Reg, REG_I2C_SLV3_REG);
impl_register!(I2CSlv3Ctrl, REG_I2C_SLV3_CTRL);
impl_register!(I2CSlv4Addr, REG_I2C_SLV4_ADDR);
impl_register!(I2CSlv4Reg, REG_I2C_SLV4_REG);
impl_register!(I2CSlv4Ctrl, REG_I2C_SLV4_CTRL);
impl_register!(I2CMstStatus, REG_I2C_MST_STATUS);
impl_register!(IntPinCfg, REG_INT_PIN_CFG);
impl_register!(IntEnable, REG_INT_ENABLE);
impl_register!(IntStatus, REG_INT_STATUS);
impl_register!(I2CMstDelayCtrl, REG_I2C_MST_DELAY_CTRL);
impl_register!(SignalPathReset, REG_SIGNAL_PATH_RESET);
impl_register!(MotDetectCtrl, REG_MOT_DETECT_CTRL);
impl_register!(UserCtrl, REG_USER_CTRL);
impl_register!(PwrMGMT1, REG_PWR_MGMT_1);
impl_register!(PwrMGMT2, REG_PWR_MGMT_2);
impl_register!(FifoRW, REG_FIFO_R_W);
