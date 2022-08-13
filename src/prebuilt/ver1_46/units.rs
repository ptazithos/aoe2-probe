use crate::{parse::Token, utils::map::*};

use super::{PlayerDataFour, PlayerDataThree, PlayerUnits};

pub struct Units;

impl Units {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(5);
        root.push_back("number_of_unit_sections", 9_u32);
        root.push_back("player_data_3", vec![PlayerDataThree::template(); 8]);
        root.push_back("number_of_players", 9_u32);
        root.push_back("player_data_4", vec![PlayerDataFour::template(); 8]);
        root.push_back("players_units", vec![PlayerUnits::template(); 1]);
        root.patches.insert(
            "players_units".to_string(),
            NumericPatch {
                source: vec!["number_of_unit_sections".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );
        root.into()
    }
}
