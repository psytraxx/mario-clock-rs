# Project Context: Super Mario Clock (Rust/ESP32-S3)

## Overview
This project is a **Super Mario-themed clock** firmware running on the **ESP32-S3** microcontroller. It is written in **Rust** using the **Embassy** async framework and **ESP-HAL**. The application drives a **64x64 HUB75 LED matrix** display to show the time with animated Mario graphics.

## Key Technologies
*   **Language:** Rust (2024 edition)
*   **Hardware:** ESP32-S3, HUB75 LED Matrix, PCF8563 RTC
*   **Frameworks/Crates:**
    *   `esp-hal`: Hardware Abstraction Layer
    *   `embassy`: Async runtime (`executor`, `time`, `sync`, `net`)
    *   `esp-hub75`: DMA-based LED matrix driver
    *   `embedded-graphics`: 2D drawing primitives
    *   `esp-radio`: WiFi connectivity, pinned exactly (pre-1.0, API still shifts between betas)

## Architecture
The system leverages the ESP32-S3's dual-core architecture and Embassy's async tasks for performance and responsiveness.

*   **Core 0 (Protocol/Logic):** Handles WiFi, NTP synchronization, and general application logic.
*   **Core 1 (Display):** Runs both the high-priority HUB75 driver task (glitch-free refresh via DMA) and the lower-priority rendering task, on separate interrupt-priority executors.
*   **Inter-Task Communication:** Two framebuffers are ping-ponged between the display and HUB75 tasks via Embassy `Signal`s. There is no pub-sub layer for game logic; the clock face resolves sprite collisions with a direct call each frame.
*   **Timekeeping:**
    1.  **Startup:** Connects to WiFi, syncs time via NTP.
    2.  **Runtime:** Keeps the WiFi connection up (reconnecting on drop) and re-syncs from NTP once a day to bound RTC drift; time otherwise comes from the I2C-connected PCF8563 RTC.

## Directory Structure
*   `src/main.rs`: Entry point, hardware setup, task spawning, WiFi/NTP sync loop.
*   `src/clock.rs`: RTC and NTP logic.
*   `src/wifi_task.rs`: Network stack management (persistent connection, auto-reconnect).
*   `src/display/`:
    *   `hub75_task.rs`: Low-level driver interaction.
    *   `display_task.rs`: Frame management.
*   `src/mario/`: Game-specific logic (clock face, sprites).
    *   `gfx/`: Raw assets (sprites, fonts).
*   `src/engine/`: Reusable graphics engine (tiles, objects).

## Build & Development

### Prerequisites
*   Rust toolchain
*   `espup` (to install Espressif Rust fork/tools)
*   `espflash` (for flashing)

### Commands
*   **Setup build environment** `~/export-esp.sh` (or put the Xtensa toolchain, e.g. `~/.espressif/tools/xtensa-esp-elf/<version>/xtensa-esp-elf/bin`, on `PATH` directly if the linker isn't found)
*   **Build:** `cargo build --release`
*   **Flash & Monitor:** `cargo run --release`
*   **QEMU:** See `README.md` for specific QEMU build/run instructions.

## Coding Conventions
*   **Async First:** Prefer `async/await` for IO and tasks.
*   **No Std:** This is a `no_std` environment.
*   **Optimization:** `opt-level = "s"` is used for dev and release to fit in flash/RAM and ensure performance.
*   **Error Handling:** `unwrap()` is generally avoided in favor of proper error propagation or logging, except during initialization where failure is fatal.
*   **Stack budget:** Tasks run on fixed-size stacks (e.g. the display task's 8 KB). Any struct carrying more than a few hundred bytes (sprite buffers, framebuffers) should be constructed in place in static storage (`StaticCell`), not returned by value.
