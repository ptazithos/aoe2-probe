use crate::data::PrefixString;

pub struct IncrementalGenerator {
    pub buffer: Vec<u8>,
    pub pos: usize,
}

impl IncrementalGenerator {
    pub fn init(buffer: Vec<u8>) -> Self {
        IncrementalGenerator {
            buffer: buffer,
            pos: 0,
        }
    }

    pub fn get_str_with_len(&mut self, len: usize) -> String {
        let current_pos = self.pos as usize;

        let buffer = &self.buffer[current_pos..(current_pos + len as usize)];
        self.pos += len;

        String::from_utf8_lossy(buffer).to_string()
    }

    pub fn get_str_with_len_repeat_16(&mut self, len: usize) -> [String; 16] {
        const REPEAT: usize = 16;
        let mut res: [String; REPEAT] = Default::default();
        for i in 0..REPEAT {
            res[i] = self.get_str_with_len(len)
        }

        return res;
    }

    pub fn get_unsigned_int_8(&mut self) -> u8 {
        let current_pos = self.pos;
        const LEN: usize = 1;

        let mut buffer: [u8; LEN] = [0; LEN];
        buffer.copy_from_slice(&self.buffer[current_pos..current_pos + LEN]);

        self.pos += LEN;

        u8::from_le_bytes(buffer)
    }

    pub fn get_unsigned_int_8_repeat_8(&mut self) -> [u8; 8] {
        [
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
        ]
    }

    pub fn get_unsigned_int_8_repeat_16(&mut self) -> [u8; 16] {
        [
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
            self.get_unsigned_int_8(),
        ]
    }

    pub fn get_unsigned_int_16(&mut self) -> u16 {
        let current_pos = self.pos;
        const LEN: usize = 2;

        let mut buffer: [u8; LEN] = [0; LEN];
        buffer.copy_from_slice(&self.buffer[current_pos..current_pos + LEN]);

        self.pos += LEN;

        u16::from_le_bytes(buffer)
    }

    pub fn get_unsigned_int_32(&mut self) -> u32 {
        let current_pos = self.pos;
        const LEN: usize = 4;

        let mut buffer: [u8; LEN] = [0; LEN];
        buffer.copy_from_slice(&self.buffer[current_pos..current_pos + LEN]);

        self.pos += LEN;

        u32::from_le_bytes(buffer)
    }

    pub fn get_unsigned_int_32_repeat_16(&mut self) -> [u32; 16] {
        const REPEAT: usize = 16;
        let mut res: [u32; REPEAT] = Default::default();
        for i in 0..REPEAT {
            res[i] = self.get_unsigned_int_32();
        }

        return res;
    }

    pub fn get_signed_int_8(&mut self) -> i8 {
        let current_pos = self.pos;
        const LEN: usize = 1;

        let mut buffer: [u8; LEN] = [0; LEN];
        buffer.copy_from_slice(&self.buffer[current_pos..current_pos + LEN]);

        self.pos += LEN;

        i8::from_le_bytes(buffer)
    }

    pub fn get_signed_int_16(&mut self) -> i16 {
        let current_pos = self.pos;
        const LEN: usize = 2;

        let mut buffer: [u8; LEN] = [0; LEN];
        buffer.copy_from_slice(&self.buffer[current_pos..current_pos + LEN]);

        self.pos += LEN;

        i16::from_le_bytes(buffer)
    }

    pub fn get_signed_int_32(&mut self) -> i32 {
        let current_pos = self.pos;
        const LEN: usize = 4;

        let mut buffer: [u8; LEN] = [0; LEN];
        buffer.copy_from_slice(&self.buffer[current_pos..current_pos + LEN]);

        self.pos += LEN;

        i32::from_le_bytes(buffer)
    }

    pub fn get_str_16(&mut self) -> PrefixString<u16> {
        let len = self.get_unsigned_int_16();
        PrefixString::new(len, &self.get_str_with_len(len as usize))
    }

    pub fn get_str_16_repeat_16(&mut self) -> [PrefixString<u16>; 16] {
        [
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
        ]
    }

    pub fn get_str_16_repeat_32(&mut self) -> [PrefixString<u16>; 32] {
        [
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
            self.get_str_16(),
        ]
    }

    pub fn get_str_32(&mut self) -> PrefixString<u32> {
        let len = self.get_unsigned_int_32();
        PrefixString::new(len, &self.get_str_with_len(len as usize))
    }

    pub fn get_float_32(&mut self) -> f32 {
        let current_pos = self.pos;
        const LEN: usize = 4;

        let mut buffer: [u8; LEN] = [0; LEN];
        buffer.copy_from_slice(&self.buffer[current_pos..current_pos + LEN]);

        self.pos += LEN;

        f32::from_le_bytes(buffer)
    }

    pub fn get_float_64(&mut self) -> f64 {
        let current_pos = self.pos;
        const LEN: usize = 8;

        let mut buffer: [u8; LEN] = [0; LEN];
        buffer.copy_from_slice(&self.buffer[current_pos..current_pos + LEN]);

        self.pos += LEN;

        f64::from_le_bytes(buffer)
    }

    pub fn get_res_bytes(&mut self) -> Vec<u8> {
        self.buffer[self.pos..].to_vec()
    }

    pub fn get_vec_u32_with_len(&mut self, len: usize) -> Vec<u32> {
        let mut res = Vec::<u32>::new();
        for _ in 0..len {
            res.push(self.get_unsigned_int_32());
        }

        res
    }

    pub fn skip_bytes(&mut self, num: usize) -> Vec<u8> {
        let current_pos = self.pos as usize;

        let buffer = &self.buffer[current_pos..(current_pos + num)];
        self.pos += num;

        buffer.to_vec()
    }
}
