use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap},
};

pub struct AI {}

impl AI {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(2);
        root.push_back("unknown", vec![(0 as u8).into(); 8]);
        root.push_back("ai_per_file_text", DynString::with_capacity(0 as u32, ""));

        root.into()
    }
}
