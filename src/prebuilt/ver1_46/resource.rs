use crate::{parse::Token, utils::PatchedMap};

pub struct Resource {}

impl Resource {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(7);
        root.push_back("gold", 0_i32);
        root.push_back("wood", 0_i32);
        root.push_back("food", 0_i32);
        root.push_back("stone", 0_i32);
        root.push_back("ore_x_unused", 0_i32);
        root.push_back("trade_goods", 0_i32);
        root.push_back("player_color", 0_i32);

        root.into()
    }
}
