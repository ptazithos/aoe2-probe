use crate::{parse::Token, utils::LinkedHashMap};

pub struct Resource {}

impl Resource {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(7);
        root.push_back("gold", 0 as i32);
        root.push_back("wood", 0 as i32);
        root.push_back("food", 0 as i32);
        root.push_back("stone", 0 as i32);
        root.push_back("ore_x_unused", 0 as i32);
        root.push_back("trade_goods", 0 as i32);
        root.push_back("player_color", 0 as i32);

        root.into()
    }
}
