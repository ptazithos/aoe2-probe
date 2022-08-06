use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

pub struct PlayerDataFour {}

impl PlayerDataFour {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(18);
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
            NumericPatch {
                source: vec!["player_count_for_diplomacy".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
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
            NumericPatch {
                source: vec!["victory_version".to_string()],
                dep_type: DepType::Exist,
                manipulation: Manipulation::Multiple,
            },
        );

        root.push_back(
            "unknown_structure_grand_theft_empires",
            vec![vec![0_u8.into(); 44].into(); 1],
        );

        root.patches.insert(
            "unknown_structure_grand_theft_empires".to_string(),
            NumericPatch {
                source: vec!["unknown_1".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("unknown_3", vec![0_u8.into(); 1]);
        root.patches.insert(
            "unknown_3".to_string(),
            NumericPatch {
                source: vec!["victory_version".to_string()],
                dep_type: DepType::Exist,
                manipulation: Manipulation::Multiple,
            },
        );

        root.push_back("unknown_4", vec![0_u8.into(); 7]);

        root.push_back(
            "unknown_structure_ww_campaign_2",
            vec![vec![0_u8.into(); 32].into(); 1],
        );

        root.patches.insert(
            "unknown_structure_ww_campaign_2".to_string(),
            NumericPatch {
                source: vec!["unknown_3".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("unknown_5", -1_i32);

        root.into()
    }
}
