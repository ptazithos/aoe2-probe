use std::rc::Rc;

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
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("number_of_unit_sections") {
                    let count = *map["number_of_unit_sections"].try_u32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );
        root.into()
    }
}
