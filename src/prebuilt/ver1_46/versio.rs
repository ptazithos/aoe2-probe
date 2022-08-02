use crate::{
    parse::{wrap::Wrappable, Token},
    utils::LinkedHashMap,
};

use super::{Cinematics, DataHeader, FileHeader, Message};

pub struct Versio {}

#[allow(dead_code)]
impl Versio {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::new();
        root.push_back("file_header", FileHeader::template());
        root.push_back("data_header", DataHeader::template());
        root.push_back("message", Message::template());
        root.push_back("cinematics", Cinematics::template());

        root.wrap()
    }
}
