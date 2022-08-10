use crate::{parse::Token, utils::PatchedMap};

pub struct Unit {}

impl Unit {
    pub fn template() -> Token {
        Self::default()
    }

    pub fn default() -> Token {
        let mut root = PatchedMap::with_capacity(9);
        root.push_back("x", 0.5_f32);
        root.push_back("y", 0.5_f32);
        root.push_back("z", 0.0_f32);
        root.push_back("reference_id", 0_i32);
        root.push_back("unit_const", 4_u16);
        root.push_back("status", 2_u8);
        root.push_back("rotation", 0.0_f32);
        root.push_back("initial_animation_frame", 0_u16);
        root.push_back("garrisoned_in_id", -1_i32);

        root.into()
    }
}
