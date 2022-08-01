use crate::utils::{LinkedHashMap, PrefixString};

use super::Versio;

pub trait Wrappable {
    fn wrap(self) -> Versio;
}

impl Wrappable for u8 {
    fn wrap(self) -> Versio {
        Versio::UInt8(self)
    }
}

impl Wrappable for u16 {
    fn wrap(self) -> Versio {
        Versio::UInt16(self)
    }
}

impl Wrappable for u32 {
    fn wrap(self) -> Versio {
        Versio::UInt32(self)
    }
}

impl Wrappable for i8 {
    fn wrap(self) -> Versio {
        Versio::Int8(self)
    }
}

impl Wrappable for i16 {
    fn wrap(self) -> Versio {
        Versio::Int16(self)
    }
}

impl Wrappable for i32 {
    fn wrap(self) -> Versio {
        Versio::Int32(self)
    }
}

impl Wrappable for f32 {
    fn wrap(self) -> Versio {
        Versio::Float32(self)
    }
}

impl Wrappable for f64 {
    fn wrap(self) -> Versio {
        Versio::Float64(self)
    }
}

impl Wrappable for PrefixString<u16> {
    fn wrap(self) -> Versio {
        Versio::Str16(self.clone())
    }
}

impl Wrappable for PrefixString<u32> {
    fn wrap(self) -> Versio {
        Versio::Str32(self.clone())
    }
}

impl Wrappable for LinkedHashMap {
    fn wrap(self) -> Versio {
        Versio::Union(self)
    }
}
