pub enum ErrorKind {
    PortNotFound,
    PortSetFailed,
    SerialWriteFailed,
    SerialReadFailed,
    //InvalidDeviceReturn(u8),
    None,
}

pub struct Error {
    error: ErrorKind,
}

impl Error {
    pub fn new(error_kind: ErrorKind) -> Error {
        Error { error: error_kind }
    }

    pub fn to_str(&self) -> String {
        let str = match self.error {
            ErrorKind::PortNotFound => "PortNotFound",
            ErrorKind::PortSetFailed => "PortSetFailed",
            ErrorKind::SerialWriteFailed => "SerialWriteFailed",
            ErrorKind::SerialReadFailed => "SerialReadFailed",
            //ErrorKind::InvalidDeviceReturn(_) => "InvalidDeviceReturn",
            ErrorKind::None => "NotDefined",
        };
        str.to_string()
    }
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error {
            ErrorKind::PortNotFound => f.write_str(
                "\x1b[1m\x1b[31merror\x1b[m\x1b[1m:\x1b[m PortNotFound. Init port before use",
            ),
            ErrorKind::PortSetFailed => {
                f.write_str("\x1b[1m\x1b[31merror\x1b[m\x1b[1m:\x1b[m PortSetFailed. Invalid port pass or Permission denided")
            }
            ErrorKind::SerialWriteFailed => {
                f.write_str("\x1b[1m\x1b[31merror\x1b[m\x1b[1m:\x1b[m SerialWriteFailed. Can't write to serial")
            }
            ErrorKind::SerialReadFailed => {
                f.write_str("\x1b[1m\x1b[31merror\x1b[m\x1b[1m:\x1b[m SerialReadFailed. Can't read serial")
            }
            //ErrorKind::InvalidDeviceReturn(data) => {
            //    f.write_str(&format!("\x1b[1m\x1b[31merror\x1b[m\x1b[1m:\x1b[m InvalidDeviceReturn. Returned {:?}", data))
            //}
            ErrorKind::None => f.write_str("\x1b[1m\x1b[31merror\x1b[m\x1b[1m:\x1b[m None. no Error"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
