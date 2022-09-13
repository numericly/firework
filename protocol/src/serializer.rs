use quartz_nbt::{io, NbtCompound};

pub struct OutboundPacketData {
    pub data: Vec<u8>,
}

impl OutboundPacketData {
    pub fn new() -> OutboundPacketData {
        OutboundPacketData { data: Vec::new() }
    }
    pub fn write_var_int(&mut self, val: i32) {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut val = val;
        loop {
            let mut current_byte = (val & SEGMENT_BITS as i32) as u8;
            val >>= 7;
            if val != 0 {
                current_byte |= CONTINUE_BIT;
            }
            self.data.push(current_byte);
            if val == 0 {
                break;
            }
        }
    }
    pub fn write_string(&mut self, val: &str) {
        let bytes = val.as_bytes();
        self.write_var_int(bytes.len() as i32);
        self.data.extend_from_slice(bytes);
    }
    pub fn write_signed_long(&mut self, val: i64) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }
    pub fn write_bytes(&mut self, val: &[u8]) {
        self.data.extend_from_slice(val);
    }
    pub fn write_uuid(&mut self, val: u128) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }
    pub fn write_bool(&mut self, val: bool) {
        self.data.push(if val { 1 } else { 0 });
    }
    pub fn write_nbt(&mut self, nbt: &NbtCompound) {
        io::write_nbt(&mut self.data, None, nbt, io::Flavor::Uncompressed).unwrap();
    }
    pub fn write_signed_byte(&mut self, val: i8) {
        self.data.push(val as u8);
    }
    pub fn write_unsigned_byte(&mut self, val: u8) {
        self.data.push(val);
    }
    pub fn write_signed_int(&mut self, val: i32) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }
    pub fn write_float(&mut self, val: f32) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }
    pub fn write_double(&mut self, val: f64) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }
    pub fn write_length(length: usize) -> Vec<u8> {
        let mut data = Vec::new();

        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut val = length;
        loop {
            let mut current_byte = (val & SEGMENT_BITS as usize) as u8;
            val >>= 7;
            if val != 0 {
                current_byte |= CONTINUE_BIT;
            }
            data.push(current_byte);
            if val == 0 {
                break;
            }
        }
        data
    }
}
