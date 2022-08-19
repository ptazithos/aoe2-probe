use std::rc::Rc;

use crate::{parse::Token, utils::map::*};

pub struct Options;

impl Options {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(56);

        for index in 1..9 {
            let fix = index.to_string();
            root.push_back(&format!("number_of_disabled_techs_player_{}", &fix), 0_u32);
        }

        root.push_back("unknown_1", vec![0_u32.into(); 8]);

        for index in 1..9 {
            let fix = index.to_string();
            root.push_back(
                &format!("disabled_tech_ids_player_{}", &fix),
                vec![0_u32.into()],
            );

            root.patches.insert(
                format!("disabled_tech_ids_player_{}", &fix),
                Rc::new(move |map: &mut PatchedMap, template: &mut Token| {
                    let key = format!("number_of_disabled_techs_player_{}", index);
                    if map.contains(&key) {
                        let count = *map[&key].try_u32();
                        let unit = template.try_vec()[0].clone();
                        let vec = template.try_mut_vec();
                        vec.clear();

                        for _ in 0..count {
                            vec.push(unit.clone());
                        }
                    }
                }),
            );
        }

        for index in 1..9 {
            let fix = index.to_string();
            root.push_back(&format!("number_of_disabled_units_player_{}", &fix), 0_u32);
        }

        root.push_back("unknown_2", vec![0_u32.into(); 8]);

        for index in 1..9 {
            let fix = index.to_string();
            root.push_back(
                &format!("disabled_unit_ids_player_{}", &fix),
                vec![0_u32.into()],
            );

            root.patches.insert(
                format!("disabled_unit_ids_player_{}", &fix),
                Rc::new(move |map: &mut PatchedMap, template: &mut Token| {
                    let key = format!("number_of_disabled_units_player_{}", &fix);
                    if map.contains(&key) {
                        let count = *map[&key].try_u32();
                        let unit = template.try_vec()[0].clone();
                        let vec = template.try_mut_vec();
                        vec.clear();

                        for _ in 0..count {
                            vec.push(unit.clone());
                        }
                    }
                }),
            );
        }

        for index in 1..9 {
            let fix = index.to_string();
            root.push_back(
                &format!("{}{}", "number_of_disabled_buildings_player_", &fix),
                0_u32,
            );
        }

        root.push_back("unknown_3", vec![0_u32.into(); 8]);

        for index in 1..9 {
            let fix = index.to_string();
            root.push_back(
                &format!("disabled_building_ids_player_{}", &fix),
                vec![0_u32.into()],
            );

            root.patches.insert(
                format!("disabled_building_ids_player_{}", &fix),
                Rc::new(move |map: &mut PatchedMap, template: &mut Token| {
                    let key = format!("number_of_disabled_buildings_player_{}", &fix);
                    if map.contains(&key) {
                        let count = *map[&key].try_u32();
                        let unit = template.try_vec()[0].clone();
                        let vec = template.try_mut_vec();
                        vec.clear();

                        for _ in 0..count {
                            vec.push(unit.clone());
                        }
                    }
                }),
            );
        }

        root.push_back("combat_mode", 0_u32);
        root.push_back("naval_mode", 0_u32);
        root.push_back("all_techs", 0_u32);
        root.push_back("per_player_starting_age", vec![0_u32.into(); 16]);
        root.push_back("unknown_4", vec![0_u8.into(); 17]);
        root.push_back("per_player_base_priority", vec![0_u8.into(); 8]);
        root.push_back("unknown_5", vec![0_u8.into(); 7]);
        root.push_back("number_of_triggers", 0_u32);

        root.into()
    }
}
