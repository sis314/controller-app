// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let message: Vec<u8> = vec![0x02];
    match serial::send(&message) {
        Ok(s) => println!("{:?}", s),
        Err(_) => println!("err"),
    };
    "hi".to_string()
}

mod serial;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    serial::set_serial_port("COM10").unwrap();
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
