pub struct OutboundPacket {
    pub data: Vec<u8>,
}

impl OutboundPacket {
    pub fn new() -> OutboundPacket {
        OutboundPacket { data: Vec::new() }
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
}
