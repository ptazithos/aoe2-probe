use std::rc::Rc;

use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

pub struct PlayerDataFour;

impl PlayerDataFour {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(18);
        root.push_back("constant_name", DynString::with_capacity(0_u16, ""));
        root.push_back("editor_camera_x", 72.0_f32);
        root.push_back("editor_camera_y", 72.0_f32);
        root.push_back("initial_camera_x", 72_i16);
        root.push_back("initial_camera_y", 72_i16);
        root.push_back("aok_allied_victory", 0_u8);
        root.push_back("player_count_for_diplomacy", 9_u16);
        root.push_back(
            "diplomacy_for_interaction",
            vec![
                3_u8.into(),
                0_u8.into(),
                3_u8.into(),
                3_u8.into(),
                3_u8.into(),
                3_u8.into(),
                3_u8.into(),
                3_u8.into(),
                3_u8.into(),
            ],
        );

        root.patches.insert(
            "diplomacy_for_interaction".to_string(),
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("player_count_for_diplomacy") {
                    let count = *map["player_count_for_diplomacy"].try_u16();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.push_back(
            "diplomacy_for_ai_system",
            vec![
                0_u32.into(),
                1_u32.into(),
                4_u32.into(),
                4_u32.into(),
                4_u32.into(),
                4_u32.into(),
                4_u32.into(),
                4_u32.into(),
                4_u32.into(),
            ],
        );

        root.push_back("color", 0_u32);
        root.push_back("victory_version", 2.0_f32);
        root.push_back("unknown_1", 0_u16);
        root.push_back("unknown_2", vec![0_u8.into(); 7]);
        root.patches.insert(
            "unknown_2".to_string(),
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if *map["victory_version"].try_f32() != 2.0 {
                    template.try_mut_vec().clear();
                }
            }),
        );

        root.push_back(
            "unknown_structure_grand_theft_empires",
            vec![vec![0_u8.into(); 44].into(); 1],
        );

        root.patches.insert(
            "unknown_structure_grand_theft_empires".to_string(),
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("unknown_1") {
                    let count = *map["unknown_1"].try_u16();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.push_back("unknown_3", vec![0_u8.into(); 1]);

        root.patches.insert(
            "unknown_3".to_string(),
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if *map["victory_version"].try_f32() != 2.0 {
                    template.try_mut_vec().clear();
                }
            }),
        );

        root.push_back("unknown_4", vec![0_u8.into(); 7]);

        root.push_back(
            "unknown_structure_ww_campaign_2",
            vec![vec![0_u8.into(); 32].into(); 1],
        );

        root.patches.insert(
            "unknown_structure_ww_campaign_2".to_string(),
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("unknown_3") {
                    let count = *map["unknown_3"].try_vec()[0].try_u8();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.push_back("unknown_5", -1_i32);

        root.into()
    }
}
