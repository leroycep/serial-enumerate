# Serial Enumerate

`serial-enumerate` provides an easy way to enumerate serial ports.

## Usage

Add the following to your `Cargo.toml`.

```
serial-enumerate = {git = "https://github.com/Geemili/serial-enumerate.git"}`
```

## Example

```
for device in serial_enumerate::enumerate_serial_ports().unwrap() {
   println!("{}", device);
}
```

## Features

[x] Supports windows
[ ] Supports linux
