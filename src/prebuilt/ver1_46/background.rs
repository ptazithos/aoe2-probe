use std::sync::Arc;

use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

use super::BitmapInfo;

pub struct Background;

impl Background {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(6);
        root.push_back("ascii_filename", DynString::with_capacity(0_u16, ""));
        root.push_back("picture_version", 3_u32);
        root.push_back("bitmap_width", 0_u32);
        root.push_back("bitmap_height", 0_u32);
        root.push_back("picture_orientation", 0_i16);

        root.push_back("bitmap_info", vec![BitmapInfo::template()]);

        root.patches.insert(
            "bitmap_info".to_string(),
            Arc::new(|map: &mut PatchedMap, template: &mut Token| {
                if *map["bitmap_width"].try_u32() == 0 || *map["bitmap_height"].try_u32() == 0 {
                    template.try_mut_vec().clear();
                }
            }),
        );

        root.into()
    }
}
