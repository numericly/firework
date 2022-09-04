pub mod parser {
    use std::{cell::Cell, io::Read, net::TcpStream};

    pub struct IndexedBuffer<'a>(pub &'a Vec<u8>, pub Cell<usize>);

    pub fn parse_packet_length(stream: &mut TcpStream) -> Result<i32, std::io::Error> {
        let mut buf = [0];
        let mut ans = 0;
        for i in 0..4 {
            stream.read_exact(&mut buf)?;
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
        for i in 0..4 {
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
}
