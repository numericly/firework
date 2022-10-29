use crate::tesr::server_bound::DeserializeError;

#[derive(Debug)]
pub struct IncomingPacketData {
    pub data: Vec<u8>,
    pub index: usize,
}

impl IncomingPacketData {
    pub fn new(data: Vec<u8>) -> IncomingPacketData {
        IncomingPacketData { data, index: 0 }
    }
    pub fn read_var_int(&mut self) -> Result<i32, DeserializeError> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut val = 0i32;

        for i in 0..4 {
            let position = i * 7;
            let current_byte = &self.data[self.index];
            self.index += 1;

            val |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            } else if i == 4 {
                return Err(DeserializeError::VarIntTooLong);
            }
        }
        Ok(val)
    }
    pub fn read_string(&mut self) -> Result<String, DeserializeError> {
        let length = self.read_var_int()? as usize;
        let string = String::from_utf8(self.data[self.index..self.index + length].to_vec())
            .map_err(|err| DeserializeError::StringParseError(err))?;
        self.index += length;
        Ok(string)
    }
    pub fn read_u16(&mut self) -> Result<u16, DeserializeError> {
        let mut buf = [0u8; 2];
        buf.copy_from_slice(&self.data[self.index..self.index + 2]);
        self.index += 2;
        Ok(u16::from_be_bytes(buf))
    }
    pub fn read_long(&mut self) -> Result<i64, DeserializeError> {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.data[self.index..self.index + 8]);
        self.index += 8;
        Ok(i64::from_be_bytes(buf))
    }
    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, DeserializeError> {
        let bytes = self.data[self.index..self.index + length].to_vec();
        self.index += length;
        Ok(bytes)
    }
    pub fn read_unsigned_byte(&mut self) -> Result<u8, DeserializeError> {
        let byte = self.data[self.index];
        self.index += 1;
        Ok(byte)
    }
    pub fn read_boolean(&mut self) -> Result<bool, DeserializeError> {
        let byte = self.read_unsigned_byte()?;
        Ok(byte != 0)
    }
    pub fn read_double(&mut self) -> Result<f64, DeserializeError> {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.data[self.index..self.index + 8]);
        self.index += 8;
        Ok(f64::from_be_bytes(buf))
    }
    pub fn read_float(&mut self) -> Result<f32, DeserializeError> {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&self.data[self.index..self.index + 4]);
        self.index += 4;
        Ok(f32::from_be_bytes(buf))
    }
    pub fn read_short(&mut self) -> Result<i16, DeserializeError> {
        let mut buf = [0u8; 2];
        buf.copy_from_slice(&self.data[self.index..self.index + 2]);
        self.index += 2;
        Ok(i16::from_be_bytes(buf))
    }
}
