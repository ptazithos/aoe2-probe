use crate::io::Source;

const BYTE_LEN_FOR_8: usize = 1;
const BYTE_LEN_FOR_16: usize = 2;
const BYTE_LEN_FOR_32: usize = 4;
const BYTE_LEN_FOR_64: usize = 8;

pub trait Serialize {
    fn to_le_vec(&self) -> Vec<u8>;
}

impl Serialize for u16 {
    fn to_le_vec(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for u32 {
    fn to_le_vec(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

pub trait Deserialize {
    fn from_le_vec(source: &mut Source) -> Self;
}

impl Deserialize for u8 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_8] = [0; BYTE_LEN_FOR_8];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_8)[..]);
        u8::from_le_bytes(temp)
    }
}

impl Deserialize for u16 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_16] = [0; BYTE_LEN_FOR_16];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_16)[..]);
        u16::from_le_bytes(temp)
    }
}

impl Deserialize for u32 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_32] = [0; BYTE_LEN_FOR_32];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_32)[..]);
        u32::from_le_bytes(temp)
    }
}

impl Deserialize for i8 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_8] = [0; BYTE_LEN_FOR_8];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_8)[..]);
        i8::from_le_bytes(temp)
    }
}

impl Deserialize for i16 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_16] = [0; BYTE_LEN_FOR_16];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_16)[..]);
        i16::from_le_bytes(temp)
    }
}

impl Deserialize for i32 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_32] = [0; BYTE_LEN_FOR_32];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_32)[..]);
        i32::from_le_bytes(temp)
    }
}

impl Deserialize for f32 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_32] = [0; BYTE_LEN_FOR_32];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_32)[..]);
        f32::from_le_bytes(temp)
    }
}

impl Deserialize for f64 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_64] = [0; BYTE_LEN_FOR_64];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_64)[..]);
        f64::from_le_bytes(temp)
    }
}
