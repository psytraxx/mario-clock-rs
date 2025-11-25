# Project Context: Super Mario Clock (Rust/ESP32-S3)

## Overview
This project is a **Super Mario-themed clock** firmware running on the **ESP32-S3** microcontroller. It is written in **Rust** using the **Embassy** async framework and **ESP-HAL**. The application drives a **64x64 HUB75 LED matrix** display to show the time with animated Mario graphics.

## Key Technologies
*   **Language:** Rust (2021 edition)
*   **Hardware:** ESP32-S3, HUB75 LED Matrix, PCF8563 RTC
*   **Frameworks/Crates:**
    *   `esp-hal`: Hardware Abstraction Layer
    *   `embassy`: Async runtime (`executor`, `time`, `sync`, `net`)
    *   `esp-hub75`: DMA-based LED matrix driver
    *   `embedded-graphics`: 2D drawing primitives
    *   `esp-wifi`: WiFi connectivity (used for initial NTP sync)

## Architecture
The system leverages the ESP32-S3's dual-core architecture and Embassy's async tasks for performance and responsiveness.

*   **Core 0 (Protocol/Logic):** Handles WiFi, NTP synchronization, and general application logic.
*   **Core 1 (Display):** Dedicated to the high-priority HUB75 driver task to ensure glitch-free refreshing via DMA.
*   **Inter-Task Communication:** Uses Embassy `Channel`s and `Signal`s to pass framebuffer data and synchronization events between the rendering logic and the display driver.
*   **Timekeeping:**
    1.  **Startup:** Connects to WiFi, syncs time via NTP.
    2.  **Runtime:** Disables WiFi to save power; maintains time using the I2C-connected PCF8563 RTC.

## Directory Structure
*   `src/main.rs`: Entry point, hardware setup, task spawning.
*   `src/clock.rs`: RTC and NTP logic.
*   `src/wifi_task.rs`: Network stack management.
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
*   **Setup build environment** `~/export-esp.sh`
*   **Build:** `cargo build --release`
*   **Flash & Monitor:** `cargo run --release`
*   **QEMU:** See `README.md` for specific QEMU build/run instructions.

## Coding Conventions
*   **Async First:** Prefer `async/await` for IO and tasks.
*   **No Std:** This is a `no_std` environment.
*   **Optimization:** `opt-level = "s"` is used for dev and release to fit in flash/RAM and ensure performance.
*   **Error Handling:** `unwrap()` is generally avoided in favor of proper error propagation or logging, except during initialization where failure is fatal.
