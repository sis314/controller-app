use std::time::Duration;

mod serial;
fn main() {
    serial::set_serial_port("COM1").unwrap();
    println!("change to COM1");
    serial::set_serial_port("COM10").unwrap();
    println!("change to COM10");
    loop {
        println!("Write...");
        let message: Vec<u8> = vec![0x02];
        match serial::send(&message) {
            Ok(_) => println!("ok"),
            Err(_) => println!("err"),
        };
        std::thread::sleep(Duration::from_millis(1000));
    }
}
