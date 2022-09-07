pub mod parser {
    use std::{cell::Cell, io::Read, net::TcpStream};

    pub struct IndexedBuffer<'a>(pub &'a Vec<u8>, pub Cell<usize>);

    pub struct PacketBuffer {
        pub data: Vec<u8>,
        pub index: Cell<usize>,
    }

    pub trait ReadUncompressed {
        fn read_packet(&mut self) -> Result<PacketBuffer, String>;
    }

    impl ReadUncompressed for TcpStream {
        fn read_packet(&mut self) -> Result<PacketBuffer, String> {
            let length = match parse_packet_length(self) {
                Ok(length) => length as usize,
                Err(e) => return Err(e),
            };

            let mut buffer = vec![0u8; length];
            if let Err(_) = &self.read_exact(&mut buffer) {
                return Err("Error filling buffer".to_string());
            }

            Ok(PacketBuffer {
                data: buffer,
                index: Cell::new(0),
            })
        }
    }

    impl PacketBuffer {
        pub fn parse_var_int(&self) -> i32 {
            let mut ans = 0;
            let mut used_bytes = 0;
            let index = &self.index.get();
            for i in 0..5 {
                used_bytes += 1;
                let val = &self.data[index + i];
                ans |= ((val & 0b0111_1111) as i32) << 7 * i;
                if val & 0b1000_0000 == 0 {
                    break;
                }
            }
            &self.index.set(index + used_bytes);
            ans
        }
        pub fn parse_var_long(&self) -> i64 {
            //not tested
            let mut ans = 0;
            let mut used_bytes = 0;
            let index = &self.index.get();
            for i in 0..10 {
                used_bytes += 1;
                let val = &self.data[index + i];
                ans |= ((val & 0b0111_1111) as i64) << 7 * i;
                if val & 0b1000_0000 == 0 {
                    break;
                }
            }
            &self.index.set(index + used_bytes);
            ans
        }
        pub fn parse_string(&self) -> String {
            let str_length = self.parse_var_int();

            let index = self.index.get();
            let slice = &self.data[index..(index + str_length as usize)];
            &self.index.set(index + str_length as usize);
            String::from_utf8_lossy(slice).to_string() //🐄
        }
        pub fn parse_unsigned_short(&self) -> u16 {
            let mut ans = 0;
            let index = self.index.get();
            for i in 0..2 {
                let val = &self.data[index + 1 - i];
                ans |= (val.clone() as u16) << (8 * i);
            }
            self.index.set(index + 2usize);
            ans
        }
        pub fn parse_signed_long(&self) -> i64 {
            let mut ans = 0;
            let index = self.index.get();
            for i in 0..8 {
                let val = &self.data[index + 7 - i];
                ans |= (val.clone() as i64) << (8 * i);
            }
            self.index.set(index + 8usize);
            ans
        }
        pub fn parse_byte_array(&self) -> Vec<u8> {
            let length = self.parse_var_int() as usize;
            let index = self.index.get();
            let read_bytes = &self.data[index..(index + length)];
            self.index.set(index + length);
            read_bytes.to_vec()
        }
    }

    pub fn parse_packet_length(stream: &mut TcpStream) -> Result<i32, String> {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..4 {
            if let Err(_) = stream.read_exact(&mut buf) {
                return Err("Stream closed by client".to_string());
            }
            ans |= ((buf[0] & 0b0111_1111) as i32) << 7 * i;
            if buf[0] & 0b1000_0000 == 0 {
                break;
            }
        }
        Ok(ans)
    }

    pub fn parse_var_int(indexed_buffer: &IndexedBuffer) -> i32 {
        let mut ans = 0;
        let mut used_bytes = 0;
        let index = indexed_buffer.1.get();
        for i in 0..5 {
            used_bytes += 1;
            let val = indexed_buffer.0.get(index + i).unwrap();
            ans |= ((val & 0b0111_1111) as i32) << 7 * i;
            if val & 0b1000_0000 == 0 {
                break;
            }
        }
        indexed_buffer.1.set(index + used_bytes);
        ans
    }

    pub fn parse_var_long(indexed_buffer: &IndexedBuffer) -> i64 {
        //not tested
        let mut ans = 0;
        let mut used_bytes = 0;
        let index = indexed_buffer.1.get();
        for i in 0..10 {
            used_bytes += 1;
            let val = indexed_buffer.0.get(index + i).unwrap();
            ans |= ((val & 0b0111_1111) as i64) << 7 * i;
            if val & 0b1000_0000 == 0 {
                break;
            }
        }
        indexed_buffer.1.set(index + used_bytes);
        ans
    }

    pub fn parse_string(indexed_buffer: &IndexedBuffer) -> String {
        let str_length = parse_var_int(indexed_buffer);

        let index = indexed_buffer.1.get();
        let slice = &indexed_buffer.0[index..(&index + str_length as usize)];
        indexed_buffer.1.set(index + str_length as usize);
        String::from_utf8_lossy(slice).to_string() //🐄
    }

    pub fn parse_boolean(indexed_buffer: &IndexedBuffer) -> bool {
        let index = indexed_buffer.1.get();
        let val = indexed_buffer.0.get(index).unwrap();
        indexed_buffer.1.set(index + 1usize);
        val == &1u8
    }

    pub fn parse_signed_byte(indexed_buffer: &IndexedBuffer) -> i8 {
        let index = indexed_buffer.1.get();
        let val = indexed_buffer.0.get(index).unwrap();
        indexed_buffer.1.set(index + 1usize);
        val.clone() as i8
    }

    pub fn parse_unsigned_byte(indexed_buffer: &IndexedBuffer) -> u8 {
        let index = indexed_buffer.1.get();
        let val = indexed_buffer.0.get(index).unwrap();
        indexed_buffer.1.set(index + 1usize);
        val.clone()
    }

    pub fn parse_unsigned_short(indexed_buffer: &IndexedBuffer) -> u16 {
        let mut ans = 0;
        let index = indexed_buffer.1.get();
        for i in 0..2 {
            let val = indexed_buffer.0.get(index + 1 - i).unwrap();
            ans |= (val.clone() as u16) << (8 * i);
        }
        indexed_buffer.1.set(index + 2usize);
        ans
    }

    pub fn parse_signed_short(indexed_buffer: &IndexedBuffer) -> i16 {
        let mut ans = 0;
        let index = indexed_buffer.1.get();
        for i in 0..2 {
            let val = indexed_buffer.0.get(index + 1 - i).unwrap();
            ans |= (val.clone() as i16) << (8 * i);
        }
        indexed_buffer.1.set(index + 2usize);
        ans
    }

    pub fn parse_signed_int(indexed_buffer: &IndexedBuffer) -> i32 {
        let mut ans = 0;
        let index = indexed_buffer.1.get();
        for i in 0..4 {
            let val = indexed_buffer.0.get(index + 3 - i).unwrap();
            ans |= (val.clone() as i32) << (8 * i);
        }
        indexed_buffer.1.set(index + 4usize);
        ans
    }

    pub fn parse_signed_long(indexed_buffer: &IndexedBuffer) -> i64 {
        let mut ans = 0;
        let index = indexed_buffer.1.get();
        for i in 0..8 {
            let val = indexed_buffer.0.get(index + 7 - i).unwrap();
            ans |= (val.clone() as i64) << (8 * i);
        }
        indexed_buffer.1.set(index + 8usize);
        ans
    }

    pub fn parse_uuid(indexed_buffer: &IndexedBuffer) -> u128 {
        //untested
        let mut ans = 0;
        let index = indexed_buffer.1.get();
        for i in 0..16 {
            let val = indexed_buffer.0.get(index + 15 - i).unwrap();
            ans |= (val.clone() as u128) << (8 * i);
        }
        indexed_buffer.1.set(index + 16usize);
        ans
    }
}
