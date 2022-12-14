use crate::{parse::Token, utils::PatchedMap};

pub struct Terrain;

impl Terrain {
    pub fn template() -> Token {
        Self::default()
    }

    pub fn default() -> Token {
        let mut root = PatchedMap::with_capacity(4);
        root.push_back("terrain_id", 0_u8);
        root.push_back("elevation", 0_u8);
        root.push_back("unused", vec![0_u8.into(); 3]);
        root.push_back("layer", -1_i16);

        root.into()
    }
}
