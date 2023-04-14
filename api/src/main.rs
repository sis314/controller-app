use std::time::Duration;

mod serial;
fn main() {
    serial::set_serial_port("COM1").unwrap();
    serial::set_serial_port("COM10").unwrap();
    loop {
        let message: Vec<u8> = vec![0x02];
        match serial::send(&message) {
            Ok(_) => println!("ok"),
            Err(_) => println!("err"),
        };
        std::thread::sleep(Duration::from_millis(1000));
    }
}

fn format(id: u8, mn: u8, dir: u8, val: u8) -> Vec<u8> {
    let header: u8 = 0x40;
    let footer: u8 = 0x0A;
    let len: u8 = 0x03;
    let param: u8 = ((id << 4) & 240) | ((mn << 1) & 14) | (dir & 1);
    let val: u8 = val;
    let checksum = param + val;
    vec![header, len, param, val, checksum, footer]
}
