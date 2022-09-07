pub mod serializer {

    // FIX THESE XAVIER

    pub fn serialize_unsigned_byte(mut current: Vec<u8>, input: u8) -> Vec<u8> {
        vec![input]
    }

    pub fn serialize_signed_byte(mut current: Vec<u8>, input: i8) -> Vec<u8> {
        vec![input as u8]
    }

    pub fn serialize_signed_short(mut current: Vec<u8>, input: i16) -> Vec<u8> {
        [current, input.to_be_bytes().to_vec()].concat()
    }

    pub fn serialize_unsigned_short(mut current: Vec<u8>, input: u16) -> Vec<u8> {
        [current, input.to_be_bytes().to_vec()].concat()
    }

    pub fn serialize_signed_int(mut current: Vec<u8>, input: i32) -> Vec<u8> {
        [current, input.to_be_bytes().to_vec()].concat()
    }

    pub fn serialize_signed_long(mut current: Vec<u8>, input: i64) -> Vec<u8> {
        [current, input.to_be_bytes().to_vec()].concat()
    }

    pub fn serialize_uuid(mut current: Vec<u8>, input: i128) -> Vec<u8> {
        [current, input.to_be_bytes().to_vec()].concat()
    }

    pub fn serialize_boolean(mut current: Vec<u8>, input: bool) -> Vec<u8> {
        vec![input as u8]
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

    pub fn serialize_string(current: Vec<u8>, input: &String) -> Vec<u8> {
        let string_data = input.clone().as_bytes().to_vec();
        let output = serialize_var_int(current, string_data.len() as i32);
        [output, string_data].concat()
    }

    pub fn serialize_byte_array(mut current: Vec<u8>, array: &mut Vec<u8>) -> Vec<u8> {
        current.append(array);
        current
    }
}
