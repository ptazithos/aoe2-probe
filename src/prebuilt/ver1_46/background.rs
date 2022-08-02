use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap},
};

pub struct Background {}

impl Background {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(6);
        root.push_back("ascii_filename", DynString::new(0 as u16, ""));
        root.push_back("picture_version", 3 as u32);
        root.push_back("bitmap_width", 0 as u32);
        root.push_back("bitmap_height", 0 as i32);
        root.push_back("picture_orientation", 0 as i16);
        root.into()
    }
}
