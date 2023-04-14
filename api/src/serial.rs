use once_cell::sync::Lazy;
use serialport::SerialPort;
use std::io::{Error, Write};
use std::sync::Mutex;
use std::time::Duration;

const DEFAULT_PATH: &str = "COM10";
const BAUD_RATE: u32 = 9600;
const TIMEOUT_DURATION: u64 = 100;

// Global mutable SerialPort instance
static SERIAL: Lazy<Mutex<Box<dyn SerialPort>>> =
    Lazy::new(|| Mutex::new(create_serial_port(DEFAULT_PATH).unwrap()));
// Path of SerialPort instance
static PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(DEFAULT_PATH.to_string()));

// reteurn SerialPort instance from path
fn create_serial_port(path: &str) -> Result<Box<dyn SerialPort>, serialport::Error> {
    let mut p = PATH.lock().unwrap();
    match serialport::new(path, BAUD_RATE)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(TIMEOUT_DURATION))
        .open()
    {
        Ok(s) => {
            *p = path.to_string();
            return Ok(s);
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(e);
        }
    }
}

// Change SerialPort connection
pub fn set_serial_port(path: &str) -> Result<(), serialport::Error> {
    let mut serial = SERIAL.lock().unwrap();
    let p;
    {
        p = PATH.lock().unwrap().clone(); //want to get p but not to want lock for a long time
    }
    if path.to_string() == p {
        return Ok(());
    } else {
        let port = create_serial_port(path);
        match port {
            Ok(s) => {
                *serial = s;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

// Write serial and return result
fn serial_write(buf: &[u8]) -> Result<(), Error> {
    let mut port = SERIAL.lock().unwrap();
    match port.write(buf) {
        Ok(_) => {
            std::io::stdout()
                .flush()
                .expect("in serial_write: Failed to flush stdout");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

// Read serial and return result
fn serial_read(buf: &mut Vec<u8>) -> Result<&[u8], Error> {
    let mut port = SERIAL.lock().unwrap();
    match port.read(buf.as_mut_slice()) {
        Ok(t) => {
            let bytes = &buf[..t];
            Ok(bytes)
        }
        Err(e) => Err(e),
    }
}

pub fn send(buf: &[u8]) -> Result<(), ()> {
    // 送信する
    match serial_write(buf) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(());
        }
    };
    //受信する
    let mut buf: Vec<u8> = vec![0; 10];
    match serial_read(&mut buf) {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(());
        }
    }
}
