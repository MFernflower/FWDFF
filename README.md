# FWDFF - Framework Desktop Fan Fix

## What it does

On Framework Desktop systems running Linux, the fan controller's speed and RGB settings can become corrupted after reboot. This tool runs at startup and directly writes configured at compilation value[...]

## Configuration

Edit `main.rs` to change:

- `FAN_SPEED_PERCENT` - fan speed from 0-100
- `ENABLE_LED_WRITE` - enable or disable LED writes
- `ENABLE_RGB_SCRAMBLE` - enable or disable periodic RGB scrambling
- `RGB_SCRAMBLE_SEC` - scramble interval in seconds
- `RGBPAYLOAD` - custom RGB values for each LED

Any change requires rebuilding the binary.

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
