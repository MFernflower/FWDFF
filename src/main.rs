use framework_lib::chromium_ec::{CrosEc, CrosEcDriver};
use framework_lib::chromium_ec::commands::RgbS;
const RGBPAYLOAD: [RgbS; 8] = [
    RgbS { r: 0xFF, g: 0xD7, b: 0x00 },
    RgbS { r: 0xFF, g: 0x45, b: 0x00 },
    RgbS { r: 0xFF, g: 0x7F, b: 0x50 },
    RgbS { r: 0xFF, g: 0xA5, b: 0x00 },
    RgbS { r: 0xFF, g: 0xFF, b: 0x00 },
    RgbS { r: 0xFF, g: 0x8C, b: 0x00 },
    RgbS { r: 0xFF, g: 0x63, b: 0x47 },
    RgbS { r: 0xFF, g: 0xD7, b: 0x00 },
];
fn main() {
    let ec = CrosEc::new();
    // Set fan rgb to the preset payload.
    let _ = ec.rgbkbd_set_color(0, RGBPAYLOAD.to_vec());
    // Set fan duty cycle to 50% (LE u32).
    let _ = ec.send_command(0x0024u16, 0, &50u32.to_le_bytes());
}
