# EvolGenesis — Development Environment Setup (macOS)

This guide walks you through setting up a complete ESP32 firmware development
environment on macOS. It covers both tracks:

* **Track A (Recommended First):** Arduino/PlatformIO — verify hardware works in ~30 min.
* **Track B (evoGenesis firmware):** Rust + `esp-idf-hal` — the real project stack.

---

## Prerequisites (your current state)

| Tool | Status |
|---|---|
| macOS | ✅ |
| Homebrew | ✅ installed |
| Rust + Cargo | ✅ `rustup 1.29.0` / `cargo 1.96.0` |
| Python 3 | ✅ `3.9.6` |
| ESP32 connected | ✅ `/dev/cu.usbserial-57860055921` |
| cmake / ninja | ❌ need to install |
| PlatformIO | ❌ need to install |

---

## Step 1 — Install Missing Build Dependencies

Run these once. They are required for both tracks.

```bash
# Build tools
brew install cmake ninja

# Python build dep (needed by esp-idf and espflash)
pip3 install --user --upgrade pip setuptools

# esptool — flash firmware over USB (works for both tracks)
pip3 install --user esptool
```

Verify:
```bash
cmake --version    # expect 3.x
ninja --version    # expect 1.x
esptool.py version # expect 4.x
```

---

## Step 2 — Identify Your ESP32 Variant

Different boards need different targets. Find your board's chip type:

```bash
# With ESP32 plugged in:
esptool.py --port /dev/cu.usbserial-57860055921 chip_id
```

| Output chip | Your board | Rust target |
|---|---|---|
| `ESP32` | Original ESP32 (WROOM, WROVER) | `xtensa-esp32-espidf` |
| `ESP32-S3` | ESP32-S3 | `xtensa-esp32s3-espidf` |
| `ESP32-C3` | ESP32-C3 (RISC-V) | `riscv32imc-esp-espidf` |
| `ESP32-C6` | ESP32-C6 (RISC-V) | `riscv32imac-esp-espidf` |

> **Note for evoGenesis v0.1:** The original **ESP32 (Xtensa)** is recommended for
> the initial rover platform. It has the most community support and example code.

---

## Track A — PlatformIO + Arduino (Quick Hardware Verification)

Use this to make sure your ESP32 is alive before diving into Rust.

### Install PlatformIO

```bash
pip3 install --user platformio
# Add to PATH (add this line to your ~/.zshrc too)
export PATH="$HOME/.local/bin:$PATH"
source ~/.zshrc
```

### Create a blink project

```bash
mkdir -p ~/temp/esp32-blink && cd ~/temp/esp32-blink
pio init --board esp32dev    # use esp32-s3-devkitc-1 for S3, etc.
```

Open `src/main.cpp` and replace its contents with:

```cpp
#include <Arduino.h>

// GPIO 2 is the onboard LED on most ESP32 devkit boards
#define LED_PIN 2

void setup() {
    Serial.begin(115200);
    pinMode(LED_PIN, OUTPUT);
    Serial.println("EvolGenesis v0.1 — Hardware Bring-up OK");
}

void loop() {
    digitalWrite(LED_PIN, HIGH);
    delay(500);
    digitalWrite(LED_PIN, LOW);
    delay(500);
}
```

### Flash and monitor

```bash
# Replace the port with your actual port
pio run --target upload --upload-port /dev/cu.usbserial-57860055921
pio device monitor --port /dev/cu.usbserial-57860055921 --baud 115200
```

You should see the LED blinking and the serial output printing the welcome message.

---

## Track B — Rust Firmware (EvolGenesis Stack)

This is the real firmware stack for the project.

### 1. Install the Espressif Rust toolchain

Espressif maintains a fork of the Rust compiler for Xtensa (ESP32) chips,
plus standard RISC-V support for ESP32-C3/C6.

```bash
# Install espup — the official esp rust installer (like rustup, but for esp)
cargo install espup

# Install the Espressif Rust toolchain (this downloads ~2GB, be patient)
espup install

# IMPORTANT: Source the export file in every new terminal session
# Add this line to your ~/.zshrc:
echo '. $HOME/export-esp.sh' >> ~/.zshrc
source ~/export-esp.sh
```

Verify:
```bash
rustup toolchain list
# You should now see: esp (active)
```

### 2. Install `cargo-generate` and `espflash`

```bash
cargo install cargo-generate espflash
```

| Tool | Purpose |
|---|---|
| `cargo-generate` | Creates a new project from ESP template |
| `espflash` | Flash firmware + monitor serial over USB |

### 3. Create your first Rust ESP32 project

```bash
cd /path/to/evoGenesis

# Use the official esp-idf template (for std support -- easier to start with)
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo

# When prompted:
# Project Name: firmware
# ESP-IDF version: v5.3.x (or latest stable)
# Target: esp32 (or your chip variant)
# STD: true
```

### 4. Explore the generated structure

```
evoGenesis/
+-- firmware/
    +-- Cargo.toml        # Dependencies
    +-- build.rs          # Build script (links esp-idf)
    +-- src/
    |   +-- main.rs       # Your entry point
    +-- .cargo/
    |   +-- config.toml   # Target + runner config
    +-- sdkconfig.defaults # ESP-IDF config
```

### 5. Write your first Rust blink

Open `firmware/src/main.rs` and use this starter:

```rust
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    // Required: link esp-idf patches
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // GPIO 2 -- onboard LED on most ESP32 devkits
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();

    println!("EvolGenesis firmware starting...");

    loop {
        led.set_high().unwrap();
        FreeRtos::delay_ms(500);
        led.set_low().unwrap();
        FreeRtos::delay_ms(500);
    }
}
```

### 6. Build and flash

```bash
cd firmware

# Build (first build is slow -- downloads esp-idf, ~5-10 min)
cargo build

# Flash to your ESP32
espflash flash target/xtensa-esp32-espidf/debug/firmware \
  --port /dev/cu.usbserial-57860055921

# Monitor serial output (Ctrl+R to reset board)
espflash monitor --port /dev/cu.usbserial-57860055921
```

---

## Troubleshooting

### Port permission denied
```bash
# Check if another app is using the port
lsof /dev/cu.usbserial-57860055921

# If using CH340/CH341 chip (common cheap clone):
# Install the driver: https://www.wch-ic.com/downloads/CH34XSER_MAC_ZIP.html
```

### Board not detected at all
1. Try a different USB cable (many cables are power-only, no data).
2. Hold the BOOT button on the board, then plug in USB, then release.
3. Try: `esptool.py --port /dev/cu.usbserial-57860055921 flash_id`

### `cargo build` fails with linker error
```bash
# Make sure you sourced the esp export script
source ~/export-esp.sh
rustup toolchain list  # should show "esp"
```

### Wrong LED pin
Refer to your board's pinout diagram. Common onboard LED pins:
* **ESP32 DevKit V1**: GPIO 2
* **ESP32-S3 DevKitC**: GPIO 38 (or no onboard LED)
* **ESP32-C3 DevKit**: GPIO 8

---

## Recommended Next Steps (evoGenesis v0.1)

- [ ] Track A: blink the LED (hardware verified)
- [ ] Track B: build and flash first Rust blink
- [ ] Add motor driver code (L298N or L293D)
- [ ] Set up Wi-Fi + WebSocket for remote control (evoGenesis v0.5 milestone)
- [ ] Connect web-dashboard to firmware over WebSocket

---

## References & Resources

- **esp-rs book** (official Rust ESP guide): https://docs.esp-rs.org/book/
- **esp-idf-hal** crate: https://github.com/esp-rs/esp-idf-hal
- **Espressif ESP32 Pinout**: https://randomnerdtutorials.com/esp32-pinout-reference-gpios/
- **espflash docs**: https://github.com/esp-rs/espflash
- **PlatformIO ESP32**: https://docs.platformio.org/en/latest/boards/espressif32/esp32dev.html
