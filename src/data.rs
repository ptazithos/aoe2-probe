use crate::serializer::{Serializable, StringPrefix};
use std::fmt;

pub struct PrefixString<T: StringPrefix + Copy + Serializable + fmt::Debug> {
    prefix: T,
    pub raw: String,
}

impl<T: StringPrefix + Copy + Serializable + fmt::Debug> PrefixString<T> {
    pub fn new(prefix_length: T, raw: &str) -> PrefixString<T> {
        PrefixString {
            prefix: prefix_length,
            raw: raw.to_string(),
        }
    }

    pub fn prefix_length(&self) -> T {
        self.prefix
    }
}

impl<T: StringPrefix + Copy + Serializable + fmt::Debug> Serializable for PrefixString<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        buffer.append(&mut self.prefix.to_bytes());
        buffer.append(&mut self.raw.to_bytes());
        buffer
    }
}

impl<T: StringPrefix + Copy + Serializable + fmt::Debug> fmt::Debug for PrefixString<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("BitMapInfoIsh")
            .field("len", &self.prefix_length())
            .field("content", &self.raw)
            .finish()
    }
}
