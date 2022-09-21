use std::io::Result;
use std::io::Write;

pub trait Serialize {
    fn write_bool(&mut self, bool: bool) -> Result<()>;
    fn write_u8(&mut self, u8: u8) -> Result<()>;
    fn write_i8(&mut self, i8: i8) -> Result<()>;
    fn write_u16(&mut self, u16: u16) -> Result<()>;
    fn write_i16(&mut self, i16: i16) -> Result<()>;
    fn write_i32(&mut self, i32: i32) -> Result<()>;
    fn write_i64(&mut self, i64: i64) -> Result<()>;
    fn write_f32(&mut self, f32: f32) -> Result<()>;
    fn write_f64(&mut self, f64: f64) -> Result<()>;
    fn write_sized_string(&mut self, sized_string: &str) -> Result<()>;
}

impl Serialize for dyn Write {
    fn write_bool(&mut self, bool: bool) -> Result<()> {
        let buf = [bool as u8; 1];
        self.write_all(&buf)
    }
    fn write_u8(&mut self, u8: u8) -> Result<()> {
        let buf = [u8; 1];
        self.write_all(&buf)
    }
    fn write_i8(&mut self, i8: i8) -> Result<()> {
        let buf = [i8 as u8; 1];
        self.write_all(&buf)
    }
    fn write_u16(&mut self, u16: u16) -> Result<()> {
        let buf = u16.to_be_bytes();
        self.write_all(&buf)
    }
    fn write_i16(&mut self, i16: i16) -> Result<()> {
        let buf = i16.to_be_bytes();
        self.write_all(&buf)
    }
    fn write_i32(&mut self, i32: i32) -> Result<()> {
        let buf = i32.to_be_bytes();
        self.write_all(&buf)
    }
    fn write_i64(&mut self, i64: i64) -> Result<()> {
        let buf = i64.to_be_bytes();
        self.write_all(&buf)
    }
    fn write_f32(&mut self, f32: f32) -> Result<()> {
        let buf = f32.to_be_bytes();
        self.write_all(&buf)
    }
    fn write_f64(&mut self, f64: f64) -> Result<()> {
        let buf = f64.to_be_bytes();
        self.write_all(&buf)
    }
    fn write_sized_string(&mut self, sized_string: &str) -> Result<()> {
        let len = sized_string.len() as u16;
        self.write_u16(len)?;
        self.write_all(sized_string.as_bytes())
    }
}
