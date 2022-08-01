use crate::{parse::{Token, wrap::Wrappable}, utils::LinkedHashMap};

pub struct Template{}

impl Template{
    fn file_header() -> Token{
        let root = LinkedHashMap::new();



        root.wrap()
    }
}