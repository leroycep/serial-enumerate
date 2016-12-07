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
