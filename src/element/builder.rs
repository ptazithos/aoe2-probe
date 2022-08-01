use super::serde::Deserialize;
use crate::io::Source;
use crate::utils::{LinkedHashMap, PrefixString};

use super::wrap::Wrappable;
use super::Versio;

pub struct VersioBuilder {}

impl VersioBuilder {
    pub fn create_from_template(template: &Versio, source: &mut Source) -> Versio {
        match template {
            Versio::UInt8(_) => u8::from_le_vec(source).wrap(),
            Versio::UInt16(_) => u16::from_le_vec(source).wrap(),
            Versio::UInt32(_) => u32::from_le_vec(source).wrap(),
            Versio::Int8(_) => i8::from_le_vec(source).wrap(),
            Versio::Int16(_) => i16::from_le_vec(source).wrap(),
            Versio::Int32(_) => i32::from_le_vec(source).wrap(),
            Versio::Float32(_) => f32::from_le_vec(source).wrap(),
            Versio::Float64(_) => f64::from_le_vec(source).wrap(),
            Versio::Str16(_) => PrefixString::<u16>::from_le_vec(source).wrap(),
            Versio::Str32(_) => PrefixString::<u32>::from_le_vec(source).wrap(),
            Versio::Union(map) => {
                let mut root = LinkedHashMap::new();
                let keys = map.keys();
                for (index, versio) in map.iter().enumerate() {
                    let key = &keys[index];
                    root.push_back(key, VersioBuilder::create_from_template(versio, source));
                }
                root.wrap()
            }
        }
    }
}
