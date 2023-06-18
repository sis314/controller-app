use once_cell::sync::Lazy; // 1.3.1
use serial::Serial;
use std::num::Wrapping;
use std::sync::Mutex;
use std::time::Duration;

mod error;
mod serial;

static SERIAL: Lazy<Mutex<Serial>> = Lazy::new(|| Mutex::new(Serial::new()));

fn main() {
    match set_port("COM10") {
        Err(e) => println!("{}", e),
        Ok(_) => (),
    }
    //Test
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
    match serial.change_port(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(e.to_str())
        }
    }
}

fn send(address: u8, mn: u8, mode: u8, dir: u8, pwm: u8) -> Result<(), String> {
    static mut SEND_NUM: u8 = 0;

    let mut serial = SERIAL.lock().unwrap();
    let send_num: u8;

    unsafe {
        send_num = SEND_NUM;
        SEND_NUM += 1;
    }

    let data: Vec<u8> = format(send_num, address, mn, mode, dir, pwm);

    match serial.send(data, send_num) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(e.to_str())
        }
    }
}

fn format(send_num: u8, address: u8, mn: u8, mode: u8, dir: u8, pwm: u8) -> Vec<u8> {
    let header: u8 = 0x40;
    let footer: u8 = 0x00;

    let data: u8 = ((mn & 3) << 6) + ((mode & 7) << 3) + (dir & 1);

    let check_sum: u8 = (Wrapping(send_num) + Wrapping(address) + Wrapping(data) + Wrapping(pwm)).0;

    vec![header, send_num, address, data, pwm, check_sum, footer]
}
