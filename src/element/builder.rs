use super::serde::Deserialize;
use crate::io::Source;
use crate::utils::{LinkedHashMap, PrefixString};

use super::wrap::Wrappable;
use super::Token;

pub struct TokenBuilder {}

impl TokenBuilder {
    pub fn create_from_template(template: &Token, source: &mut Source) -> Token {
        match template {
            Token::UInt8(_) => u8::from_le_vec(source).wrap(),
            Token::UInt16(_) => u16::from_le_vec(source).wrap(),
            Token::UInt32(_) => u32::from_le_vec(source).wrap(),
            Token::Int8(_) => i8::from_le_vec(source).wrap(),
            Token::Int16(_) => i16::from_le_vec(source).wrap(),
            Token::Int32(_) => i32::from_le_vec(source).wrap(),
            Token::Float32(_) => f32::from_le_vec(source).wrap(),
            Token::Float64(_) => f64::from_le_vec(source).wrap(),
            Token::Str16(_) => PrefixString::<u16>::from_le_vec(source).wrap(),
            Token::Str32(_) => PrefixString::<u32>::from_le_vec(source).wrap(),
            Token::Union(map) => {
                let mut root = LinkedHashMap::new();
                let keys = map.keys();
                for (index, token) in map.iter().enumerate() {
                    let key = &keys[index];
                    root.push_back(key, TokenBuilder::create_from_template(token, source));
                }
                root.wrap()
            }
        }
    }
}
