# FWDFF - Framework Desktop Fan Fix

Fixes a fan bug on Framework Desktop running Linux.

## What it does

On Framework Desktop systems, the fan controller's speed and color settings often get corrupted after reboot. This tool runs a cronjob that executes a small Rust program at startup to overwrite the corrupted settings with your compile-time configured values.

The program directly writes to the Embedded Controller (EC) to:
- Set fan speed to a configured percentage (default: 60%)
- Overwrite RGB controller settings with custom color configuration (if enabled)

These settings are reapplied every boot, ensuring your configuration persists despite the corruption.

## Configuration

Edit `src/main.rs` to customize:
- `FAN_SPEED_PERCENT` - Fan speed in percentage (0-100)
- `ENABLE_LED_WRITE` - Enable/disable RGB controller write (true/false)
- `RGBPAYLOAD` - Custom RGB color values as 8 zones in hex format (e.g., `RgbS { r: 0x39, g: 0xFF, b: 0x14 }`)

## Installation

### Prerequisites

<h1>Before building, install the required system packages and ensure you have a functioning rust and cargo install!</h1>

### Build and Install

```bash
./installer.sh
```

This will:
1. Build the Rust binary (release mode)
2. Install it to `/usr/bin/FWDFF`
3. Set up a cronjob to run at boot

## Removal

```bash
./remove.sh
```

## Requirements

- Linux
- Framework Desktop
