pub mod writer {
    pub fn write_var_int(int: i32) -> Vec<u8> {
        let mut output = Vec::new();
        let mut value = ((int << 1) ^ (int >> 31)) as u32;

        while value >= 0b10000000 {
            output.push(((value & 0b01111111) as u8) | 0b10000000);
            value = value >> 7;
        }

        output.push((value & 0b01111111) as u8);
        output
    }
}
