
use framework_lib::chromium_ec::{CrosEc, CrosEcDriver, EcError};
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

fn main() -> Result<(), EcError> {
    let ec = CrosEc::new();

    println!("applying LED preset");
    if let Err(e) = ec.rgbkbd_set_color(0, RGBPAYLOAD.to_vec()) {
        eprintln!("rgb error: {:?}", e);
        std::process::exit(1);
    }

    println!("setting fan to 50%");
    // PwmSetFanDuty (0x24): payload is the percent as a LE u32.
    if let Err(e) = ec.send_command(0x0024u16, 0, &50u32.to_le_bytes()) {
        eprintln!("fan error: {:?}", e);
        std::process::exit(1);
    }

    println!("done");
    Ok(())
}
