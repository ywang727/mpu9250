use crate::error::{Error, Result};
use crate::fmt::*;
use crate::regs::*;

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::{Error as _, I2c as I2cTrait};
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as I2cTrait;

use crate::builder::MpuConfig;

#[derive(Debug)]
#[cfg(feature = "async")]
pub struct Mpu9250<I2C> {
    pub(crate) i2c: I2C,
    addr: u8,
    pub config: MpuConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GyroScale {
    Dps250 = 0,
    Dps500 = 1,
    Dps1000 = 2,
    #[default]
    Dps2000 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccelScale {
    G2 = 0,
    G4 = 1,
    #[default]
    G8 = 2,
    G16 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Dlpf {
    Hz250 = 0,
    Hz184 = 1,
    #[default]
    Hz92 = 2,
    Hz41 = 3,
    Hz20 = 4,
    Hz10 = 5,
    Hz5 = 6,
}

#[cfg(feature = "async")]
impl<I2C> Mpu9250<I2C>
where
    I2C: I2cTrait,
{
    //by default, the sensor is in shutdown state
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            config: MpuConfig::default(),
        }
    }
}

#[cfg(not(feature = "async"))]
impl<I2C, D> Mpu9250<I2C, D>
where
    I2C: I2cTrait,
    D: embedded_hal::delay::DelayNs,
{
    pub fn new(i2c: I2C, addr: u8, delay: D) -> Self {
        Self {
            i2c,
            addr,
            config: MpuConfig::default(),
            delay,
        }
    }
}

#[cfg(feature = "async")]
impl<I2C> Mpu9250<I2C>
where
    I2C: I2cTrait,
{
    pub async fn read_reg(&mut self, reg: u8) -> Result<u8> {
        let mut buf = [0u8; 1];
        let reg_buf = [reg];
        self.i2c
            .write_read(self.addr, &reg_buf, &mut buf)
            .await
            .map_err(|_| Error::I2CReadError)?;
        let val = buf[0];
        trace!("MPU9050: read reg 0x{:02x} = 0x{:02x}", reg, val);
        Ok(val)
    }

    pub async fn write_reg(&mut self, reg: u8, val: u8) -> Result<()> {
        trace!("MPU9050: write reg 0x{:02x} = 0x{:02x}", reg, val);
        let buf = [reg, val];
        self.i2c
            .write(self.addr, &buf)
            .await
            .map_err(|_| Error::I2CWriteError)
    }

    pub async fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<()> {
        let reg_buf = [reg];
        self.i2c
            .write_read(self.addr, &reg_buf, buf)
            .await
            .map_err(|_| Error::I2CReadError)?;
        trace!(
            "MPU9050: read bytes from reg 0x{:02x} (len={}): {:?}",
            reg,
            buf.len(),
            buf
        );
        Ok(())
    }

    pub async fn write_bytes(&mut self, reg: u8, buf: &[u8]) -> Result<()> {
        trace!(
            "MPU9050: write bytes to reg 0x{:02x} (len={}): {:?}",
            reg,
            buf.len(),
            buf
        );
        let mut write_buf = [0u8; 32];
        if buf.len() + 1 > write_buf.len() {
            return Err(Error::I2CWriteError);
        }
        write_buf[0] = reg;
        write_buf[1..1 + buf.len()].copy_from_slice(buf);
        self.i2c
            .write(self.addr, &write_buf[..1 + buf.len()])
            .await
            .map_err(|_| Error::I2CWriteError)
    }

    pub async fn who_am_i(&mut self) -> Result<u8> {
        self.read_reg(REG_WHO_AM_I).await
    }

    pub async fn reset(&mut self) -> Result<()> {
        self.write_reg(REG_PWR_MGMT_1, 0x80).await
    }

    pub async fn read_register<R>(&mut self) -> Result<R>
    where
        R: Register,
    {
        let val = self.read_reg(R::ADDRESS).await?;
        Ok(R::from_u8(val))
    }

    pub async fn write_register<R>(&mut self, reg: R) -> Result<()>
    where
        R: Register,
    {
        self.write_reg(R::ADDRESS, reg.to_u8()).await
    }

    // --- Data Block Registers ---
    pub async fn self_test_gyro(&mut self) -> Result<SelfTestGyro> {
        let mut buf = [0u8; 3];
        self.read_bytes(REG_SELF_TEST_GYRO_START, &mut buf).await?;
        Ok(SelfTestGyro(buf))
    }

    pub async fn self_test_accel(&mut self) -> Result<SelfTestAccel> {
        let mut buf = [0u8; 3];
        self.read_bytes(REG_SELF_TEST_ACCEL_START, &mut buf).await?;
        Ok(SelfTestAccel(buf))
    }

    pub async fn gravity_offset(&mut self) -> Result<GravityOffset> {
        let mut buf = [0u8; 6];
        self.read_bytes(REG_GRAVITY_OFFSET_START, &mut buf).await?;
        Ok(GravityOffset(buf))
    }
    pub async fn set_gravity_offset(&mut self, val: GravityOffset) -> Result<()> {
        self.write_bytes(REG_GRAVITY_OFFSET_START, &val.0).await
    }

    pub async fn accel_offset(&mut self) -> Result<AccelOffset> {
        let mut buf = [0u8; 6];
        self.read_bytes(REG_ACCEL_OFFSET_START, &mut buf).await?;
        Ok(AccelOffset(buf))
    }
    pub async fn set_accel_offset(&mut self, val: AccelOffset) -> Result<()> {
        self.write_bytes(REG_ACCEL_OFFSET_START, &val.0).await
    }

    pub async fn fifo_count(&mut self) -> Result<FifoCount> {
        let mut buf = [0u8; 2];
        self.read_bytes(REG_FIFO_COUNT_START, &mut buf).await?;
        Ok(FifoCount(buf))
    }

    pub async fn agt_data_raw(&mut self) -> Result<AgtDataOut> {
        let mut buf = [0u8; 14];
        self.read_bytes(REG_ACCEL_OUT_START, &mut buf).await?;
        Ok(AgtDataOut(buf))
    }

    pub async fn agt_data(&mut self) -> Result<AgtData> {
        let raw = self.agt_data_raw().await?;
        let (ax, ay, az, temp, gx, gy, gz) = raw.agt_out();
        Ok(AgtData {
            accel: self.config.parse_accel((ax, ay, az)),
            temp: self.config.parse_temp(temp),
            gyro: self.config.parse_gyro((gx, gy, gz)),
        })
    }

    pub fn read_bytes_blocking(&mut self, reg: u8, buf: &mut [u8]) -> Result<()>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let reg_buf = [reg];
        embedded_hal::i2c::I2c::write_read(&mut self.i2c, self.addr, &reg_buf, buf)
            .map_err(|_| Error::I2CReadError)
    }

    pub fn agt_data_raw_blocking(&mut self) -> Result<AgtDataOut>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let mut buf = [0u8; 14];
        self.read_bytes_blocking(REG_ACCEL_OUT_START, &mut buf)?;
        Ok(AgtDataOut(buf))
    }

    pub fn agt_data_blocking(&mut self) -> Result<AgtData>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let raw = self.agt_data_raw_blocking()?;
        let (ax, ay, az, temp, gx, gy, gz) = raw.agt_out();
        Ok(AgtData {
            accel: self.config.parse_accel((ax, ay, az)),
            temp: self.config.parse_temp(temp),
            gyro: self.config.parse_gyro((gx, gy, gz)),
        })
    }
    pub async fn accel_out(&mut self) -> Result<AccelOut> {
        let mut buf = [0u8; 6];
        self.read_bytes(REG_ACCEL_OUT_START, &mut buf).await?;
        Ok(AccelOut(buf))
    }

    pub async fn temp_out(&mut self) -> Result<TempOut> {
        let mut buf = [0u8; 2];
        self.read_bytes(REG_TEMP_OUT_START, &mut buf).await?;
        Ok(TempOut(buf))
    }

    pub async fn gyro_out(&mut self) -> Result<GyroOut> {
        let mut buf = [0u8; 6];
        self.read_bytes(REG_GYRO_OUT_START, &mut buf).await?;
        Ok(GyroOut(buf))
    }

    pub async fn read_gat_data(&mut self) -> Result<(GyroOut, AccelOut, TempOut)> {
        let gyro = self.gyro_out().await?;
        let accel = self.accel_out().await?;
        let temp = self.temp_out().await?;
        Ok((gyro, accel, temp))
    }

    pub async fn ext_sens_data(&mut self) -> Result<ExtSensData> {
        let mut buf = [0u8; 24];
        self.read_bytes(REG_EXT_SENSOR_DATA_START, &mut buf).await?;
        Ok(ExtSensData(buf))
    }

    pub async fn set_slave_data_out(&mut self, slave: u8, val: u8) -> Result<()> {
        let reg = match slave {
            0 => REG_I2C_SLV0_DO,
            1 => REG_I2C_SLV1_DO,
            2 => REG_I2C_SLV2_DO,
            3 => REG_I2C_SLV3_DO,
            _ => return Err(Error::I2CWriteError), // 或者是参数错误
        };
        self.write_reg(reg, val).await
    }
    pub async fn slave_data_out(&mut self, slave: u8) -> Result<u8> {
        let reg = match slave {
            0 => REG_I2C_SLV0_DO,
            1 => REG_I2C_SLV1_DO,
            2 => REG_I2C_SLV2_DO,
            3 => REG_I2C_SLV3_DO,
            _ => return Err(Error::I2CReadError),
        };
        self.read_reg(reg).await
    }

    /*
    pub async fn apply_config(&mut self, config: MpuConfig) -> Result<()> {
        let mut pwr_mgmt_1 = PwrMGMT1::default();
        pwr_mgmt_1.set_clksel(config.clock_source);
        pwr_mgmt_1.set_sleep(config.sleep);
        self.write_register(pwr_mgmt_1).await?;

        let smplrt_div = SmplrtDiv(config.sample_rate_div);
        self.write_register(smplrt_div).await?;

        let mut reg_config = Config::default();
        reg_config.set_dlpf_cfg(config.dlpf as u8);
        self.write_register(reg_config).await?;

        let mut accel_config_2 = AccelConfig2::default();
        accel_config_2.set_a_dlpf_cfg(config.dlpf as u8);
        accel_config_2.set_accel_fchoice_b(false);
        self.write_register(accel_config_2).await?;

        let mut gyro_config = GyroConfig::default();
        gyro_config.set_gyro_fs_sel(config.gyro_scale as u8);
        gyro_config.set_fchoice_b(0);
        self.write_register(gyro_config).await?;

        let mut accel_config_1 = AccelConfig1::default();
        accel_config_1.set_accel_fs_sel(config.accel_scale as u8);
        self.write_register(accel_config_1).await?;

        let mut int_pin_cfg = IntPinCfg::default();
        int_pin_cfg.set_bypass_en(config.ak8963_enable);
        self.write_register(int_pin_cfg).await?;

        let mut user_ctrl = UserCtrl::default();
        user_ctrl.set_i2c_mst_en(false);
        self.write_register(user_ctrl).await?;

        self.config = config;

        Ok(())
    }
    */
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AgtData {
    pub accel: (f32, f32, f32),
    pub temp: f32,
    pub gyro: (f32, f32, f32),
}

#[derive(Debug)]
#[cfg(not(feature = "async"))]
pub struct Mpu9250<I2C, D> {
    pub(crate) i2c: I2C,
    addr: u8,
    pub config: MpuConfig,
    pub(crate) delay: D,
}

#[cfg(not(feature = "async"))]
impl<I2C, D> Mpu9250<I2C, D>
where
    I2C: I2cTrait,
    D: embedded_hal::delay::DelayNs,
{
    pub fn read_reg(&mut self, reg: u8) -> Result<u8> {
        let mut buf = [0u8; 1];
        let mut retries = 3;

        loop {
            match self.i2c.write_read(self.addr, &[reg], &mut buf) {
                Ok(()) => {
                    let val = buf[0] as u8;
                    trace!("MPU9050: read reg 0x{:02x} = 0x{:02x}", reg, val);
                    return Ok(val);
                }
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        if e.kind() == embedded_hal::i2c::ErrorKind::Other {
                            return Err(Error::Timeout);
                        } else {
                            return Err(Error::I2CReadError);
                        }
                    }
                    self.delay.delay_ms(10);
                }
            }
        }
    }

    pub fn write_reg(&mut self, reg: u8, val: u8) -> Result<()> {
        trace!("MPU9050: write reg 0x{:02x} = 0x{:02x}", reg, val);
        let buf = [reg, val];
        let mut retries = 3;
        loop {
            match self.i2c.write(self.addr, &buf) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        if e.kind() == embedded_hal::i2c::ErrorKind::Other {
                            return Err(Error::Timeout);
                        } else {
                            return Err(Error::I2CWriteError);
                        }
                    }
                    self.delay.delay_ms(10);
                }
            }
        }
    }

    pub fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<()> {
        let mut retries = 3;
        loop {
            match self.i2c.write_read(self.addr, &[reg], buf) {
                Ok(()) => {
                    trace!(
                        "MPU9050: read bytes from reg 0x{:02x} (len={}): {:?}",
                        reg,
                        buf.len(),
                        buf
                    );
                    return Ok(());
                }
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        if e.kind() == embedded_hal::i2c::ErrorKind::Other {
                            return Err(Error::Timeout);
                        } else {
                            return Err(Error::I2CReadError);
                        }
                    }
                    self.delay.delay_ms(10);
                }
            }
        }
    }

    pub fn write_bytes(&mut self, reg: u8, buf: &[u8]) -> Result<()> {
        trace!(
            "MPU9050: write bytes to reg 0x{:02x} (len={}): {:?}",
            reg,
            buf.len(),
            buf
        );
        let mut write_buf = [0u8; 32];
        if buf.len() + 1 > write_buf.len() {
            return Err(Error::I2CWriteError);
        }
        write_buf[0] = reg;
        write_buf[1..1 + buf.len()].copy_from_slice(buf);

        let mut retries = 3;
        loop {
            match self.i2c.write(self.addr, &write_buf[..1 + buf.len()]) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        if e.kind() == embedded_hal::i2c::ErrorKind::Other {
                            return Err(Error::Timeout);
                        } else {
                            return Err(Error::I2CWriteError);
                        }
                    }
                    self.delay.delay_ms(10);
                }
            }
        }
    }

    pub fn who_am_i(&mut self) -> Result<u8> {
        self.read_reg(REG_WHO_AM_I)
    }

    pub fn read_register<R>(&mut self) -> Result<R>
    where
        R: Register,
    {
        let val = self.read_reg(R::ADDRESS)?;
        Ok(R::from_u8(val))
    }

    pub fn write_register<R>(&mut self, reg: R) -> Result<()>
    where
        R: Register,
    {
        self.write_reg(R::ADDRESS, reg.to_u8())
    }

    // --- Data Block Registers ---
    pub fn self_test_gyro(&mut self) -> Result<SelfTestGyro> {
        let mut buf = [0u8; 3];
        self.read_bytes(REG_SELF_TEST_GYRO_START, &mut buf)?;
        Ok(SelfTestGyro(buf))
    }

    pub fn self_test_accel(&mut self) -> Result<SelfTestAccel> {
        let mut buf = [0u8; 3];
        self.read_bytes(REG_SELF_TEST_ACCEL_START, &mut buf)?;
        Ok(SelfTestAccel(buf))
    }

    pub fn gravity_offset(&mut self) -> Result<GravityOffset> {
        let mut buf = [0u8; 6];
        self.read_bytes(REG_GRAVITY_OFFSET_START, &mut buf)?;
        Ok(GravityOffset(buf))
    }
    pub fn set_gravity_offset(&mut self, val: GravityOffset) -> Result<()> {
        self.write_bytes(REG_GRAVITY_OFFSET_START, &val.0)
    }

    pub fn accel_offset(&mut self) -> Result<AccelOffset> {
        let mut buf = [0u8; 6];
        self.read_bytes(REG_ACCEL_OFFSET_START, &mut buf)?;
        Ok(AccelOffset(buf))
    }
    pub fn set_accel_offset(&mut self, val: AccelOffset) -> Result<()> {
        self.write_bytes(REG_ACCEL_OFFSET_START, &val.0)
    }

    pub fn fifo_count(&mut self) -> Result<FifoCount> {
        let mut buf = [0u8; 2];
        self.read_bytes(REG_FIFO_COUNT_START, &mut buf)?;
        Ok(FifoCount(buf))
    }

    pub fn agt_data_raw(&mut self) -> Result<AgtDataOut> {
        let mut buf = [0u8; 14];
        self.read_bytes(REG_ACCEL_OUT_START, &mut buf)?;
        Ok(AgtDataOut(buf))
    }

    pub fn agt_data(&mut self) -> Result<AgtData> {
        let raw = self.agt_data_raw()?;
        let (ax, ay, az, temp, gx, gy, gz) = raw.agt_out();
        Ok(AgtData {
            accel: self.config.parse_accel((ax, ay, az)),
            temp: self.config.parse_temp(temp),
            gyro: self.config.parse_gyro((gx, gy, gz)),
        })
    }

    pub fn accel_out(&mut self) -> Result<AccelOut> {
        let mut buf = [0u8; 6];
        self.read_bytes(REG_ACCEL_OUT_START, &mut buf)?;
        Ok(AccelOut(buf))
    }

    pub fn temp_out(&mut self) -> Result<TempOut> {
        let mut buf = [0u8; 2];
        self.read_bytes(REG_TEMP_OUT_START, &mut buf)?;
        Ok(TempOut(buf))
    }

    pub fn gyro_out(&mut self) -> Result<GyroOut> {
        let mut buf = [0u8; 6];
        self.read_bytes(REG_GYRO_OUT_START, &mut buf)?;
        Ok(GyroOut(buf))
    }

    pub fn ext_sens_data(&mut self) -> Result<ExtSensData> {
        let mut buf = [0u8; 24];
        self.read_bytes(REG_EXT_SENSOR_DATA_START, &mut buf)?;
        Ok(ExtSensData(buf))
    }

    pub fn apply_config(&mut self, config: MpuConfig) -> Result<()> {
        let mut pwr_mgmt_1 = PwrMGMT1::default();
        pwr_mgmt_1.set_clksel(config.clock_source);
        pwr_mgmt_1.set_sleep(config.sleep);
        self.write_register(pwr_mgmt_1)?;

        let smplrt_div = SmplrtDiv(config.sample_rate_div);
        self.write_register(smplrt_div)?;

        let mut reg_config = Config::default();
        reg_config.set_dlpf_cfg(config.dlpf as u8);
        self.write_register(reg_config)?;

        let mut accel_config_2 = AccelConfig2::default();
        accel_config_2.set_a_dlpf_cfg(config.dlpf as u8);
        accel_config_2.set_accel_fchoice_b(false);
        self.write_register(accel_config_2)?;

        let mut gyro_config = GyroConfig::default();
        gyro_config.set_gyro_fs_sel(config.gyro_scale as u8);
        gyro_config.set_fchoice_b(0);
        self.write_register(gyro_config)?;

        let mut accel_config_1 = AccelConfig1::default();
        accel_config_1.set_accel_fs_sel(config.accel_scale as u8);
        self.write_register(accel_config_1)?;

        let mut int_pin_cfg = IntPinCfg::default();
        int_pin_cfg.set_bypass_en(config.ak8963_enable);
        self.write_register(int_pin_cfg)?;

        let mut user_ctrl = UserCtrl::default();
        user_ctrl.set_i2c_mst_en(false);
        self.write_register(user_ctrl)?;

        self.config = config;

        Ok(())
    }
}

#[cfg(all(test, not(target_os = "none")))]
#[cfg(not(feature = "async"))]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};

    const ADDR: u8 = 0x68;

    struct DummyDelay;
    impl embedded_hal::delay::DelayNs for DummyDelay {
        fn delay_ns(&mut self, _ns: u32) {}
    }

    #[test]
    fn test_read_reg() {
        let expectations = [I2cTrans::write_read(ADDR, vec![0x75], vec![0x71])];
        let i2c = I2cMock::new(&expectations);
        let mut sensor = Mpu9250::new(i2c, ADDR, DummyDelay);

        let val = sensor.read_reg(0x75).unwrap();
        assert_eq!(val, 0x71);

        let mut i2c = sensor.i2c;
        i2c.done();
    }

    #[test]
    fn test_write_reg() {
        let expectations = [I2cTrans::write(ADDR, vec![0x6b, 0x01])];
        let i2c = I2cMock::new(&expectations);
        let mut sensor = Mpu9250::new(i2c, ADDR, DummyDelay);

        sensor.write_reg(0x6b, 0x01).unwrap();

        let mut i2c = sensor.i2c;
        i2c.done();
    }

    #[test]
    fn test_register_trait() {
        let expectations = [
            I2cTrans::write_read(ADDR, vec![0x1a], vec![0x00]),
            I2cTrans::write(ADDR, vec![0x1a, 0x40]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut sensor = Mpu9250::new(i2c, ADDR, DummyDelay);

        let mut config: Config = sensor.read_register().unwrap();
        assert_eq!(config.to_u8(), 0x00);
        config.set_fifo_mode(true); // Bit 6 = 1 -> 0x40
        sensor.write_register(config).unwrap();

        let mut i2c = sensor.i2c;
        i2c.done();
    }

    #[test]
    fn test_apply_config() {
        let config = MpuConfig::new()
            .clock_source(ClkSel::AutoSelect1)
            .gyro_scale(GyroScale::Dps500)
            .accel_scale(AccelScale::G4)
            .dlpf(Dlpf::Hz92)
            .sample_rate_div(10)
            .ak8963_enable(true)
            .sleep(false);

        let expectations = [
            I2cTrans::write(ADDR, vec![0x6b, 0x01]), // ClkSel::AutoSelect1
            I2cTrans::write(ADDR, vec![0x19, 10]),   // sample_rate_div = 10
            I2cTrans::write(ADDR, vec![0x1a, 0x02]), // Dlpf::Hz92
            I2cTrans::write(ADDR, vec![0x1d, 0x02]), // AccelConfig2
            I2cTrans::write(ADDR, vec![0x1b, 0x08]), // GyroConfig
            I2cTrans::write(ADDR, vec![0x1c, 0x08]), // AccelConfig1
            I2cTrans::write(ADDR, vec![0x37, 0x02]), // IntPinCfg
            I2cTrans::write(ADDR, vec![0x6a, 0x00]), // UserCtrl
        ];

        let i2c = I2cMock::new(&expectations);
        let mut sensor = Mpu9250::new(i2c, ADDR, DummyDelay);
        sensor.apply_config(config).unwrap();

        let mut i2c = sensor.i2c;
        i2c.done();
    }
}

#[cfg(all(test, not(target_os = "none")))]
#[cfg(feature = "async")]
mod async_tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
    use futures::executor::block_on;

    const ADDR: u8 = 0x68;

    #[test]
    fn test_read_reg_async() {
        let expectations = [I2cTrans::write_read(ADDR, vec![0x75], vec![0x71])];
        let i2c = I2cMock::new(&expectations);
        let mut sensor = Mpu9250::new(i2c, ADDR);

        let val = block_on(sensor.read_reg(0x75)).unwrap();
        assert_eq!(val, 0x71);

        let mut i2c = sensor.i2c;
        i2c.done();
    }

    #[test]
    fn test_register_trait_async() {
        let expectations = [
            I2cTrans::write_read(ADDR, vec![0x1a], vec![0x00]),
            I2cTrans::write(ADDR, vec![0x1a, 0x40]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut sensor = Mpu9250::new(i2c, ADDR);

        // let mut config: Config = block_on(sensor.read_register()).unwrap();
        let mut config = block_on(sensor.read_register::<Config>()).unwrap();
        assert_eq!(config.to_u8(), 0x00);
        config.set_fifo_mode(true); // Bit 6 = 1 -> 0x40
        block_on(sensor.write_register(config)).unwrap();

        let mut i2c = sensor.i2c;
        i2c.done();
    }

    #[test]
    fn test_apply_config_async() {
        let config = MpuConfig::new()
            .clock_source(ClkSel::AutoSelect1)
            .gyro_scale(GyroScale::Dps500)
            .accel_scale(AccelScale::G4)
            .dlpf(Dlpf::Hz92)
            .sample_rate_div(10)
            .bypass_i2c(true)
            .sleep(false);

        let expectations = [
            I2cTrans::write(ADDR, vec![0x6b, 0x01]), // ClkSel::AutoSelect1
            I2cTrans::write(ADDR, vec![0x19, 10]),   // sample_rate_div = 10
            I2cTrans::write(ADDR, vec![0x1a, 0x02]), // Dlpf::Hz92
            I2cTrans::write(ADDR, vec![0x1d, 0x02]), // AccelConfig2
            I2cTrans::write(ADDR, vec![0x1b, 0x08]), // GyroConfig
            I2cTrans::write(ADDR, vec![0x1c, 0x08]), // AccelConfig1
            I2cTrans::write(ADDR, vec![0x37, 0x02]), // IntPinCfg
            I2cTrans::write(ADDR, vec![0x6a, 0x00]), // UserCtrl
        ];

        let i2c = I2cMock::new(&expectations);
        let mut sensor = Mpu9250::new(i2c, ADDR);
        block_on(sensor.apply_config(config)).unwrap();

        let mut i2c = sensor.i2c;
        i2c.done();
    }
}
