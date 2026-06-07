use core::marker::PhantomData;

use crate::error::Result;
use crate::{
    regs::{AccelOut, AgtDataOut, ClkSel, GyroOut, TempOut},
    sensor::{AccelScale, Dlpf, GyroScale, Mpu9250},
};

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c as I2cTrait;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as I2cTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AccelConfig {
    pub scale: AccelScale,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GyroConfig {
    pub scale: GyroScale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MagConfig {
    by_pass: bool,
    //pub scale: MagScale,
}
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct MagConfig {
//     pub scale: MagScale,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MpuConfig {
    pub clock_source: ClkSel,
    pub dlpf: Dlpf,
    pub sample_rate_div: u8,
    pub gyro: GyroConfig,
    pub accel: AccelConfig,
    //pub mag: Option<MagConfig>,
    //pub temp_out: bool,
}

impl Default for MpuConfig {
    fn default() -> Self {
        Self {
            clock_source: ClkSel::Internal20M1,
            dlpf: Dlpf::default(),
            sample_rate_div: 1,
            gyro: GyroConfig::default(),
            accel: AccelConfig::default(),
            //mag: None,
            //temp_out: false,
        }
    }
}

impl MpuConfig {
    pub fn parse_accel(&self, raw: (i16, i16, i16)) -> (f32, f32, f32) {
        let sensitivity = match self.accel.scale {
            AccelScale::G2 => 16384.0,
            AccelScale::G4 => 8192.0,
            AccelScale::G8 => 4096.0,
            AccelScale::G16 => 2048.0,
        };
        (
            raw.0 as f32 / sensitivity,
            raw.1 as f32 / sensitivity,
            raw.2 as f32 / sensitivity,
        )
    }

    pub fn parse_gyro(&self, raw: (i16, i16, i16)) -> (f32, f32, f32) {
        let sensitivity = match self.gyro.scale {
            GyroScale::Dps250 => 131.0,
            GyroScale::Dps500 => 65.5,
            GyroScale::Dps1000 => 32.8,
            GyroScale::Dps2000 => 16.4,
        };
        (
            raw.0 as f32 / sensitivity,
            raw.1 as f32 / sensitivity,
            raw.2 as f32 / sensitivity,
        )
    }

    pub fn parse_temp(&self, raw: i16) -> f32 {
        (raw as f32) / 333.87 + 21.0
    }

    pub fn parse_accel_out(&self, out: &AccelOut) -> (f32, f32, f32) {
        self.parse_accel(out.accel_out())
    }

    pub fn parse_gyro_out(&self, out: &GyroOut) -> (f32, f32, f32) {
        self.parse_gyro(out.gyro_out())
    }

    pub fn parse_temp_out(&self, out: &TempOut) -> f32 {
        self.parse_temp(out.temp_out())
    }

    pub fn parse_agt_data(&self, agt: &AgtDataOut) -> ((f32, f32, f32), f32, (f32, f32, f32)) {
        let (ax, ay, az, temp, gx, gy, gz) = agt.agt_out();
        (
            self.parse_accel((ax, ay, az)),
            self.parse_temp(temp),
            self.parse_gyro((gx, gy, gz)),
        )
    }
}
#[cfg(feature = "async")]
pub struct PowerOn;
#[cfg(feature = "async")]
pub struct Normal;

#[cfg(feature = "async")]
pub struct Mpu9250Builder<I2C, S = PowerOn> {
    mpu: Mpu9250<I2C>,
    config: MpuConfig,
    _state: PhantomData<S>,
}

#[cfg(feature = "async")]
impl<I2C> Mpu9250Builder<I2C, PowerOn>
where
    I2C: I2cTrait,
{
    pub fn new(i2c: I2C, addr: u8, config: Option<MpuConfig>) -> Self {
        Self {
            mpu: Mpu9250::new(i2c, addr),
            config: config.unwrap_or_default(),
            _state: PhantomData,
        }
    }

    pub async fn reset(mut self) -> Result<Mpu9250Builder<I2C, Normal>> {
        self.mpu.reset().await?;
        Ok(Mpu9250Builder {
            mpu: self.mpu,
            config: self.config,
            _state: PhantomData,
        })
    }
}

#[cfg(feature = "async")]
impl<I2C> Mpu9250Builder<I2C, Normal>
where
    I2C: I2cTrait,
{
    pub async fn init(mut self) -> Result<Mpu9250<I2C>> {
        let mut pwr_mgmt_1 = crate::regs::PwrMGMT1::default();
        pwr_mgmt_1.set_clksel(self.config.clock_source);
        pwr_mgmt_1.set_sleep(false); // 确保清除 SLEEP 位以唤醒芯片
        self.mpu.write_register(pwr_mgmt_1).await?;

        // 1. 配置陀螺仪/温度传感器的 DLPF (CONFIG 寄存器)
        let mut reg_config = crate::regs::Config::default();
        reg_config.set_dlpf_cfg(self.config.dlpf as u8);
        self.mpu.write_register(reg_config).await?;

        // 2. 配置加速度计的 DLPF (ACCEL_CONFIG_2 寄存器)
        let mut accel_config_2 = crate::regs::AccelConfig2::default();
        accel_config_2.set_a_dlpf_cfg(self.config.dlpf as u8);
        accel_config_2.set_accel_fchoice_b(false); // 启用 DLPF 选项
        self.mpu.write_register(accel_config_2).await?;

        let smplrt_div = crate::regs::SmplrtDiv(self.config.sample_rate_div);
        self.mpu.write_register(smplrt_div).await?;

        let mut gyro_config = crate::regs::GyroConfig::default();
        gyro_config.set_gyro_fs_sel(self.config.gyro.scale as u8);
        self.mpu.write_register(gyro_config).await?;

        let mut accel_config_1 = crate::regs::AccelConfig1::default();
        accel_config_1.set_accel_fs_sel(self.config.accel.scale as u8);
        self.mpu.write_register(accel_config_1).await?;

        self.mpu.config = self.config;
        Ok(self.mpu)
    }
}
