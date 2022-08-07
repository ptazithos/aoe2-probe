use crate::{
    parse::Token,
    utils::{DynString, PatchedMap},
};

pub struct AI {}

impl AI {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(2);
        root.push_back("unknown", vec![0_u8.into(); 8]);
        root.push_back("ai_per_file_text", DynString::with_capacity(0_u32, ""));

        root.into()
    }
}
