pub mod serializer {
    // use std::{cell::Cell, io::Read, net::TcpStream};

    pub fn serialize_unsigned_byte(input: u8) -> Vec<u8> {
        input.to_be_bytes().to_vec()
    }

    pub fn serialize_signed_byte(input: i8) -> Vec<u8> {
        input.to_be_bytes().to_vec()
    }

    pub fn serialize_signed_short(input: i16) -> Vec<u8> {
        input.to_be_bytes().to_vec()
    }

    pub fn serialize_unsigned_short(input: u16) -> Vec<u8> {
        input.to_be_bytes().to_vec()
    }

    pub fn serialize_signed_int(input: i32) -> Vec<u8> {
        input.to_be_bytes().to_vec()
    }

    pub fn serialize_signed_long(input: i64) -> Vec<u8> {
        input.to_be_bytes().to_vec()
    }

    pub fn serialize_uuid(input: i128) -> Vec<u8> {
        input.to_be_bytes().to_vec()
    }

    pub fn serialize_boolean(input: bool) -> Vec<u8> {
        (input as u8).to_be_bytes().to_vec() //0000_0000: false, 0000_0001: true
    }

    pub fn serialize_var_int(mut input: i32) -> Vec<u8> {
        let mut output = Vec::new();

        while input as u32 >= 0b10000000 {
            output.push(((input & 0b01111111) as u8) | 0b10000000);
            input = ((input as u32) >> 7) as i32;
        }

        output.push((input & 0b01111111) as u8);
        output
    }

    pub fn serialize_var_long(mut input: i64) -> Vec<u8> {
        let mut output = Vec::new();

        while input as u64 >= 0b10000000 {
            output.push(((input & 0b01111111) as u8) | 0b10000000);
            input = ((input as u64) >> 7) as i64;
        }

        output.push((input & 0b01111111) as u8);
        output
    }

    pub fn serialize_string(input: String) -> Vec<u8> {
        let mut string_data = input.into_bytes();
        let mut output = serialize_var_int(string_data.len() as i32);
        output.append(&mut string_data);
        output
    }
}
