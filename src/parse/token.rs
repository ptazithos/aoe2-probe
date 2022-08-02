use crate::utils::{DynString, LinkedHashMap, C256, C4};

use super::serde::Serialize;

#[derive(Clone, Debug)]
pub enum Token {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),

    Int8(i8),
    Int16(i16),
    Int32(i32),

    Float32(f32),
    Float64(f64),

    Char4(C4),
    Char256(C256),

    Str16(DynString<u16>),
    Str32(DynString<u32>),

    Union(LinkedHashMap),
    Vector(Vec<Token>),
}

impl Token {
    pub fn try_u8(&self) -> u8 {
        match self {
            Token::UInt8(value) => *value,
            _ => panic!("Not a u8 element!"),
        }
    }
    pub fn try_u16(&self) -> u16 {
        match self {
            Token::UInt16(value) => *value,
            _ => panic!("Not a u16 element!"),
        }
    }
    pub fn try_u32(&self) -> u32 {
        match self {
            Token::UInt32(value) => *value,
            _ => panic!("Not a u32 element!"),
        }
    }

    pub fn try_i8(&self) -> i8 {
        match self {
            Token::Int8(value) => *value,
            _ => panic!("Not a u8 element!"),
        }
    }
    pub fn try_i16(&self) -> i16 {
        match self {
            Token::Int16(value) => *value,
            _ => panic!("Not a u16 element!"),
        }
    }
    pub fn try_i32(&self) -> i32 {
        match self {
            Token::Int32(value) => *value,
            _ => panic!("Not a u32 element!"),
        }
    }

    pub fn try_f32(&self) -> f32 {
        match self {
            Token::Float32(value) => *value,
            _ => panic!("Not a f32 element!"),
        }
    }

    pub fn try_f64(&self) -> f64 {
        match self {
            Token::Float64(value) => *value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_c4(&self) -> &C4 {
        match self {
            Token::Char4(value) => value,
            _ => panic!("Not a c4 element!"),
        }
    }

    pub fn try_c256(&self) -> &C256 {
        match self {
            Token::Char256(value) => value,
            _ => panic!("Not a c256 element!"),
        }
    }

    pub fn try_str16(&self) -> &DynString<u16> {
        match self {
            Token::Str16(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_str32(&self) -> &DynString<u32> {
        match self {
            Token::Str32(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_map(&self) -> &LinkedHashMap {
        match self {
            Token::Union(value) => value,
            _ => panic!("Not a union element!"),
        }
    }

    pub fn try_vec(&mut self) -> &mut Vec<Token> {
        match self {
            Token::Vector(value) => value,
            _ => panic!("Not a vec element!"),
        }
    }
}

impl Serialize for Token {
    fn to_le_vec(&self) -> Vec<u8> {
        match self {
            Token::UInt8(value) => value.to_le_bytes().to_vec(),
            Token::UInt16(value) => value.to_le_vec(),
            Token::UInt32(value) => value.to_le_vec(),

            Token::Int8(value) => value.to_le_bytes().to_vec(),
            Token::Int16(value) => value.to_le_bytes().to_vec(),
            Token::Int32(value) => value.to_le_bytes().to_vec(),

            Token::Float32(value) => value.to_le_bytes().to_vec(),
            Token::Float64(value) => value.to_le_bytes().to_vec(),

            Token::Char4(str) => str.clone().to_le_vec(),
            Token::Char256(str) => str.clone().to_le_vec(),

            Token::Str16(string) => string.to_le_vec(),
            Token::Str32(string) => string.to_le_vec(),

            Token::Union(union) => union.to_le_vec(),
            Token::Vector(vec) => vec.to_le_vec(),
        }
    }
}
