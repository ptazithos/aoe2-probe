use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

use super::{Condition, Effect};

pub struct Trigger {}

impl Trigger {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(19);
        root.push_back("enabled", 1_u32);
        root.push_back("looping", 0_i8);
        root.push_back("description_string_table_id", 0_i32);
        root.push_back("display_as_objective", 0_u8);
        root.push_back("objective_description_order", 0_u32);
        root.push_back("make_header", 0_u8);
        root.push_back("short_description_string_table_id", 0_i32);
        root.push_back("display_on_screen", 0_u8);
        root.push_back("unknown", vec![0_u8.into(); 5]);
        root.push_back("mute_objectives", 0_u8);
        root.push_back("trigger_description", DynString::with_capacity(0_u32, ""));
        root.push_back("trigger_name", DynString::with_capacity(0_u32, ""));
        root.push_back("short_description", DynString::with_capacity(0_u32, ""));
        root.push_back("number_of_effects", 0_i32);
        root.push_back("effect_data", vec![Effect::template(); 1]);
        root.patches.insert(
            "effect_data".to_string(),
            NumericPatch {
                source: vec!["number_of_effects".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );
        root.push_back("effect_display_order_array", vec![0_i32.into()]);
        root.patches.insert(
            "effect_display_order_array".to_string(),
            NumericPatch {
                source: vec!["number_of_effects".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("number_of_conditions", 0_i32);
        root.push_back("condition_data", vec![Condition::template(); 1]);
        root.patches.insert(
            "condition_data".to_string(),
            NumericPatch {
                source: vec!["number_of_conditions".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("condition_display_order_array", vec![0_i32.into()]);
        root.patches.insert(
            "condition_display_order_array".to_string(),
            NumericPatch {
                source: vec!["number_of_conditions".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.into()
    }
    
    pub fn default() -> Token {
        let mut root = PatchedMap::with_capacity(19);
        root.push_back("enabled", 1_u32);
        root.push_back("looping", 0_i8);
        root.push_back("description_string_table_id", 0_i32);
        root.push_back("display_as_objective", 0_u8);
        root.push_back("objective_description_order", 0_u32);
        root.push_back("make_header", 0_u8);
        root.push_back("short_description_string_table_id", 0_i32);
        root.push_back("display_on_screen", 0_u8);
        root.push_back("unknown", vec![0_u8.into(); 5]);
        root.push_back("mute_objectives", 0_u8);
        root.push_back("trigger_description", DynString::with_capacity(0_u32, ""));
        root.push_back("trigger_name", DynString::with_capacity(0_u32, ""));
        root.push_back("short_description", DynString::with_capacity(0_u32, ""));
        root.push_back("number_of_effects", 0_i32);
        root.push_back("effect_data", vec![]);
        root.push_back("effect_display_order_array", vec![]);
        root.push_back("number_of_conditions", 0_i32);
        root.push_back("condition_data", vec![]);
        root.push_back("condition_display_order_array", vec![]);

        root.into()
    }
}
