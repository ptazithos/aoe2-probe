use crate::{parse::Token, utils::PatchedMap};

pub struct PlayerDataThree;

impl PlayerDataThree {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(7);
        root.push_back("food_duplicate", 0.0_f32);
        root.push_back("wood_duplicate", 0.0_f32);
        root.push_back("gold_duplicate", 0.0_f32);
        root.push_back("stone_duplicate", 0.0_f32);
        root.push_back("ore_x_duplicate", 0.0_f32);
        root.push_back("trade_goods_duplicate", 0.0_f32);
        root.push_back("population_limit", 0.0_f32);

        root.into()
    }
}
