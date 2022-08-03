use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap},
};

pub struct Variable {}

impl Variable {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(2);
        root.push_back("variable_id", 0 as u32);
        root.push_back("variable_name", DynString::new(0 as u32, ""));

        root.into()
    }
}
