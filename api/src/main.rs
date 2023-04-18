use serial::Serial;
use std::time::Duration;

use crate::data::SendData;

mod data;
mod error;
mod serial;

fn main() {
    let mut serial = Serial::new();
    let a = serial.set_port("COM10");
    println!("{:?}", a);
    loop {
        let data = SendData::new(2, 3, 4, 5);
        match serial.send(&data) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        };
        std::thread::sleep(Duration::from_millis(1000));
    }
}
