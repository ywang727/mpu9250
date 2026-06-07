# MPU9250 Rust Driver

A `#![no_std]` Rust driver for the MPU9250 9-axis Inertial Measurement Unit (IMU), supporting both **async** and **blocking** modes compatible with `embedded-hal` and `embedded-hal-async` (v1.0).

This crate is optimized for high-performance and low-latency environments (such as STM32 microcontrollers running the Embassy framework).

---

## Features

- **Typestate Builder Pattern**: Compile-time safe initialization flow (`PowerOn` -> `reset()` -> `Normal` -> `init()`), ensuring proper powerup and register configuration before reading sensor data.
- **Unified Driver Interface**:
  - **Async Mode**: Powered by `embedded-hal-async`, utilizing async/await and DMA for non-blocking I/O.
  - **Blocking Mode**: Directly accessible via `embedded-hal` traits.
  - **Hybrid / Blocking on Async**: Provides fallback synchronous blocking APIs (e.g. `agt_data_blocking()`) directly on the async driver structure, bypassing async scheduler context switches on timing-critical hot paths.
- **Efficient Contiguous Reads**: Combined 14-byte reads (`Accel + Temp + Gyro`) to minimize I2C bus overhead.
- **Optimized Performance**:
  - Async I2C read (400kHz): **~939 µs** (due to scheduling & interrupt latency).
  - Hybrid Blocking read (400kHz): **~475 µs** (near the wire speed limit of ~382 µs for 153 bits).
- **Embedded Logging**: First-class `defmt` support for low-overhead logging in embedded targets.

---

## API & Usage

### 1. Cargo.toml Setup

You can add the dependency to your `Cargo.toml` in two ways:

#### Option A: Direct Git Dependency (Recommended)
Add the driver directly from the Git repository. Since this repository is a Cargo workspace, Cargo will automatically traverse the workspace members to locate the crate named `mpu9250`:

```toml
[dependencies]
mpu9250 = { git = "https://github.com/ywang727/mpu9250.git", features = ["async", "defmt"] }
```

#### Option B: Local Path Dependency
Download or clone the repository locally and reference the driver's nested crate path:

```toml
[dependencies]
mpu9250 = { path = "path/to/mpu9250/mpu9250", features = ["async", "defmt"] }
```

Available Features:
- `async` (default): Enables `embedded-hal-async` integration.
- `defmt` (default): Enables `defmt::Format` derivation on types.

---

### 2. Initialization (Typestate Builder)

The driver requires explicit state transitions to ensure safe initialization:

```rust
use mpu9250::builder::{Mpu9250Builder, MpuConfig};

// Create the builder instance
let builder = Mpu9250Builder::new(i2c, mpu_addr, Some(MpuConfig::default()));

// Reset the device (returns a builder in the Normal state)
let builder = builder.reset().await.unwrap();
embassy_time::Timer::after_millis(100).await; // Give time for reset to settle

// Initialize registers and return the configured Mpu9250 driver instance
let mut mpu = builder.init().await.expect("Failed to initialize MPU9250");
```

---

### 3. Reading Sensor Data

#### Async Read
Reads raw register values asynchronously:

```rust
match mpu.agt_data().await {
    Ok(agt) => {
        // agt.accel: (f32, f32, f32) in g
        // agt.temp: f32 in °C
        // agt.gyro: (f32, f32, f32) in dps
        info!("Accel: {:?}", agt.accel);
    }
    Err(e) => error!("Read error: {:?}", e),
}
```

#### Hybrid Blocking Read
Reads raw register values synchronously on the async driver (requires `embedded_hal::i2c::I2c` implemented on the same I2C peripheral):

```rust
// Bypasses async scheduler to achieve lower latencies (~475 us)
match mpu.agt_data_blocking() {
    Ok(agt) => {
        info!("Accel: {:?}", agt.accel);
    }
    Err(e) => error!("Blocking read error: {:?}", e),
}
```

---

## STM32 Examples

The package contains an example project designed for the **STM32F411CE** development board using the **Embassy** framework.

### Pre-requisites

Make sure you have `probe-rs` and `cargo-binutils` installed:

```bash
cargo install probe-rs --features cli
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### Build & Flash Tools

We use `invoke` (Python task runner) to simplify building, flashing, and monitoring RTT logs.

Install `invoke`:
```bash
pip install invoke
```

Available commands (defined in `tasks.py`):
```bash
inv build            # Build the firmware (debug profile)
inv build --release  # Build the firmware (release profile)
inv run              # Flash the target and print defmt/RTT logs
inv flash            # Flash the firmware without spawning a logger
inv attach           # Attach to currently running firmware and show RTT logs
inv size             # Display ELF binary sizes
inv clean            # Clean cargo targets
```

### Running the Example

Connect your STM32F411 board via ST-Link or J-Link, then execute:

```bash
inv run
```
This will build `examples-stm32/src/bin/read_id_async.rs`, program the microcontroller, reset it, and tail the log output containing realtime accelerometer, temperature, and gyroscope data.

---

## Development Status & Testing

> [!NOTE]
> This driver is currently under active development. Below is the current testing status:

### Testing Matrix
- **I2C Interface**:
  - [x] **Async Mode** (Tested and verified on STM32F411CE with Embassy)
  - [x] **Hybrid Mode (Blocking on Async)** (Tested and verified on STM32F411CE with Embassy)
  - [ ] **Pure Synchronous Mode** (Not yet tested/verified)
- **SPI Interface**:
  - [ ] **Async Mode** (Not yet tested/verified)
  - [ ] **Synchronous Mode** (Not yet tested/verified)

### Future Roadmap
- **Builder Improvements**: The `Mpu9250Builder` currently implements a simplified configuration flow. It will be further refined to support finer-grained register control and sensor parameter options.
- **SPI Support**: Verify and complete the SPI bus communication code paths.

