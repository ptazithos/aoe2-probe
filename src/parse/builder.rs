use super::serde::Deserialize;
use crate::io::Source;
use crate::utils::{C4,C256, DynString, LinkedHashMap};

use super::Token;

pub struct TokenBuilder {}

impl TokenBuilder {
    pub fn create_from_template(template: &Token, source: &mut Source) -> Token {
        match template {
            Token::UInt8(_) => u8::from_le_vec(source).into(),
            Token::UInt16(_) => u16::from_le_vec(source).into(),
            Token::UInt32(_) => u32::from_le_vec(source).into(),
            Token::Int8(_) => i8::from_le_vec(source).into(),
            Token::Int16(_) => i16::from_le_vec(source).into(),
            Token::Int32(_) => i32::from_le_vec(source).into(),
            Token::Float32(_) => f32::from_le_vec(source).into(),
            Token::Float64(_) => f64::from_le_vec(source).into(),
            Token::Char4(_) => C4::from_le_vec(source).into(),
            Token::Char256(_) => C256::from_le_vec(source).into(),
            Token::Str16(_) => DynString::<u16>::from_le_vec(source).into(),
            Token::Str32(_) => DynString::<u32>::from_le_vec(source).into(),
            Token::Union(map) => {
                let mut mock = LinkedHashMap::new();
                let keys = map.keys();
                for (index, token) in map.iter().enumerate() {
                    let key = &keys[index];
                    mock.push_back(key, TokenBuilder::create_from_template(token, source));
                }
                mock.into()
            }
            Token::Vector(vec) => {
                let mut mock = Vec::<Token>::new();
                for token in vec {
                    mock.push(Self::create_from_template(token, source))
                }

                mock.into()
            }
        }
    }
}
