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
<<<<<<< HEAD
    let data: u8 = ((mn<<6)&0b11000000) + (dir&1);
    let sum: u8 = (Wrapping(send_num) + Wrapping(address) + Wrapping(data) + Wrapping(speed)).0;
    vec![header,send_num,address,data,speed,sum,footer]
=======
    let length: u8 = 4;
    mn = mn & 0x03;
    func = func & 0x07;
    dir = dir & 0x01;
    let data3 = (mn << 4) | (func << 1) | 1;

    let sum: u16 = (send_num + address + data3 + speed).into();
    let check_sum: u8 = sum as u8;

    vec![
        header, length, send_num, address, data3, speed, check_sum, footer,
    ]
>>>>>>> c4c5c3f99fa9c1180b73aef6ec904237637d31d0
}
