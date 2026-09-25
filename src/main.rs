use framework_lib::chromium_ec::{CrosEc, CrosEcDriver};
use framework_lib::chromium_ec::commands::RgbS;
use framework_lib::chromium_ec::EcError;
use std::thread;
use std::time::Duration;

// Compile-time configuration variables
const FAN_SPEED_PERCENT: u32 = 55; // Set fan speed percentage (0-100)
const ENABLE_LED_WRITE: bool = true; // Set to true to enable LED write, false to disable
const ENABLE_RGB_ROTATION: bool = true; // Set to true to rotate the RGB payload periodically
const RGB_ROTATION_MINUTES: u64 = 5; // Rotate the payload after this many minutes
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
// END

fn main() -> Result<(), EcError> {
    let ec = CrosEc::new();

    // Set fan speed to configured percentage.
    ec.send_command(0x0024u16, 0, &FAN_SPEED_PERCENT.to_le_bytes())?;

    if ENABLE_LED_WRITE {
        let mut payload = RGBPAYLOAD;
        ec.rgbkbd_set_color(0, payload.to_vec())?;

        if ENABLE_RGB_ROTATION {
            // A zero-minute interval would create a busy loop, so reject it at
            // compile time rather than silently using an invalid configuration.
            assert!(RGB_ROTATION_MINUTES > 0, "RGB_ROTATION_MINUTES must be greater than zero");
            let rotation_interval = Duration::from_secs(RGB_ROTATION_MINUTES * 60);

            loop {
                thread::sleep(rotation_interval);
                payload.rotate_left(1);
                ec.rgbkbd_set_color(0, payload.to_vec())?;
            }
        }
    }

    Ok(())
}
