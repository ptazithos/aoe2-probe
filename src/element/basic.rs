use crate::utils::map::LinkedHashMap;

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

    Union(LinkedHashMap),
}

impl Versio {
    pub fn try_f32(&self) -> f32 {
        match self {
            Versio::Float32(value) => *value,
            _ => panic!("Not a f32 element!"),
        }
    }
}
