use std::rc::Rc;

use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

use super::Terrain;

pub struct Map;

impl Map {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(19);
        root.push_back("starter_1", 0_u16);
        root.push_back("water_definition", DynString::with_capacity(0_u16, ""));
        root.push_back("starter_2", 0_u16);
        root.push_back("map_color_mood", DynString::with_capacity(5_u16, "Empty"));
        root.push_back("starter_3", 0_u16);
        root.push_back("script_name", DynString::with_capacity(0_u16, ""));
        root.push_back("collide_and_correct", 0_u8);
        root.push_back("villager_force_drop", 0_u8);
        root.push_back("unknown_1", vec![255_u8.into(); 128]);
        root.push_back("lock_coop_alliances", 0_u8);
        root.push_back("ai_map_type", 0_i32);
        root.push_back("per_player_population_cap", vec![(200_u32).into(); 16]);
        root.push_back("secondary_game_mode", 0_u32);
        root.push_back("unknown_2", vec![0_u8.into(); 4]);
        root.push_back("unknown_3", vec![0_u8.into(); 4]);
        root.push_back("no_waves_on_shore", 0_i8);
        root.push_back("map_width", 120_u32);
        root.push_back("map_height", 120_u32);
        root.push_back("terrain_data", vec![Terrain::template(); 1]);

        root.patches.insert(
            "terrain_data".to_string(),
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("map_width") && map.contains("map_height") {
                    let count = *map["map_width"].try_u32() * *map["map_width"].try_u32();
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
