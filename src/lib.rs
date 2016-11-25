
extern crate winreg;

pub fn enumerate_serial_ports() -> Vec<String> {
    use winreg::RegKey;
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let serial_device_map = hklm.open_subkey_with_flags("HARDWARE\\DEVICEMAP\\SERIALCOMM", KEY_READ)
        .unwrap();

    let mut devices = vec![];
    for entry in serial_device_map.enum_values() {
        match entry {
            Ok((name, _reg_value)) => devices.push(serial_device_map.get_value(name).unwrap()),
            Err(_) => panic!("Error in device listing"),
        }
    }

    devices
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
