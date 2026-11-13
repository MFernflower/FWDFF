# FWDFF - Framework Desktop Fan Fix

## What it does

On [Framework Desktop](https://frame.work/desktop) systems running Linux, the internal fan controller's speed and RGB settings can sometimes become corrupted after reboot. This tool runs at startup and directly writes values configured at compilation to the controller to fix this issue!

## Configuration

Edit `main.rs` to change:

- `FAN_SPEED_PERCENT` - fan speed from 0-100
- `ENABLE_LED_WRITE` - enable or disable LED writes
- `ENABLE_RGB_SCRAMBLE` - enable or disable periodic RGB scrambling
- `RGB_SCRAMBLE_SEC` - scramble interval in seconds
- `RGBPAYLOAD` - the array of custom RGB values to write to the fan controller

Note: Any change requires rebuilding the binary and reinstalling the utility!

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
