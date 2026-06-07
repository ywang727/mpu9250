#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _; // defmt logger over RTT
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::i2c::I2c;
use embassy_stm32::rcc::*;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals, Config};
use mpu9250::builder::{Mpu9250Builder, MpuConfig};

bind_interrupts!(struct Irqs {
    I2C1_EV => embassy_stm32::i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => embassy_stm32::i2c::ErrorInterruptHandler<peripherals::I2C1>;
    DMA1_STREAM6 => embassy_stm32::dma::InterruptHandler<peripherals::DMA1_CH6>;
    DMA1_STREAM0 => embassy_stm32::dma::InterruptHandler<peripherals::DMA1_CH0>;
});

fn config_clock() -> Config {
    let mut config = Config::default();
    config.enable_debug_during_sleep = true;

    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll_src = PllSource::Hse;
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::Div8,
        mul: PllMul::Mul192,
        divp: Some(PllPDiv::Div2), // 96MHz sysclk (max 100MHz)
        divq: Some(PllQDiv::Div4), // 48MHz clock for USB
        divr: None,
    });
    config.rcc.sys = Sysclk::Pll1P;
    config.rcc.ahb_pre = AHBPrescaler::Div1;
    config.rcc.apb1_pre = APBPrescaler::Div2;
    config.rcc.apb2_pre = APBPrescaler::Div1;
    config.rcc.mux.clk48sel = mux::Clk48sel::Pll1Q;
    config
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("----------------------------------------");
    info!("BOOTING MPU9250 ID DEMO...");
    let p = embassy_stm32::init(config_clock());

    let mut led = Output::new(p.PA6, Level::Low, Speed::Low);

    info!("Initializing I2C1 (SCL: PB8, SDA: PB9, 400kHz)...");
    let mut i2c_config = embassy_stm32::i2c::Config::default();
    i2c_config.frequency = Hertz(400_000);

    let i2c = I2c::new(
        p.I2C1, p.PB8, p.PB9, p.DMA1_CH6, // TX DMA
        p.DMA1_CH0, // RX DMA
        Irqs, i2c_config,
    );

    // MPU9250 usually has I2C address 0x68 (AD0=GND) or 0x69 (AD0=3.3V)
    let mpu_addr = 0x69;
    info!(
        "Creating Mpu9250 driver instance with address 0x{:x}...",
        mpu_addr
    );

    let mpu9250 = Mpu9250Builder::new(i2c, mpu_addr, Some(MpuConfig::default()));
    let builder = mpu9250.reset().await.unwrap();
    embassy_time::Timer::after_millis(100).await;

    let mut mpu = builder.init().await.expect("Failed to initialize MPU9250");
    let mut count = 1;
    loop {
        info!("----------------------------------------");
        info!("READ ATTEMPT #{}", count);

        let start = embassy_time::Instant::now();
        //let _agt_res = mpu.agt_data_raw();
        let agt_res = mpu.agt_data_blocking();
        let elapsed = start.elapsed();
        //info!("agt_data_raw_blocking took: {} us", elapsed.as_micros());
        match agt_res {
            Ok(agt) => {
                info!("agt_data took: {} us", elapsed.as_micros());
                let fmt_f32_3 = |val: f32| {
                    let sign = if val < 0.0 { "-" } else { "" };
                    let val_abs = val.abs();
                    let integer = val_abs as i32;
                    let fraction = ((val_abs - integer as f32) * 1000.0 + 0.5) as i32;
                    (sign, integer, fraction % 1000)
                };
                let fmt_f32_2 = |val: f32| {
                    let sign = if val < 0.0 { "-" } else { "" };
                    let val_abs = val.abs();
                    let integer = val_abs as i32;
                    let fraction = ((val_abs - integer as f32) * 100.0 + 0.5) as i32;
                    (sign, integer, fraction % 100)
                };

                let (ax_s, ax_i, ax_f) = fmt_f32_3(agt.accel.0);
                let (ay_s, ay_i, ay_f) = fmt_f32_3(agt.accel.1);
                let (az_s, az_i, az_f) = fmt_f32_3(agt.accel.2);
                let (t_s, t_i, t_f) = fmt_f32_2(agt.temp);
                let (gx_s, gx_i, gx_f) = fmt_f32_3(agt.gyro.0);
                let (gy_s, gy_i, gy_f) = fmt_f32_3(agt.gyro.1);
                let (gz_s, gz_i, gz_f) = fmt_f32_3(agt.gyro.2);

                info!(
                    "Accel: ({}{}.{:03}, {}{}.{:03}, {}{}.{:03}) g, Temp: {}{}.{:02} C, Gyro: ({}{}.{:03}, {}{}.{:03}, {}{}.{:03}) dps",
                    ax_s, ax_i, ax_f,
                    ay_s, ay_i, ay_f,
                    az_s, az_i, az_f,
                    t_s, t_i, t_f,
                    gx_s, gx_i, gx_f,
                    gy_s, gy_i, gy_f,
                    gz_s, gz_i, gz_f
                );
            }
            Err(e) => {
                info!("Failed to read AGT data: {:?}", e);
            }
        }

        count += 1;
        embassy_time::Timer::after_secs(2).await;
        led.toggle();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    defmt::error!("Panic occurred!");
    defmt::error!("{}", defmt::Display2Format(info));
    loop {}
}
