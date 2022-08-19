use std::sync::Arc;

use crate::{parse::Token, utils::map::*};

pub struct BitmapInfo;

impl BitmapInfo {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(13);
        root.push_back("size", 0_i32);
        root.push_back("width", 0_u32);
        root.push_back("height", 0_u32);
        root.push_back("planes", 0_i16);
        root.push_back("bit_count", 0_i16);
        root.push_back("compression", 0_u32);
        root.push_back("image_size", 0_u32);
        root.push_back("x_pels", 0_u32);
        root.push_back("y_pels", 0_u32);
        root.push_back("number_of_colors_used", 0_u32);
        root.push_back("important_colors", 0_u32);

        root.push_back("colors_used", vec![0_u32.into()]);

        root.patches.insert(
            "colors_used".to_string(),
            Arc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("number_of_colors_used") {
                    let count = *map["number_of_colors_used"].try_u32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.push_back("image", vec![0_u8.into()]);
        root.patches.insert(
            "image".to_string(),
            Arc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("width") && map.contains("height") {
                    let count = *map["width"].try_u32() * *map["height"].try_u32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.into()
    }
}
