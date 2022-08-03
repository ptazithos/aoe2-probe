use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap},
};

use super::{Resource, AI};

pub struct PlayerDataTwo {}

impl PlayerDataTwo {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(6);
        root.push_back(
            "strings",
            vec![DynString::with_capacity(0_u16, "").into(); 32],
        );
        root.push_back(
            "ai_names",
            vec![DynString::with_capacity(0_u16, "").into(); 16],
        );
        root.push_back("ai_files", vec![AI::template(); 16]);
        root.push_back("ai_type", vec![1_u8.into(); 16]);
        root.push_back("separator", 4294967197_u32);
        root.push_back("resources", vec![Resource::template(); 16]);
        root.into()
    }
}
