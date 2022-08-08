use crate::io::Source;

use super::Token;

const BYTE_LEN_FOR_8: usize = 1;
const BYTE_LEN_FOR_16: usize = 2;
const BYTE_LEN_FOR_32: usize = 4;
const BYTE_LEN_FOR_64: usize = 8;

pub trait Encode {
    fn to_le_vec(&self) -> Vec<u8>;
}

impl Encode for u16 {
    fn to_le_vec(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Encode for u32 {
    fn to_le_vec(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Encode for Vec<Token> {
    fn to_le_vec(&self) -> Vec<u8> {
        self.iter().flat_map(|token| token.to_le_vec()).collect()
    }
}

pub trait Decode {
    fn from_le_vec(source: &mut Source) -> Self;
}

impl Decode for u8 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_8] = [0; BYTE_LEN_FOR_8];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_8)[..]);
        u8::from_le_bytes(temp)
    }
}

impl Decode for u16 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_16] = [0; BYTE_LEN_FOR_16];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_16)[..]);
        u16::from_le_bytes(temp)
    }
}

impl Decode for u32 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_32] = [0; BYTE_LEN_FOR_32];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_32)[..]);
        u32::from_le_bytes(temp)
    }
}

impl Decode for i8 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_8] = [0; BYTE_LEN_FOR_8];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_8)[..]);
        i8::from_le_bytes(temp)
    }
}

impl Decode for i16 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_16] = [0; BYTE_LEN_FOR_16];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_16)[..]);
        i16::from_le_bytes(temp)
    }
}

impl Decode for i32 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_32] = [0; BYTE_LEN_FOR_32];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_32)[..]);
        i32::from_le_bytes(temp)
    }
}

impl Decode for f32 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_32] = [0; BYTE_LEN_FOR_32];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_32)[..]);
        f32::from_le_bytes(temp)
    }
}

impl Decode for f64 {
    fn from_le_vec(source: &mut Source) -> Self {
        let mut temp: [u8; BYTE_LEN_FOR_64] = [0; BYTE_LEN_FOR_64];
        temp.copy_from_slice(&source.get_vec(BYTE_LEN_FOR_64)[..]);
        f64::from_le_bytes(temp)
    }
}
