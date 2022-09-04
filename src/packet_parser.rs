pub mod Parser {
    use std::{io::Read, net::TcpStream};

    pub fn parse_var_int(stream: &mut TcpStream) -> Result<i32, std::io::Error> {
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
}
