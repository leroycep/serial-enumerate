This project is project is no longer being maintained. The [serialport-rs](https://gitlab.com/susurrus/serialport-rs) library offers functionality to interact with serialports and can also list available ports. Use it instead.

# Serial Enumerate

`serial-enumerate` provides an easy way to enumerate serial ports on windows and unix operating systems.

## Usage

Add the following to your `Cargo.toml`.

```
serial_enumerate = "0.1.0"
```

## Example

```
for device in serial_enumerate::enumerate_serial_ports().unwrap() {
   println!("{}", device);
}
```
