# Components

This document lists hardware components used in the mission-apollo-21 project. Use the template below to add each component as a separate entry. The template is designed to capture the information engineers, purchasers, and testers need.

## How to use this template

- Copy the "Component entry" section and create one entry per component. Keep the `Part number` and `Datasheet` links up-to-date.
- Add images in the `images/` directory (or the path your repo uses) and reference them with relative links.
- Keep `Revision history` updated when details change.

---

## Component entry template

### Component name

- Part number: `MANUFACTURER-XXX-####`
- Manufacturer: Manufacturer Name
- Image: `![Component name](path/to/image.png)` (relative path)

### Summary

A short one-line description of the component and its role in the system.

### Purpose / Function in project

Explain what this component does in the overall design. Mention any subsystems that depend on it.

### Electrical characteristics

- Supply voltage(s): e.g., 3.3 V, 5 V
- Typical current consumption (idle / active): e.g., 2 mA / 150 mA
- Communication interfaces: e.g., I2C, SPI, UART, CAN
- Signal levels and tolerances: e.g., TTL, CMOS, ±10%
- Key parameters: e.g., sensitivity, gain, resolution, accuracy

### Mechanical / Physical

- Package / footprint: e.g., SOT-23, QFN-32, 0805
- Dimensions (L×W×H): mm
- Mounting: SMD / through-hole / connector
- Weight (if relevant)

### Pinout / Connections

| Pin | Name | Type | Notes |
|---:|---|---|---|
| 1 | VCC | Power | 3.3 V |
| 2 | GND | Power | Ground |
| 3 | SDA | I/O | I2C data |
| 4 | SCL | I/O | I2C clock |

> Add a schematic callout (sheet name & reference) and PCB net names if they differ from part names.

### Datasheet / Documentation

- Datasheet: [PDF link](URL)
- App notes: [link]
- Reference schematic / example circuit: `schematics/<file>.pdf`

### Suppliers and procurement

- Approved suppliers: Supplier A, Supplier B
- Typical lead time: 2–4 weeks (or vendor-specific)
- MOQ and packaging: e.g., 1000 pcs / reel
- Alternate / cross-references: Equivalent parts or drop-in replacements

### Bill of Materials (BOM)

- Standard designator(s): e.g., R12, C5, U3
- Unit cost (typical): $x.xx (QTY)
- Currency and date of price

### Firmware / Driver dependencies

- Required driver: repository/path or vendor driver name
- Configuration settings: default addresses, IRQs, mode pins
- Initialization sequence (short bullet list)

### Test / Validation

- Test procedure: how to verify the component after assembly (e.g., power-on test, communication test, calibration)
- Pass criteria: expected readings, tolerances
- Test jig / fixture references

### Safety / Compliance / Notes

- Any certifications (RoHS, CE, FCC) or special handling notes
- ESD sensitivity
- Hazardous materials

### PCB footprint / CAD

- Footprint file: `cad/footprints/<footprint_file>`
- 3D model: `cad/3d/<model_file>`

### Revision history

- v1.0 — 2025-11-12 — Initial template and example entry added by repo maintainer

---

## Example component — STM32H743ZI (example MCU)

### STM32H743ZI

- Part number: `STM32H743ZIT6`  
- Manufacturer: STMicroelectronics  
- Image: `![STM32H743ZI](images/stm32h743zi.png)`

### Summary

High-performance ARM Cortex-M7 microcontroller used as the main flight computer. Provides multiple high-speed peripherals and large RAM.

### Purpose / Function in project

Acts as the main mission controller: sensor fusion, communications, and control loops.

### Electrical characteristics

- Supply voltage(s): 1.8 V (core), 3.3 V (I/O)
- Typical current consumption: ~20 mA (standby), 100–200 mA (active, depending on peripherals)
- Communication interfaces: UART, SPI, I2C, CAN, USB, Ethernet (if used on custom carrier)

### Mechanical / Physical

- Package / footprint: LQFP-176
- Dimensions: refer to datasheet

### Pinout / Connections

| Pin | Name | Type | Notes |
|---:|---|---|---|
| 1 | VDD | Power | 3.3 V I/O |
| ... | ... | ... | ... |

### Datasheet / Documentation

- Datasheet: https://www.st.com/resource/en/datasheet/stm32h743zi.pdf
- Reference manual and application notes: ST website

### Suppliers and procurement

- Suppliers: Digi-Key, Mouser, Arrow
- Typical lead time: variable; check supplier stock

### BOM

- Designators: U1
- Unit cost: $8.00 (qty 100)

### Firmware / Driver dependencies

- STM32CubeH7 HAL/LL drivers
- Bootloader/boot configuration notes: external QSPI used for firmware storage (if applicable)

### Test / Validation

- Power-on: verify VDD, VSS rails
- Bootloader handshake over UART

### Safety / Compliance / Notes

- Handle with ESD precautions

### PCB footprint / CAD

- Footprint: `cad/footprints/STM32H743ZI.lbr`

### Revision history

- v1.0 — 2025-11-12 — Example entry added

---

## Next steps / suggestions

- Add images for each component under `images/`.
- Add a small script to export a CSV/Excel BOM from these entries (future enhancement).
- Consider adding tags (e.g., power, sensor, comms) to make components searchable.

If you'd like, I can:

1. Add tag fields and a small front-matter YAML per entry to make it machine-parsable (for BOM generation).
2. Create a helper script to generate a consolidated BOM CSV from the markdown file.

Tell me which of those you'd prefer next and I will add it.
