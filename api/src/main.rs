use crate::data::SendData;
use once_cell::sync::Lazy; // 1.3.1
use serial::Serial;
use std::sync::Mutex;
use std::time::Duration;

mod data;
mod error;
mod serial;

static SERIAL: Lazy<Mutex<Serial>> = Lazy::new(|| Mutex::new(Serial::new()));

fn main() {
    match set_port("COM10") {
        Err(_) => (),
        Ok(_) => (),
    }
    loop {
        match send(2, 3, 4, 5) {
            Err(_) => (),
            Ok(_) => (),
        };
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn set_port(path: &str) -> Result<(), ()> {
    let mut serial = SERIAL.lock().unwrap();
    match serial.set_port(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(())
        }
    }
}

fn send(id: u8, mn: u8, dir: u8, val: u8) -> Result<(), ()> {
    let data = SendData::new(id, mn, dir, val);
    let mut serial = SERIAL.lock().unwrap();
    match serial.send(&data) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(())
        }
    }
}
