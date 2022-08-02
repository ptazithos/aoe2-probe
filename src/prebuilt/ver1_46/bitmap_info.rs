use crate::{parse::Token, utils::map::*};

pub struct BitmapInfo {}

impl BitmapInfo {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(13);
        root.push_back("size", 0 as i32);
        root.push_back("width", 0 as u32);
        root.push_back("height", 0 as u32);
        root.push_back("planes", 0 as i16);
        root.push_back("bit_count", 0 as i16);
        root.push_back("compression", 0 as u32);
        root.push_back("image_size", 0 as u32);
        root.push_back("x_pels", 0 as u32);
        root.push_back("y_pels", 0 as u32);
        root.push_back("number_of_colors_used", 0 as u32);
        root.push_back("important_colors", 0 as u32);

        root.push_back("colors_used", vec![(0 as u32).into()]);
        root.patchs.insert(
            "colors_used".to_string(),
            NumericPatch {
                source: vec!["number_of_colors_used".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("image", vec![(0 as u8).into()]);
        root.patchs.insert(
            "image".to_string(),
            NumericPatch {
                source: vec!["width".to_string(), "height".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Multiple,
            },
        );

        root.into()
    }
}