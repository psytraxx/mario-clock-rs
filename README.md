# Super Mario Clock

A Super Mario themed clock running on an ESP32-S3 microcontroller, written in Rust. This project is inspired by the [Clockwise project](https://github.com/jnthas/clockwise) and reimplements it in Rust using modern embedded development practices.

This project uses the Embassy async runtime, ESP-HAL, and Embedded Graphics to display a clock interface inspired by Super Mario on a 64x64 HUB75 LED matrix display. The architecture consists of two independent Embassy tasks: one handling the display updates and another responsible for rendering the visuals, communicating via channels to push framebuffer data between them.

The display driving is handled by the excellent [esp-hub75](https://github.com/liebman/esp-hub75) Rust driver, which provides super-fast DMA transfers for smooth and efficient display updates with minimal CPU overhead.

![Mario Clock Running](images/mario-clock-running.jpg)
_Mario Clock in action_

![Mario Clock Hardware](images/mario-clock-hardware.jpg)
_Hardware setup with ESP32-S3 and HUB75 matrix_

## Features

- Displays the current time with Super Mario themed graphics
- Runs on ESP32-S3 hardware
- Built with Rust using the Embassy framework
- Uses Embassy pub/sub for inter-sprite communication and animation coordination
- Two independent Embassy tasks for display and rendering, connected via channels
- High-performance display updates using DMA transfers via esp-hub75 driver
- Modern Rust async/await programming model
- Efficient framebuffer updates through Embassy channels

## Dependencies

This project relies on several key Rust crates:

- `esp-hal`: Hardware Abstraction Layer for Espressif devices
- `esp-wifi`: WiFi support for ESP devices
- `embassy`: Asynchronous runtime for embedded systems
- `esp-hub75`: High-performance HUB75 LED matrix driver with DMA support
- `embedded-graphics`: 2D graphics library for embedded displays
- `heapless`: Static data structures

## Building

To build this project, you'll need the Rust ESP development environment set up with the Xtensa target:

```bash
# Install required tools
cargo install espup
espup install

# Source the export file (add this to your shell's rc file)
. $HOME/export-esp.sh

# Build the project
cargo build --target xtensa-esp32s3-none-elf --release
```

## Running

To flash the firmware to your ESP32-S3 board:

```bash
# Install flashing tool if you haven't already
cargo install espflash

# Flash and monitor (adjust port as needed)
espflash flash --monitor /dev/ttyUSB0 target/xtensa-esp32s3-none-elf/release/mario-clock-rs
```

## Project Structure

- `src/main.rs` - Main application entry point, hardware initialization and core management
- `src/clock.rs` - RTC and NTP time synchronization implementation
- `src/wifi_task.rs` - WiFi connectivity and network stack management
- `src/display/` - Display handling and rendering
  - `display_task.rs` - Main display update loop and frame management
  - `hub75_task.rs` - HUB75 matrix driver task with DMA transfers
- `src/engine/` - Core graphics engine components
  - `object.rs` - Basic object rendering primitives
  - `tile.rs` - Tile-based graphics management
- `src/mario/` - Mario-themed graphics and game logic
  - `clockface.rs` - Main clock face implementation with sprite coordination
  - `gfx/` - Graphic assets and sprites
    - `assets.rs` - Static graphic resources
    - `block.rs` - Question block implementation
    - `font.rs` - Custom font definitions
    - `mario.rs` - Mario sprite implementation

Each component is designed to work independently, communicating through Embassy channels and signals. The display system uses a double-buffering approach with DMA transfers for smooth updates, while the clock logic runs on a separate core to ensure consistent timing.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is distributed under the MIT license. See `LICENSE` for more information.

## Acknowledgments

- [Clockwise project](https://github.com/jnthas/clockwise) by jnthas for the original inspiration
- The Embassy team for their excellent embedded async runtime
- The esp-rs working group for their Rust support on ESP32 devices
