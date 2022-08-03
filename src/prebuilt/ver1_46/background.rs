use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

use super::BitmapInfo;

pub struct Background {}

impl Background {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(6);
        root.push_back("ascii_filename", DynString::with_capacity(0 as u16, ""));
        root.push_back("picture_version", 3 as u32);
        root.push_back("bitmap_width", 0 as u32);
        root.push_back("bitmap_height", 0 as u32);
        root.push_back("picture_orientation", 0 as i16);

        root.push_back("bitmap_info", BitmapInfo::template());
        root.patchs.insert(
            "image".to_string(),
            NumericPatch {
                source: vec!["bitmap_width".to_string(), "bitmap_height".to_string()],
                dep_type: DepType::Exist,
                manipulation: Manipulation::Multiple,
            },
        );

        root.into()
    }
}
