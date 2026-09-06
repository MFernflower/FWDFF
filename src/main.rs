use framework_lib::chromium_ec::{CrosEc, CrosEcDriver};
use framework_lib::chromium_ec::commands::RgbS;
use framework_lib::chromium_ec::EcError;

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

fn main() -> Result<(), EcError> {
    let ec = CrosEc::new();
    ec.rgbkbd_set_color(0, RGBPAYLOAD.to_vec())?; /// Write to RGB leds
    ec.send_command(0x0024u16, 0, &50u32.to_le_bytes())?; /// set fan speed to 50%
    Ok(())
}
