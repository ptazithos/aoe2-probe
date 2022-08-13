use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;

pub struct ConditionConfig {
    pub name: &'static str,
    pub attrs: Vec<&'static str>,
}

impl ConditionConfig {
    fn new(name: &'static str, attrs: Vec<&'static str>) -> Self {
        ConditionConfig { name, attrs }
    }
}

lazy_static! {
    pub static ref CONDITION_SCHEME: LinkedHashMap<i32, ConditionConfig> = {
        let mut mapping = LinkedHashMap::new();

        mapping.insert(0, ConditionConfig::new("None", vec![]));
        mapping.insert(
            1,
            ConditionConfig::new(
                "BRING_OBJECT_TO_AREA",
                vec![
                    "unit_object",
                    "area_x1",
                    "area_y1",
                    "area_x2",
                    "area_y2",
                    "inverted",
                ],
            ),
        );
        mapping.insert(
            2,
            ConditionConfig::new(
                "BRING_OBJECT_TO_OBJECT",
                vec!["unit_object", "next_object", "inverted"],
            ),
        );
        mapping.insert(
            3,
            ConditionConfig::new(
                "OWN_OBJECTS",
                vec![
                    "quantity",
                    "object_list",
                    "source_player",
                    "object_group",
                    "object_type",
                    "include_changeable_weapon_objects",
                ],
            ),
        );
        mapping.insert(
            4,
            ConditionConfig::new(
                "OWN_FEWER_OBJECTS",
                vec![
                    "quantity",
                    "object_list",
                    "source_player",
                    "area_x1",
                    "area_y1",
                    "area_x2",
                    "area_y2",
                    "object_group",
                    "object_type",
                    "include_changeable_weapon_objects",
                ],
            ),
        );
        mapping.insert(
            5,
            ConditionConfig::new(
                "OBJECTS_IN_AREA",
                vec![
                    "quantity",
                    "object_list",
                    "source_player",
                    "area_x1",
                    "area_y1",
                    "area_x2",
                    "area_y2",
                    "object_group",
                    "object_type",
                    "object_state",
                    "inverted",
                    "include_changeable_weapon_objects",
                ],
            ),
        );
        mapping.insert(
            6,
            ConditionConfig::new("DESTROY_OBJECT", vec!["unit_object", "inverted"]),
        );
        mapping.insert(
            7,
            ConditionConfig::new(
                "CAPTURE_OBJECT",
                vec!["unit_object", "source_player", "inverted"],
            ),
        );
        mapping.insert(
            8,
            ConditionConfig::new(
                "ACCUMULATE_ATTRIBUTE",
                vec!["quantity", "attribute", "source_player", "inverted"],
            ),
        );
        mapping.insert(
            9,
            ConditionConfig::new(
                "RESEARCH_TECHNOLOGY",
                vec!["source_player", "technology", "inverted"],
            ),
        );

        mapping.insert(10, ConditionConfig::new("TIMER", vec!["timer", "inverted"]));
        mapping.insert(
            11,
            ConditionConfig::new("OBJECT_SELECTED", vec!["unit_object", "inverted"]),
        );
        mapping.insert(
            12,
            ConditionConfig::new("AI_SIGNAL", vec!["ai_signal", "inverted"]),
        );
        mapping.insert(
            13,
            ConditionConfig::new("PLAYER_DEFEATED", vec!["source_player", "inverted"]),
        );
        mapping.insert(
            14,
            ConditionConfig::new(
                "OBJECT_HAS_TARGET",
                vec![
                    "unit_object",
                    "next_object",
                    "object_list",
                    "object_group",
                    "object_type",
                    "inverted",
                ],
            ),
        );
        mapping.insert(
            15,
            ConditionConfig::new("OBJECT_VISIBLE", vec!["unit_object"]),
        );
        mapping.insert(
            16,
            ConditionConfig::new("OBJECT_NOT_VISIBLE", vec!["unit_object"]),
        );
        mapping.insert(
            17,
            ConditionConfig::new(
                "RESEARCHING_TECH",
                vec!["source_player", "technology", "inverted"],
            ),
        );
        mapping.insert(
            18,
            ConditionConfig::new(
                "UNITS_GARRISONED",
                vec!["quantity", "unit_object", "inverted"],
            ),
        );
        mapping.insert(
            19,
            ConditionConfig::new(
                "DIFFICULTY_LEVEL",
                vec!["condition_type", "quantity", "inverted"],
            ),
        );

        mapping.insert(20, ConditionConfig::new("CHANCE", vec!["quantity"]));
        mapping.insert(
            21,
            ConditionConfig::new(
                "TECHNOLOGY_STATE",
                vec!["quantity", "source_player", "technology", "inverted"],
            ),
        );
        mapping.insert(
            22,
            ConditionConfig::new(
                "VARIABLE_VALUE",
                vec!["quantity", "inverted", "variable", "comparison"],
            ),
        );
        mapping.insert(
            23,
            ConditionConfig::new(
                "OBJECT_HP",
                vec!["quantity", "unit_object", "inverted", "comparison"],
            ),
        );
        mapping.insert(
            24,
            ConditionConfig::new(
                "DIPLOMACY_STATE",
                vec!["quantity", "source_player", "inverted", "target_player"],
            ),
        );
        mapping.insert(25, ConditionConfig::new("SCRIPT_CALL", vec!["xs_function"]));
        mapping.insert(
            26,
            ConditionConfig::new(
                "OBJECT_SELECTED_MULTIPLAYER",
                vec!["unit_object", "source_player", "inverted"],
            ),
        );
        mapping.insert(
            27,
            ConditionConfig::new(
                "OBJECT_VISIBLE_MULTIPLAYER",
                vec!["unit_object", "source_player", "inverted"],
            ),
        );
        mapping.insert(
            28,
            ConditionConfig::new(
                "OBJECT_HAS_ACTION",
                vec!["unit_object", "next_object", "inverted", "unit_ai_action"],
            ),
        );
        mapping.insert(29, ConditionConfig::new("OR", vec![]));

        mapping.insert(
            30,
            ConditionConfig::new("AI_SIGNAL_MULTIPLAYER", vec!["ai_signal", "inverted"]),
        );
        mapping.insert(
            54,
            ConditionConfig::new("BUILDING_IS_TRADING", vec!["unit_object", "inverted"]),
        );
        mapping.insert(
            55,
            ConditionConfig::new("DISPLAY_TIMER_TRIGGERED", vec!["timer_id", "inverted"]),
        );
        mapping.insert(
            56,
            ConditionConfig::new(
                "VICTORY_TIMER",
                vec![
                    "quantity",
                    "source_player",
                    "inverted",
                    "comparison",
                    "victory_timer_type",
                ],
            ),
        );
        mapping.insert(57, ConditionConfig::new("AND", vec![]));

        mapping
    };
}
