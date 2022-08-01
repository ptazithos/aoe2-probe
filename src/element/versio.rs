use crate::utils::{LinkedHashMap, PrefixString};

use super::serde::Serialize;

#[derive(Clone)]
pub enum Versio {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),

    Int8(i8),
    Int16(i16),
    Int32(i32),

    Float32(f32),
    Float64(f64),

    Str16(PrefixString<u16>),
    Str32(PrefixString<u32>),

    Union(LinkedHashMap),
}

impl Versio {
    pub fn try_u8(&self) -> u8 {
        match self {
            Versio::UInt8(value) => *value,
            _ => panic!("Not a u8 element!"),
        }
    }
    pub fn try_u16(&self) -> u16 {
        match self {
            Versio::UInt16(value) => *value,
            _ => panic!("Not a u16 element!"),
        }
    }
    pub fn try_u32(&self) -> u32 {
        match self {
            Versio::UInt32(value) => *value,
            _ => panic!("Not a u32 element!"),
        }
    }

    pub fn try_i8(&self) -> i8 {
        match self {
            Versio::Int8(value) => *value,
            _ => panic!("Not a u8 element!"),
        }
    }
    pub fn try_i16(&self) -> i16 {
        match self {
            Versio::Int16(value) => *value,
            _ => panic!("Not a u16 element!"),
        }
    }
    pub fn try_i32(&self) -> i32 {
        match self {
            Versio::Int32(value) => *value,
            _ => panic!("Not a u32 element!"),
        }
    }

    pub fn try_f32(&self) -> f32 {
        match self {
            Versio::Float32(value) => *value,
            _ => panic!("Not a f32 element!"),
        }
    }

    pub fn try_f64(&self) -> f64 {
        match self {
            Versio::Float64(value) => *value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_str16(&self) -> &PrefixString<u16> {
        match self {
            Versio::Str16(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_str32(&self) -> &PrefixString<u32> {
        match self {
            Versio::Str32(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_map(&self) -> &LinkedHashMap {
        match self {
            Versio::Union(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }
}

impl Serialize for Versio {
    fn to_le_vec(&self) -> Vec<u8> {
        match self {
            Versio::UInt8(value) => value.to_le_bytes().to_vec(),
            Versio::UInt16(value) => value.to_le_vec(),
            Versio::UInt32(value) => value.to_le_vec(),

            Versio::Int8(value) => value.to_le_bytes().to_vec(),
            Versio::Int16(value) => value.to_le_bytes().to_vec(),
            Versio::Int32(value) => value.to_le_bytes().to_vec(),

            Versio::Float32(value) => value.to_le_bytes().to_vec(),
            Versio::Float64(value) => value.to_le_bytes().to_vec(),

            Versio::Str16(string) => string.to_le_vec(),
            Versio::Str32(string) => string.to_le_vec(),

            Versio::Union(union) => union.to_le_vec(),
        }
    }
}
