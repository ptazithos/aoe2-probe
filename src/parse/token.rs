use serde::{Deserialize, Serialize};

use crate::utils::{DynString, PatchedMap, C256, C4};

use super::code::Encode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Token {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),

    Int8(i8),
    Int16(i16),
    Int32(i32),

    Float32(f32),
    Float64(f64),

    Char4(C4),
    Char256(C256),

    Str16(DynString<u16>),
    Str32(DynString<u32>),

    Union(PatchedMap),
    Vector(Vec<Token>),
}

impl Token {
    /// Retrieve an immutable value by the given path.
    /// The return value will be a Token enum.
    /// using token.try_[type] function to get the actual data.
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::Scenario;
    ///
    /// let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    /// //Immutable borrow author string.
    /// let author = scenario.versio.get_by_path("/file_header/creator_name").try_str32();
    /// ```
    pub fn get_by_path(&self, path: &str) -> &Token {
        let keys: Vec<&str> = path.split('/').filter(|str| !str.is_empty()).collect();
        let mut value = self;
        for key in keys {
            value = &value.try_map()[key];
        }
        value
    }

    /// Retrieve a mutable value by the given path.
    /// The return value will be a Token enum.
    /// using token.try_mut_[type] function to get the actual data.
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::Scenario;
    ///
    /// //Mutable borrow author string.
    /// let mut scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    /// let author = scenario.versio.get_by_path_mut("/file_header/creator_name").try_mut_str32();
    /// author.set_content("Arian");
    /// ```
    pub fn get_by_path_mut(&mut self, path: &str) -> &mut Token {
        let keys: Vec<&str> = path.split('/').filter(|str| !str.is_empty()).collect();
        let mut value = self;
        for key in keys {
            value = &mut value.try_mut_map()[key];
        }
        value
    }

    pub fn try_u8(&self) -> &u8 {
        match self {
            Token::UInt8(value) => value,
            _ => panic!("Not a u8 element!"),
        }
    }

    pub fn try_mut_u8(&mut self) -> &mut u8 {
        match self {
            Token::UInt8(value) => value,
            _ => panic!("Not a u8 element!"),
        }
    }

    pub fn try_u16(&self) -> &u16 {
        match self {
            Token::UInt16(value) => value,
            _ => panic!("Not a u16 element!"),
        }
    }

    pub fn try_mut_u16(&mut self) -> &mut u16 {
        match self {
            Token::UInt16(value) => value,
            _ => panic!("Not a u16 element!"),
        }
    }

    pub fn try_u32(&self) -> &u32 {
        match self {
            Token::UInt32(value) => value,
            _ => panic!("Not a u32 element!"),
        }
    }

    pub fn try_mut_u32(&mut self) -> &mut u32 {
        match self {
            Token::UInt32(value) => value,
            _ => panic!("Not a u32 element!"),
        }
    }

    pub fn try_i8(&self) -> &i8 {
        match self {
            Token::Int8(value) => value,
            _ => panic!("Not a u8 element!"),
        }
    }

    pub fn try_mut_i8(&mut self) -> &mut i8 {
        match self {
            Token::Int8(value) => value,
            _ => panic!("Not a u8 element!"),
        }
    }

    pub fn try_i16(&self) -> &i16 {
        match self {
            Token::Int16(value) => value,
            _ => panic!("Not a u16 element!"),
        }
    }

    pub fn try_mut_i16(&mut self) -> &mut i16 {
        match self {
            Token::Int16(value) => value,
            _ => panic!("Not a u16 element!"),
        }
    }

    pub fn try_i32(&self) -> &i32 {
        match self {
            Token::Int32(value) => value,
            _ => panic!("Not a i32 element!"),
        }
    }

    pub fn try_mut_i32(&mut self) -> &mut i32 {
        match self {
            Token::Int32(value) => value,
            _ => panic!("Not a i32 element!"),
        }
    }

    pub fn try_f32(&self) -> &f32 {
        match self {
            Token::Float32(value) => value,
            _ => panic!("Not a f32 element!"),
        }
    }

    pub fn try_mut_f32(&mut self) -> &mut f32 {
        match self {
            Token::Float32(value) => value,
            _ => panic!("Not a f32 element!"),
        }
    }

    pub fn try_f64(&self) -> &f64 {
        match self {
            Token::Float64(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_mut_f64(&mut self) -> &mut f64 {
        match self {
            Token::Float64(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_c4(&self) -> &C4 {
        match self {
            Token::Char4(value) => value,
            _ => panic!("Not a c4 element!"),
        }
    }

    pub fn try_mut_c4(&mut self) -> &mut C4 {
        match self {
            Token::Char4(value) => value,
            _ => panic!("Not a c4 element!"),
        }
    }

    pub fn try_c256(&self) -> &C256 {
        match self {
            Token::Char256(value) => value,
            _ => panic!("Not a c256 element!"),
        }
    }

    pub fn try_mut_c256(&mut self) -> &mut C256 {
        match self {
            Token::Char256(value) => value,
            _ => panic!("Not a c256 element!"),
        }
    }

    pub fn try_str16(&self) -> &DynString<u16> {
        match self {
            Token::Str16(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_mut_str16(&mut self) -> &mut DynString<u16> {
        match self {
            Token::Str16(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_str32(&self) -> &DynString<u32> {
        match self {
            Token::Str32(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_mut_str32(&mut self) -> &mut DynString<u32> {
        match self {
            Token::Str32(value) => value,
            _ => panic!("Not a f64 element!"),
        }
    }

    pub fn try_map(&self) -> &PatchedMap {
        match self {
            Token::Union(value) => value,
            _ => panic!("Not a union element!"),
        }
    }

    pub fn try_mut_map(&mut self) -> &mut PatchedMap {
        match self {
            Token::Union(value) => value,
            _ => panic!("Not a union element!"),
        }
    }

    pub fn try_vec(&self) -> &Vec<Token> {
        match self {
            Token::Vector(value) => value,
            _ => panic!("Not a vec element!"),
        }
    }

    pub fn try_mut_vec(&mut self) -> &mut Vec<Token> {
        match self {
            Token::Vector(value) => value,
            _ => panic!("Not a vec element!"),
        }
    }

    pub fn try_compatible_u64(&self) -> u64 {
        match self {
            Token::UInt8(num) => *num as u64,
            Token::UInt16(num) => *num as u64,
            Token::UInt32(num) => *num as u64,
            Token::Int8(num) => {
                if *num >= 0 {
                    *num as u64
                } else {
                    0
                }
            }
            Token::Int16(num) => {
                if *num >= 0 {
                    *num as u64
                } else {
                    0
                }
            }
            Token::Int32(num) => {
                if *num >= 0 {
                    *num as u64
                } else {
                    0
                }
            }
            Token::Float32(num) => {
                if *num == 2.0 {
                    1
                } else {
                    0
                }
            }
            Token::Float64(num) => *num as u64,
            Token::Vector(vec) => {
                if !vec.is_empty() {
                    vec[0].try_compatible_u64()
                } else {
                    0
                }
            }
            _ => panic!("Not compatible value!"),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UInt8(_), Self::UInt8(_)) => true,
            (Self::UInt16(_), Self::UInt16(_)) => true,
            (Self::UInt32(_), Self::UInt32(_)) => true,
            (Self::Int8(_), Self::Int8(_)) => true,
            (Self::Int16(_), Self::Int16(_)) => true,
            (Self::Int32(_), Self::Int32(_)) => true,
            (Self::Float32(_), Self::Float32(_)) => true,
            (Self::Float64(_), Self::Float64(_)) => true,
            (Self::Char4(_), Self::Char4(_)) => true,
            (Self::Char256(_), Self::Char256(_)) => true,
            (Self::Str16(_), Self::Str16(_)) => true,
            (Self::Str32(_), Self::Str32(_)) => true,
            (Self::Union(_), Self::Union(_)) => true,
            (Self::Vector(_), Self::Vector(_)) => true,
            (_, _) => false,
        }
    }
}

impl Encode for Token {
    fn to_le_vec(&self) -> Vec<u8> {
        match self {
            Token::UInt8(value) => value.to_le_bytes().to_vec(),
            Token::UInt16(value) => value.to_le_vec(),
            Token::UInt32(value) => value.to_le_vec(),

            Token::Int8(value) => value.to_le_bytes().to_vec(),
            Token::Int16(value) => value.to_le_bytes().to_vec(),
            Token::Int32(value) => value.to_le_bytes().to_vec(),

            Token::Float32(value) => value.to_le_bytes().to_vec(),
            Token::Float64(value) => value.to_le_bytes().to_vec(),

            Token::Char4(str) => str.clone().to_le_vec(),
            Token::Char256(str) => str.clone().to_le_vec(),

            Token::Str16(string) => string.to_le_vec(),
            Token::Str32(string) => string.to_le_vec(),

            Token::Union(union) => union.to_le_vec(),
            Token::Vector(vec) => vec.to_le_vec(),
        }
    }
}

impl From<u8> for Token {
    fn from(uint8: u8) -> Self {
        Token::UInt8(uint8)
    }
}

impl From<u16> for Token {
    fn from(uint16: u16) -> Self {
        Token::UInt16(uint16)
    }
}

impl From<u32> for Token {
    fn from(uint32: u32) -> Self {
        Token::UInt32(uint32)
    }
}

impl From<i8> for Token {
    fn from(int8: i8) -> Self {
        Token::Int8(int8)
    }
}

impl From<i16> for Token {
    fn from(int16: i16) -> Self {
        Token::Int16(int16)
    }
}

impl From<i32> for Token {
    fn from(int32: i32) -> Self {
        Token::Int32(int32)
    }
}

impl From<f32> for Token {
    fn from(float32: f32) -> Self {
        Token::Float32(float32)
    }
}

impl From<f64> for Token {
    fn from(float64: f64) -> Self {
        Token::Float64(float64)
    }
}

impl From<C4> for Token {
    fn from(char4: C4) -> Self {
        Token::Char4(char4)
    }
}

impl From<C256> for Token {
    fn from(char256: C256) -> Self {
        Token::Char256(char256)
    }
}

impl From<DynString<u16>> for Token {
    fn from(str16: DynString<u16>) -> Self {
        Token::Str16(str16)
    }
}

impl From<DynString<u32>> for Token {
    fn from(str32: DynString<u32>) -> Self {
        Token::Str32(str32)
    }
}

impl From<PatchedMap> for Token {
    fn from(map: PatchedMap) -> Self {
        Token::Union(map)
    }
}

impl From<Vec<Token>> for Token {
    fn from(vec: Vec<Token>) -> Self {
        Token::Vector(vec)
    }
}
