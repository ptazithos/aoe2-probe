use crate::{parse::Token, utils::LinkedHashMap};

pub struct Terrain {}

impl Terrain {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(4);
        root.push_back("terrain_id", 0 as u8);
        root.push_back("elevation", 0 as u8);
        root.push_back("unused", vec![(0 as u8).into(); 3]);
        root.push_back("layer", -1 as i16);

        root.into()
    }
}
