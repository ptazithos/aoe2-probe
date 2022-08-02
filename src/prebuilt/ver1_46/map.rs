use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

use super::Terrain;

pub struct Map {}

impl Map {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(19);
        root.push_back("starter_1", 0 as u16);
        root.push_back("water_definition", DynString::new(0 as u16, ""));
        root.push_back("starter_2", 0 as u16);
        root.push_back("map_color_mood", DynString::new(5 as u16, "Empty"));
        root.push_back("starter_3", 0 as u16);
        root.push_back("script_name", DynString::new(5 as u16, ""));
        root.push_back("collide_and_correct", 0 as u8);
        root.push_back("villager_force_drop", 0 as u8);
        root.push_back("unknown_1", vec![(255 as u8).into(); 128]);
        root.push_back("lock_coop_alliances", 0 as u8);
        root.push_back("ai_map_type", 0 as i32);
        root.push_back("per_player_population_cap", vec![(200 as u32).into(); 16]);
        root.push_back("secondary_game_mode", 0 as u32);
        root.push_back("unknown_2", vec![(0 as u8).into(); 4]);
        root.push_back("unknown_3", vec![(0 as u8).into(); 4]);
        root.push_back("no_waves_on_shore", 0 as i8);
        root.push_back("map_width", 120 as u32);
        root.push_back("map_height", 120 as u32);
        root.push_back("terrain_data", vec![Terrain::template(); 1]);
        root.patchs.insert(
            "terrain_data".to_string(),
            NumericPatch {
                source: vec!["map_width".to_string(), "map_height".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Multiple,
            },
        );

        root.into()
    }
}
