use crate::{
    parse::Token,
    utils::{DynString, PatchedMap},
};

pub struct AI2 {}

impl AI2 {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(2);
        root.push_back("ai_file_name", DynString::with_capacity(0_u32, ""));
        root.push_back("ai_file", DynString::with_capacity(0_u32, ""));

        root.into()
    }
}
