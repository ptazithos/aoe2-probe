use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap},
};

pub struct Message {}

impl Message {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(12);
        root.push_back("instructions", (4294967294 as u32).into());
        root.push_back("hints", (4294967294 as u32).into());
        root.push_back("victory", (4294967294 as u32).into());
        root.push_back("loss", (4294967294 as u32).into());
        root.push_back("history", (4294967294 as u32).into());
        root.push_back("scouts", (4294967294 as u32).into());
        root.push_back("ascii_instructions", DynString::new(0 as u16, "").into());
        root.push_back("ascii_hints", DynString::new(0 as u16, "").into());
        root.push_back("ascii_victory", DynString::new(0 as u16, "").into());
        root.push_back("ascii_loss", DynString::new(0 as u16, "").into());
        root.push_back("ascii_history", DynString::new(0 as u16, "").into());
        root.push_back("ascii_scouts", DynString::new(0 as u16, "").into());

        root.into()
    }
}
