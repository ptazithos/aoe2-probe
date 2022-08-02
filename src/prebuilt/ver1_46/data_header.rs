use crate::{
    parse::{wrap::Wrappable, Token},
    utils::{LinkedHashMap, C256},
};

use super::PlayerDataOne;

pub struct DataHeader {}

impl DataHeader {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::new();
        root.push_back("next_unit_id_to_place", (0 as u32).wrap());
        root.push_back("version", (1.42 as f32).wrap());
        root.push_back("tribe_names", vec![C256::new("").wrap(); 16].wrap());
        root.push_back(
            "string_table_player_names",
            vec![(4294967294 as u32).wrap(); 16].wrap(),
        );
        root.push_back("player_data_1", vec![PlayerDataOne::template(); 16].wrap());

        root.wrap()
    }
}
