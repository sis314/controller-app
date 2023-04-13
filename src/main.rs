use std::time::Duration;

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyUSB", e);
            ::std::process::exit(1);
        });
}
