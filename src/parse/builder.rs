use super::code::Decode;
use crate::io::Source;
use crate::utils::{DynString, PatchedMap, C256, C4};

use super::Token;

pub struct TokenBuilder {}

impl TokenBuilder {
    /// Build a token tree according to the given one.
    pub fn create_from_template(template: &Token, source: &mut Source) -> Result<Token, String> {
        match template {
            Token::UInt8(_) => Ok(u8::from_le_vec(source)?.into()),
            Token::UInt16(_) => Ok(u16::from_le_vec(source)?.into()),
            Token::UInt32(_) => Ok(u32::from_le_vec(source)?.into()),
            Token::Int8(_) => Ok(i8::from_le_vec(source)?.into()),
            Token::Int16(_) => Ok(i16::from_le_vec(source)?.into()),
            Token::Int32(_) => Ok(i32::from_le_vec(source)?.into()),
            Token::Float32(_) => Ok(f32::from_le_vec(source)?.into()),
            Token::Float64(_) => Ok(f64::from_le_vec(source)?.into()),
            Token::Char4(_) => Ok(C4::from_le_vec(source)?.into()),
            Token::Char256(_) => Ok(C256::from_le_vec(source)?.into()),
            Token::Str16(_) => Ok(DynString::<u16>::from_le_vec(source)?.into()),
            Token::Str32(_) => Ok(DynString::<u32>::from_le_vec(source)?.into()),
            Token::Union(map) => {
                let mut mock = PatchedMap::new();
                let patches = &map.patches;
                for (key, token) in map.iter() {
                    let mut template = token.clone();
                    if patches.contains_key(key) {
                        let patch = &patches[key];
                        patch(&mut mock, &mut template);
                    }
                    mock.push_back(key, TokenBuilder::create_from_template(&template, source)?);
                }
                Ok(mock.into())
            }
            Token::Vector(vec) => {
                let mut mock = Vec::<Token>::new();
                for token in vec {
                    mock.push(Self::create_from_template(token, source)?)
                }

                Ok(mock.into())
            }
        }
    }
}
