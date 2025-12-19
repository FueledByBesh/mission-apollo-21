# Mission Apollo 21 — Mini FPV Drone (Rust + STM32)

Mission Apollo 21 is a low-cost mini FPV platform built around STM32 microcontrollers, a 2.4 GHz nRF24L01 radio link, and a 3D‑printed frame. The firmware is written in Rust. The Android app is used only for telemetry via USB‑OTG — it does NOT control the drone.

TL;DR
- Flight controller: STM32F4 + BMI323 IMU, brushed motors
- Handheld controller: STM32F1 + nRF24L01 radio
- Link: 2.4G (nRF24L01) between controller and drone
- Mobile app: Android (Kotlin), USB‑OTG telemetry only (no control)
- Frame: 3D‑printed (Fusion 360 sources)

Status: Early development/MVP. Expect breaking changes.

## Repository layout
- firmware/
  - stm32f4-drone — Rust flight controller firmware (STM32F4, BMI323, motors)
  - stm32f1-controller — Rust handheld controller firmware (STM32F1, nRF24L01)
  - stm32f4-testing — Rust playground/sandboxes for STM32F4
- mobile_app/ — Android project (Kotlin) for telemetry over USB‑OTG
- frame3design/ — 3D printable frame sources (Fusion 360 / STL)
- images/ — assets and diagrams (optional)
- Components.md — components/BOM notes
- LICENSE — project license

## Hardware overview
- MCUs
  - STM32F4 (e.g., F411) on the drone (flight control, PID, sensor fusion)
  - STM32F1 (e.g., F103) in the handheld controller (joysticks, link)
- Sensors: BMI323 (gyro/accelerometer)
- Radio: nRF24L01 2.4 GHz (controller ↔ drone)
- Actuation: Brushed motors (PWM)
- Frame: 3D‑printed custom design

## Features (MVP)
- Basic attitude stabilization (PID)
- Radio control via nRF24L01 (controller → drone)
- Telemetry stream to Android over USB‑OTG (battery, link quality, etc.)
- Emergency stop on the controller

Non‑functional goals
- Latency target < 150 ms end‑to‑end
- Durable, low‑cost frame (< $100 build target)
- Simple, beginner‑friendly UX

## Building the firmware (Rust)
Prerequisites
- Rust toolchain (rustup) and Cargo
- ARM targets
  - For STM32F4: thumbv7em-none-eabihf
  - For STM32F1: thumbv7m-none-eabi
- An SWD probe (e.g., ST‑Link, J‑Link) and either probe‑rs tools or OpenOCD

Install targets
- rustup target add thumbv7em-none-eabihf
- rustup target add thumbv7m-none-eabi

Build
- Drone (STM32F4):
  - cd firmware/stm32f4-drone
  - cargo build --release --target thumbv7em-none-eabihf
- Controller (STM32F1):
  - cd firmware/stm32f1-controller
  - cargo build --release --target thumbv7m-none-eabi

Flashing (examples)
- Using probe‑rs (recommended):
  - cargo install cargo-flash
  - cargo flash --chip STM32F411RETx --release --target thumbv7em-none-eabihf  # drone (adjust chip)
  - cargo flash --chip STM32F103C8Tx --release --target thumbv7m-none-eabi     # controller (adjust chip)
- Or use OpenOCD + arm-none-eabi-gdb per your probe/board

Runtime logging
- RTT is enabled in stm32f4-drone (rtt-target). Use probe‑rs/SEGGER RTT to read logs when supported.

## Mobile app (Android, Kotlin) — telemetry only
- Path: mobile_app/
- Purpose: Display live telemetry via USB‑OTG from the controller/drone side. It is NOT a controller and does not send flight commands.
- How to run:
  - Open in Android Studio, build and run on a device with USB‑OTG support
  - Grant USB permissions when prompted
  - Connect the device via OTG cable to the hardware’s USB‑UART interface
- Expected data: battery %, signal/link quality, and other basic metrics

Important: The phone is not used for flight control. All control input comes from the STM32F1 handheld controller over 2.4G to the drone.

## Architecture (high level)
- Controller (STM32F1) reads inputs and transmits commands via nRF24L01
- Drone (STM32F4) receives commands, fuses BMI323 data, runs PID, outputs PWM to brushed motors
- Telemetry is sent back and can be forwarded to the Android app over USB‑OTG for display

## Safety
- Always test with props off first
- Keep a clear area, use an emergency stop switch
- Verify radio failsafe behavior before flight

## Roadmap
- Tune PID and filtering for BMI323
- Expand telemetry set and add logging
- Add calibration flows (IMU, radio endpoints)
- Improve frame durability and serviceability

## License
See LICENSE for details.
