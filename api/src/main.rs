use once_cell::sync::Lazy; // 1.3.1
use serial::Serial;
use std::sync::Mutex;
use std::time::Duration;
use std::num::Wrapping;

mod error;
mod serial;

static SERIAL: Lazy<Mutex<Serial>> = Lazy::new(|| Mutex::new(Serial::new()));

fn main() {
    match set_port("COM10") {
        Err(e) => println!("{}", e),
        Ok(_) => (),
    }
    loop {
        match send(1, 2, 3, 4, 5) {
            Err(e) => println!("{}", e),
            Ok(_) => (),
        };
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn set_port(path: &str) -> Result<(), String> {
    let mut serial = SERIAL.lock().unwrap();
    match serial.set_port(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(e.to_string())
        }
    }
}

fn send(address: u8, mn: u8,dir: u8, speed: u8, id: u8) -> Result<(), String> {
    let mut serial = SERIAL.lock().unwrap();
    let send_num: u8 = 0; //*todo */
    let data: Vec<u8> = format(send_num, address, mn, dir, speed);

    match serial.send(data, send_num) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(e.to_string())
        }
    }
}

fn format(send_num: u8, address: u8, mn: u8, dir: u8, speed: u8) -> Vec<u8> {
    let header: u8 = 0x40;
    let footer: u8 = 0x00;
    let data: u8 = ((mn<<6)&0b11000000) + (dir&1);
    let sum: u8 = (Wrapping(send_num) + Wrapping(address) + Wrapping(data) + Wrapping(speed)).0;
    vec![header,send_num,address,data,speed,sum,footer]
}
