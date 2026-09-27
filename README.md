# FWDFF - Framework Desktop Fan Fix

## What it does

On Framework Desktop systems running Linux, the fan controller's speed and RGB settings can become corrupted after reboot. This tool runs at startup and directly writes configured at compilation values back to the embedded controller to restore stable behavior.

## Configuration

Edit `src/main.rs` to customize the behavior:

- `FAN_SPEED_PERCENT` - Target fan speed percentage from 0-100
- `ENABLE_LED_WRITE` - Enables or disables LED writes entirely
- `ENABLE_RGB_SCRAMBLE` - Enables periodic RGB payload scrambling
- `RGB_SCRAMBLE_MINUTES` - How often the RGB payload is scrambled, in minutes
- `RGBPAYLOAD` - Custom RGB values for each LED zone

Note: changing any variables necessitates rebuilding the binary.

## Installation

<h1>Before building, install the required system packages (e.g libudev-dev) and ensure you have a functioning rust and cargo install!</h1>

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
