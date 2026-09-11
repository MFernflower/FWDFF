# FWDFF - Framework Desktop Fan Fix

Fixes a fan bug on Framework Desktop running Linux.

## What it does

On Framework Desktop systems, the RGB fan controller and fan settings often get corrupted after reboot. This tool runs a cronjob that executes a small Rust program at startup to overwrite the corrupted settings with your configured values.

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

Before building, install required development packages:

```bash
# Debian/Ubuntu
sudo apt-get install libudev-dev build-essential

# Fedora
sudo dnf install systemd-devel gcc

# Arch
sudo pacman -S systemd base-devel
```

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
- Rust toolchain (for building)
- Development headers for libudev
