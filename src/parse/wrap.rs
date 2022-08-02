use crate::utils::{DynString, LinkedHashMap, C256, C4};

use super::Token;

impl Into<Token> for u8 {
    fn into(self) -> Token {
        Token::UInt8(self)
    }
}

impl Into<Token> for u16 {
    fn into(self) -> Token {
        Token::UInt16(self)
    }
}

impl Into<Token> for u32 {
    fn into(self) -> Token {
        Token::UInt32(self)
    }
}

impl Into<Token> for i8 {
    fn into(self) -> Token {
        Token::Int8(self)
    }
}

impl Into<Token> for i16 {
    fn into(self) -> Token {
        Token::Int16(self)
    }
}

impl Into<Token> for i32 {
    fn into(self) -> Token {
        Token::Int32(self)
    }
}

impl Into<Token> for f32 {
    fn into(self) -> Token {
        Token::Float32(self)
    }
}

impl Into<Token> for f64 {
    fn into(self) -> Token {
        Token::Float64(self)
    }
}

impl Into<Token> for C4 {
    fn into(self) -> Token {
        Token::Char4(self)
    }
}

impl Into<Token> for C256 {
    fn into(self) -> Token {
        Token::Char256(self)
    }
}

impl Into<Token> for DynString<u16> {
    fn into(self) -> Token {
        Token::Str16(self.clone())
    }
}

impl Into<Token> for DynString<u32> {
    fn into(self) -> Token {
        Token::Str32(self.clone())
    }
}

impl Into<Token> for LinkedHashMap {
    fn into(self) -> Token {
        Token::Union(self)
    }
}

impl Into<Token> for Vec<Token> {
    fn into(self) -> Token {
        Token::Vector(self)
    }
}
