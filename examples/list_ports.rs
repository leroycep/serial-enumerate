
extern crate serial_enumerate;

fn main() {
    for device in serial_enumerate::enumerate_serial_ports().expect("Ports could not be listed") {
        println!("{}", device);
    }
}
