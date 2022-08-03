use crate::{parse::Token, utils::LinkedHashMap};

pub struct Unit {}

impl Unit {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(9);
        root.push_back("x", 0.5 as f32);
        root.push_back("y", 0.5 as f32);
        root.push_back("z", 0.0 as f32);
        root.push_back("reference_id", 0 as i32);
        root.push_back("unit_const", 4 as u16);
        root.push_back("status", 2 as u8);
        root.push_back("rotation", 0.0 as f32);
        root.push_back("initial_animation_frame", 0 as u16);
        root.push_back("garrisoned_in_id", -1 as i32);

        root.into()
    }
}
