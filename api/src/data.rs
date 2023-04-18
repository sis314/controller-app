#[derive(Debug)]
pub struct SendData(pub Vec<u8>);

impl SendData {
    pub fn new(id: u8, mn: u8, dir: u8, val: u8) -> SendData {
        let header: u8 = 0x40;
        let footer: u8 = 0x0A;
        let len: u8 = 0x03;
        let param: u8 = ((id << 4) & 240) | ((mn << 1) & 14) | (dir & 1);
        let val: u8 = val;
        let checksum = param + val;
        let data = vec![header, len, param, val, checksum, footer];
        SendData(data)
    }
}
