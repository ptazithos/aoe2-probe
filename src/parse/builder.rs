use super::serde::Deserialize;
use crate::io::Source;
use crate::utils::{DynString, LinkedHashMap, C256, C4};

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
                let patchs = &map.patchs;
                for (index, token) in map.iter().enumerate() {
                    let key = &keys[index];
                    let mut template = token.clone();
                    if patchs.contains_key(key) {
                        let patch = &patchs[key];
                        match patch.dep_type {
                            crate::utils::map::DepType::Exist => {
                                let flag = patch
                                    .source
                                    .iter()
                                    .map(|key| mock[key].try_u32() > 0)
                                    .fold(true, |sum, val| sum & val);

                                if !flag {
                                    template.try_vec().clear();
                                }
                            }
                            crate::utils::map::DepType::Calculate => match patch.manipulation {
                                crate::utils::map::Manipulation::Equal => {
                                    let dep_key = &patch.source[0];
                                    let num = mock[dep_key].try_u32();
                                    let vec = template.try_vec();
                                    let unit = vec[0].clone();
                                    vec.clear();
                                    for _ in 0..num {
                                        vec.push(unit.clone());
                                    }
                                }
                                crate::utils::map::Manipulation::Multiple => {
                                    let rkey = &patch.source[0];
                                    let lkey = &patch.source[1];

                                    let rnum = mock[rkey].try_u32();
                                    let lnum = mock[lkey].try_u32();

                                    let vec = template.try_vec();
                                    let unit = vec[0].clone();
                                    vec.clear();

                                    for _ in 0..lnum * rnum {
                                        vec.push(unit.clone());
                                    }
                                }
                            },
                        }
                    }
                    mock.push_back(key, TokenBuilder::create_from_template(&template, source));
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
