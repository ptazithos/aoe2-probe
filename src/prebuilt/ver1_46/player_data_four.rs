use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

pub struct PlayerDataFour {}

impl PlayerDataFour {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(18);
        root.push_back("constant_name", DynString::with_capacity(0 as u16, ""));
        root.push_back("editor_camera_x", 72.0 as f32);
        root.push_back("editor_camera_y", 72.0 as f32);
        root.push_back("initial_camera_x", 72 as i16);
        root.push_back("initial_camera_y", 72 as i16);
        root.push_back("aok_allied_victory", 0 as u8);
        root.push_back("player_count_for_diplomacy", 9 as u16);
        root.push_back(
            "diplomacy_for_interaction",
            vec![
                (3 as u8).into(),
                (0 as u8).into(),
                (3 as u8).into(),
                (3 as u8).into(),
                (3 as u8).into(),
                (3 as u8).into(),
                (3 as u8).into(),
                (3 as u8).into(),
                (3 as u8).into(),
            ],
        );
        root.patchs.insert(
            "diplomacy_for_interaction".to_string(),
            NumericPatch {
                source: vec!["player_count_for_diplomacy".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back(
            "diplomacy_for_ai_system",
            vec![
                (0 as u32).into(),
                (1 as u32).into(),
                (4 as u32).into(),
                (4 as u32).into(),
                (4 as u32).into(),
                (4 as u32).into(),
                (4 as u32).into(),
                (4 as u32).into(),
                (4 as u32).into(),
            ],
        );

        root.push_back("color", 0 as u32);
        root.push_back("victory_version", 2.0 as f32);
        root.push_back("unknown_1", 0 as u16);
        root.push_back("unknown_2", vec![(0 as u8).into(); 7]);
        root.patchs.insert(
            "unknown_2".to_string(),
            NumericPatch {
                source: vec!["victory_version".to_string()],
                dep_type: DepType::Exist,
                manipulation: Manipulation::Multiple,
            },
        );

        root.push_back(
            "unknown_structure_grand_theft_empires",
            vec![vec![(0 as u8).into(); 44].into(); 1],
        );

        root.patchs.insert(
            "unknown_structure_grand_theft_empires".to_string(),
            NumericPatch {
                source: vec!["unknown_1".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("unknown_3", vec![(0 as u8).into(); 1]);
        root.patchs.insert(
            "unknown_3".to_string(),
            NumericPatch {
                source: vec!["victory_version".to_string()],
                dep_type: DepType::Exist,
                manipulation: Manipulation::Multiple,
            },
        );

        root.push_back("unknown_4", vec![(0 as u8).into(); 7]);

        root.push_back(
            "unknown_structure_ww_campaign_2",
            vec![vec![(0 as u8).into(); 32].into(); 1],
        );

        root.patchs.insert(
            "unknown_structure_ww_campaign_2".to_string(),
            NumericPatch {
                source: vec!["unknown_3".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("unknown_5", -1 as i32);

        root.into()
    }
}
