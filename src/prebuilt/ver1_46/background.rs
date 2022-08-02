use crate::{
    parse::{wrap::Wrappable, Token},
    utils::LinkedHashMap,
};

pub struct Background {}

impl Background {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(6);
        root.wrap()
    }
}
