use crate::utils::map::LinkedHashMap;

use super::Versio;

pub trait Serialize {
    fn to_le_vec(&self) -> Vec<u8>;
}

pub trait Deserialize {
    fn from_le_vec(vec: Vec<u8>) -> Self;
}

impl Serialize for LinkedHashMap {
    fn to_le_vec(&self) -> Vec<u8> {
        self.iter().map(|ele| ele.to_le_vec()).flatten().collect()
    }
}

impl Serialize for Versio {
    fn to_le_vec(&self) -> Vec<u8> {
        match self {
            Versio::UInt8(value) => value.to_le_bytes().to_vec(),
            Versio::UInt16(value) => value.to_le_bytes().to_vec(),
            Versio::UInt32(value) => value.to_le_bytes().to_vec(),

            Versio::Int8(value) => value.to_le_bytes().to_vec(),
            Versio::Int16(value) => value.to_le_bytes().to_vec(),
            Versio::Int32(value) => value.to_le_bytes().to_vec(),

            Versio::Float32(value) => value.to_le_bytes().to_vec(),
            Versio::Float64(value) => value.to_le_bytes().to_vec(),
            Versio::Union(union) => union.to_le_vec(),
        }
    }
}
