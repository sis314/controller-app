// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(_name: &str) -> String {
    let id = 15;
    let mn = 7;
    let pos = 1;
    let val = 1;
    let buf: Vec<u8> = format(
        id.try_into().unwrap(),
        mn.try_into().unwrap(),
        pos.try_into().unwrap(),
        val.try_into().unwrap(),
    );
    serial::send(&buf).unwrap();
    "hi".to_string()
}

#[tauri::command]
fn operate(id: i32, mn: i32, pos: i32, val: i32) {
    let buf: Vec<u8> = format(
        id.try_into().unwrap(),
        mn.try_into().unwrap(),
        pos.try_into().unwrap(),
        val.try_into().unwrap(),
    );
    serial::send(&buf).unwrap(); //must to hundle
}

mod serial;

fn main() {
    serial::set_serial_port("COM10").unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn format(id: u8, mn: u8, pos: u8, val: u8) -> Vec<u8> {
    let header: u8 = 0x40;
    let footer: u8 = 0x0A;
    let len: u8 = 0x03;
    let param: u8 = ((id << 4) & 240) | ((mn << 1) & 14) | (pos & 1);
    let val: u8 = val;
    let checksum = param.wrapping_add(val);
    vec![header, len, param, val, checksum, footer]
}
