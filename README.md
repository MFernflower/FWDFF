# FWDFF - Framework Desktop Fan Fix

## What it does

On Framework Desktop systems, the fan controller's speed and RGB settings can become corrupted after reboot. This tool runs at startup and directly writes configured at compilation values back to the embedded controller to restore stable behavior.

## Configuration

Edit `src/main.rs` to customize the behavior:

- `FAN_SPEED_PERCENT` - Target fan speed percentage from 0-100
- `ENABLE_LED_WRITE` - Enables or disables LED writes entirely
- `ENABLE_RGB_SCRAMBLE` - Enables periodic RGB payload scrambling
- `RGB_SCRAMBLE_MINUTES` - How often the RGB payload is scrambled, in minutes
- `RGBPAYLOAD` - Custom RGB values for each LED zone

Example:

```rust
const FAN_SPEED_PERCENT: u32 = 55;
const ENABLE_LED_WRITE: bool = true;
const ENABLE_RGB_SCRAMBLE: bool = true;
const RGB_SCRAMBLE_MINUTES: u64 = 5;
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

Note: changing any variables necessitates rebuilding the binary.

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
- The RGB scramble is implemented as a deterministic pseudo-randomized transformation of the payload array, so the color pattern shifts in a more dynamic, varied way over time.
- If you do not want periodic RGB motion, set `ENABLE_RGB_SCRAMBLE` to `false`.
