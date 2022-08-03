use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap},
};

pub struct Variable {}

impl Variable {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(2);
        root.push_back("variable_id", 0_u32);
        root.push_back("variable_name", DynString::with_capacity(0_u32, ""));

        root.into()
    }
}
