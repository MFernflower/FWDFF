use framework_lib::chromium_ec::{CrosEc, CrosEcDriver};
use framework_lib::chromium_ec::commands::RgbS;
use framework_lib::chromium_ec::EcError;

const RGBPAYLOAD: [RgbS; 8] = [
    RgbS { r: 0x00, g: 0xFF, b: 0x80 },
    RgbS { r: 0x00, g: 0xE5, b: 0xCC },
    RgbS { r: 0x1F, g: 0xC5, b: 0xB5 },
    RgbS { r: 0x00, g: 0xD4, b: 0xAA },
    RgbS { r: 0x20, g: 0xB2, b: 0xAA },
    RgbS { r: 0x00, g: 0xCE, b: 0xD1 },
    RgbS { r: 0x00, g: 0xFF, b: 0x9F },
    RgbS { r: 0x00, g: 0xFF, b: 0x80 },
];

fn main() -> Result<(), EcError> {
    let ec = CrosEc::new();
    // Set fan rgb to the preset payload.
    ec.rgbkbd_set_color(0, RGBPAYLOAD.to_vec())?;
    // Set fan duty cycle to 50% (LE u32).
    ec.send_command(0x0024u16, 0, &50u32.to_le_bytes())?;
    Ok(())
}
