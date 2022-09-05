pub mod serializer {

    // FIX THESE XAVIER

    /*
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
    */

    pub fn serialize_boolean(mut current: Vec<u8>, input: bool) -> Vec<u8> {
        current.push(input as u8);
        current
    }

    pub fn serialize_var_int(mut current: Vec<u8>, mut input: i32) -> Vec<u8> {
        while input as u32 >= 0b10000000 {
            current.push(((input & 0b01111111) as u8) | 0b10000000);
            input = ((input as u32) >> 7) as i32;
        }

        current.push((input & 0b01111111) as u8);
        current
    }

    pub fn serialize_var_long(mut current: Vec<u8>, mut input: i64) -> Vec<u8> {
        while input as u64 >= 0b10000000 {
            current.push(((input & 0b01111111) as u8) | 0b10000000);
            input = ((input as u64) >> 7) as i64;
        }

        current.push((input & 0b01111111) as u8);
        current
    }

    pub fn serialize_string(mut current: Vec<u8>, input: &String) -> Vec<u8> {
        let mut string_data = input.clone().into_bytes();
        let mut output = serialize_var_int(current, string_data.len() as i32);
        output.append(&mut string_data);
        output
    }
}
