use crate::utils::{
    C4, C256, DynString, LinkedHashMap,
};

use super::Token;

pub trait Wrappable {
    fn wrap(self) -> Token;
}

impl Wrappable for u8 {
    fn wrap(self) -> Token {
        Token::UInt8(self)
    }
}

impl Wrappable for u16 {
    fn wrap(self) -> Token {
        Token::UInt16(self)
    }
}

impl Wrappable for u32 {
    fn wrap(self) -> Token {
        Token::UInt32(self)
    }
}

impl Wrappable for i8 {
    fn wrap(self) -> Token {
        Token::Int8(self)
    }
}

impl Wrappable for i16 {
    fn wrap(self) -> Token {
        Token::Int16(self)
    }
}

impl Wrappable for i32 {
    fn wrap(self) -> Token {
        Token::Int32(self)
    }
}

impl Wrappable for f32 {
    fn wrap(self) -> Token {
        Token::Float32(self)
    }
}

impl Wrappable for f64 {
    fn wrap(self) -> Token {
        Token::Float64(self)
    }
}

impl Wrappable for C4 {
    fn wrap(self) -> Token {
        Token::Char4(self)
    }
}

impl Wrappable for C256 {
    fn wrap(self) -> Token {
        Token::Char256(self)
    }
}

impl Wrappable for DynString<u16> {
    fn wrap(self) -> Token {
        Token::Str16(self.clone())
    }
}

impl Wrappable for DynString<u32> {
    fn wrap(self) -> Token {
        Token::Str32(self.clone())
    }
}

impl Wrappable for LinkedHashMap {
    fn wrap(self) -> Token {
        Token::Union(self)
    }
}

impl Wrappable for Vec<Token> {
    fn wrap(self) -> Token {
        Token::Vector(self)
    }
}
