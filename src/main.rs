use framework_lib::chromium_ec::{CrosEc, CrosEcDriver};
use framework_lib::chromium_ec::commands::RgbS;
use framework_lib::chromium_ec::EcError;
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};

// Compile-time configuration variables
const FAN_SPEED_PERCENT: u32 = 55; // Set fan speed percentage (0-100)
const ENABLE_LED_WRITE: bool = true; // Set to true to enable LED write, false to disable
const ENABLE_RGB_SCRAMBLE: bool = true; // Set to true to scramble the RGB payload periodically
const RGB_SCRAMBLE_MINUTES: u64 = 2; // Scramble the payload after this many minutes
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

fn scramble_rgb_payload(payload: &mut [RgbS; 8]) {
    let mut rng = rand::thread_rng();

    for i in (1..payload.len()).rev() {
        let j = rng.gen_range(0..=i);
        payload.swap(i, j);
    }
}

fn main() -> Result<(), EcError> {
    let ec = CrosEc::new();

    ec.send_command(0x0024u16, 0, &FAN_SPEED_PERCENT.to_le_bytes())?;

    if ENABLE_LED_WRITE {
        let mut payload = RGBPAYLOAD;
        ec.rgbkbd_set_color(0, payload.to_vec())?;

        if ENABLE_RGB_SCRAMBLE {
            assert!(RGB_SCRAMBLE_MINUTES > 0, "RGB_SCRAMBLE_MINUTES must be greater than zero");

            let scramble_interval = Duration::from_secs(RGB_SCRAMBLE_MINUTES * 60);
            let mut next_scramble = Instant::now() + scramble_interval;

            loop {
                let now = Instant::now();

                if now >= next_scramble {
                    scramble_rgb_payload(&mut payload);
                    ec.rgbkbd_set_color(0, payload.to_vec())?;
                    next_scramble += scramble_interval;
                }

                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    Ok(())
}
