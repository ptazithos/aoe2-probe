use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap},
};

pub struct Message {}

impl Message {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(12);
        root.push_back("instructions", 4294967294_u32);
        root.push_back("hints", 4294967294_u32);
        root.push_back("victory", 4294967294_u32);
        root.push_back("loss", 4294967294_u32);
        root.push_back("history", 4294967294_u32);
        root.push_back("scouts", 4294967294_u32);
        root.push_back("ascii_instructions", DynString::with_capacity(0_u16, ""));
        root.push_back("ascii_hints", DynString::with_capacity(0_u16, ""));
        root.push_back("ascii_victory", DynString::with_capacity(0_u16, ""));
        root.push_back("ascii_loss", DynString::with_capacity(0_u16, ""));
        root.push_back("ascii_history", DynString::with_capacity(0_u16, ""));
        root.push_back("ascii_scouts", DynString::with_capacity(0_u16, ""));

        root.into()
    }
}
