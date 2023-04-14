use once_cell::sync::Lazy;
use serialport::SerialPort;
use std::io::Error;
use std::io::Write;
use std::sync::Mutex;
use std::time::Duration;

// Global mutable SerialPort instance
static SERIAL: Lazy<Mutex<Box<dyn SerialPort>>> =
    Lazy::new(|| Mutex::new(create_serial_port("COM10")));

// reteurn SerialPort instance from path
fn create_serial_port(path: &str) -> Box<dyn SerialPort> {
    serialport::new(path, 9600)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(100))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", path, e);
            ::std::process::exit(1);
        })
}

// Write serial and return result
fn serial_write(buf: &[u8]) -> Result<(), Error> {
    let mut port = SERIAL.lock().unwrap();
    match port.write(buf) {
        Ok(_) => {
            std::io::stdout()
                .flush()
                .expect("in serial_write: Failed to flush stdout");
            return Ok(());
        }
        Err(e) => return Err(e),
    }
}

// Read serial and return result
fn serial_read(buf: &mut Vec<u8>) -> Result<&[u8], Error> {
    let mut port = SERIAL.lock().unwrap();
    match port.read(buf.as_mut_slice()) {
        Ok(t) => {
            let bytes = &buf[..t];
            return Ok(bytes);
        }
        Err(e) => return Err(e),
    };
}

fn main() {
    let mut buf: Vec<u8> = vec![0; 1000];
    loop {
        println!("Write...");
        let message: u8 = 0x02;
        match serial_write(&[message]) {
            Ok(_) => (),
            Err(e) => eprintln!("{:?}", e),
        }

        println!("Read...");
        let read = serial_read(&mut buf);
        match read {
            Ok(data) => println!("{:?}", data),
            Err(e) => eprintln!("{:?}", e),
        }
        std::thread::sleep(Duration::from_millis(1000));
    }
}
