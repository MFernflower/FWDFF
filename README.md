# FWDFF - Framework Desktop Fan Fix

## What it does

On Framework Desktop systems, the fan controller's speed and RGB settings can become corrupted after reboot. This tool runs at startup and directly writes the expected values back to the EC to restore a stable configuration.

The program currently does the following:

- Sets the fan speed to a configured percentage
- Overwrites the RGB keyboard controller with a custom LED payload
- Optionally rotates the RGB payload periodically by shifting each color slot over time

This ensures your preferred fan and lighting behavior remains consistent even if the EC state is abnormal.

## Features

- Fan speed control via `FAN_SPEED_PERCENT`
- LED writes via `ENABLE_LED_WRITE`
- Optional RGB rotation via `ENABLE_RGB_ROTATION`
- Configurable rotation interval via `RGB_ROTATION_MINUTES`
- Custom 8-zone RGB payload through `RGBPAYLOAD`
- Shift-based color cycling, where each RGB entry is moved one slot forward in the array

## Configuration

Edit `src/main.rs` to customize the runtime behavior:

- `FAN_SPEED_PERCENT` - Target fan speed percentage from 0-100
- `ENABLE_LED_WRITE` - Enables or disables LED writes entirely
- `ENABLE_RGB_ROTATION` - Enables periodic RGB payload shifting
- `RGB_ROTATION_MINUTES` - How often the RGB payload is rotated, in minutes
- `RGBPAYLOAD` - Custom RGB values for each LED zone

Example:

```rust
const FAN_SPEED_PERCENT: u32 = 55;
const ENABLE_LED_WRITE: bool = true;
const ENABLE_RGB_ROTATION: bool = true;
const RGB_ROTATION_MINUTES: u64 = 5;
const RGBPAYLOAD: [RgbS; 8] = [
    RgbS { r: 0x39, g: 0xFF, b: 0x14 },
    RgbS { r: 0x7F, g: 0xFF, b: 0x00 },
    RgbS { r: 0xFF, g: 0xFF, b: 0x00 },
    RgbS { r: 0x00, g: 0xFF, b: 0x00 },
    RgbS { r: 0x00, g: 0xFF, b: 0x7F },
    RgbS { r: 0x00, g: 0xFF, b: 0xAA },
    RgbS { r: 0xFF, g: 0x10, b: 0xF0 },
    RgbS { r: 0x9D, g: 0x00, b: 0xFF },
];
```

Note: changing the rotation timing or enabling/disabling rotation requires rebuilding the binary.

## Installation

<h1>Before building, install the required system packages and ensure you have a functioning rust and cargo install!</h1>

### Build and install

```bash
./installer.sh
```

This will:

1. Build the Rust binary in release mode
2. Install it to `/usr/bin/FWDFF`
3. Set up a cronjob to run `/usr/bin/FWDFF` at every boot

## Removal

```bash
./remove.sh
```

## Notes

- The program writes directly to the EC and should be used with care.
- The RGB rotation is implemented as a shift operation over the payload array, so the color pattern cycles in a predictable direction over time.
- If you do not want periodic RGB motion, set `ENABLE_RGB_ROTATION` to `false`.
