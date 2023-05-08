use crate::data::SendData;
use crate::error::{Error, ErrorKind};
use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;

const BAUD_RATE: u32 = 9600;
const TIMEOUT_DURATION: u64 = 100;

pub struct Serial {
    serial: Option<Box<dyn SerialPort>>,
    path: String,
}

impl Serial {
    pub fn new() -> Serial {
        Serial {
            serial: None,
            path: "".to_string(),
        }
    }

    /// reteurn SerialPort instance from path
    fn create_port(&mut self, path: &str) -> Result<Box<dyn SerialPort>, Error> {
        match serialport::new(path, BAUD_RATE)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .timeout(Duration::from_millis(TIMEOUT_DURATION))
            .open()
        {
            Ok(s) => {
                self.path = path.to_string();
                Ok(s)
            }
            Err(_) => Err(Error::new(ErrorKind::PortSetFailed)),
        }
    }

    /// Change SerialPort connection
    pub fn set_port(&mut self, path: &str) -> Result<(), Error> {
        if path == &self.path[..] {
            println!("port is already {}", path);
            Ok(())
        } else {
            let port = self.create_port(path);
            match port {
                Ok(s) => {
                    self.serial = Some(s);
                    println!("port set to {}", path);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    }

    fn check_received(data: &[u8], id: u8) -> bool {
        data[0] == id
    }

    /// Write serial and return result
    fn serial_write(&mut self, data: &SendData) -> Result<(), Error> {
        let port: &mut Box<dyn SerialPort> = match self.serial.as_mut() {
            Some(a) => a,
            None => {
                return Err(Error::new(ErrorKind::PortNotFound));
            }
        };
        match port.write(data.0.as_slice()) {
            Ok(_) => {
                std::io::stdout()
                    .flush()
                    .expect("in serial_write: Failed to flush stdout");
                Ok(())
            }
            Err(_) => Err(Error::new(ErrorKind::SerialWriteFailed)),
        }
    }

    /// Read serial and return result
    fn serial_read<'a>(&mut self, buf: &'a mut Vec<u8>) -> Result<&'a [u8], Error> {
        let port: &mut Box<dyn SerialPort> = match self.serial.as_mut() {
            Some(a) => a,
            None => {
                return Err(Error::new(ErrorKind::PortNotFound));
            }
        };
        match port.read(buf.as_mut_slice()) {
            Ok(t) => {
                let bytes = &buf[..t];
                Ok(bytes)
            }
            Err(_) => Err(Error::new(ErrorKind::SerialReadFailed)),
        }
    }

    pub fn send(&mut self, data: Vec<u8>) -> Result<(), Error> {
        let mut errbuf: Error = Error::new(ErrorKind::None);
        for _i in 0..=2 {
            let id: u8 = 0;

            // 送信する
            match self.serial_write(data) {
                Ok(_) => (),
                Err(e) => {
                    errbuf = e;
                    continue;
                }
            };
            //受信する
            let mut buf: Vec<u8> = vec![0; 10];
            match self.serial_read(&mut buf) {
                Ok(data) => {
                    if Self::check_received(data, id) {
                        return Ok(());
                    } else {
                        errbuf = Error::new(ErrorKind::InvalidDeviceReturn(data[0]));
                        continue;
                    }
                }
                Err(e) => {
                    errbuf = e;
                    continue;
                }
            }
        }
        Err(errbuf)
    }
}
