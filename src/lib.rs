
//! This crate provides an easy way to enumerate serial ports.
//!
//! ```
//! for device in serial_enumerate::enumerate_serial_ports().unwrap() {
//!     println!("{}", device);
//! }
//!

#[macro_use]
extern crate error_chain;
extern crate winreg;

pub mod errors;

use errors::*;

/// Lists the serial ports that are connected to the computer
#[cfg(windows)]
pub fn enumerate_serial_ports() -> Result<Vec<String>> {
    use winreg::RegKey;
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let serial_device_map = hklm.open_subkey_with_flags("HARDWARE\\DEVICEMAP\\SERIALCOMM", KEY_READ)
        .chain_err(|| "Unable to open registry")?;

    let mut devices = vec![];
    for entry in serial_device_map.enum_values() {
        let name = entry.chain_err(|| "The registry entry could not be retrieved")?.0;
        let device = serial_device_map.get_value(name)
            .chain_err(|| "The registry entry's value could not be retrieved")?;
        devices.push(device);
    }

    Ok(devices)
}
