use std::{io::Write, time::Duration};

fn main() {
    let mut port = serialport::new("COM1", 9600)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(100))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyUSB", e);
            ::std::process::exit(1);
        });

    let mut buf: Vec<u8> = vec![0; 1000];
    loop {
        println!("Write...");
        let message: u8 = 0x02;
        match port.write(&[message]) {
            Ok(_) => std::io::stdout().flush().unwrap(),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }

        println!("Read...");
        match port.read(buf.as_mut_slice()) {
            Ok(t) => {
                let bytes = &buf[..t];
                println!("bytes: {:?}", bytes);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        std::thread::sleep(Duration::from_millis(1000));
    }
}
