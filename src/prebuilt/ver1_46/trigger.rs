use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

use super::{Condition, Effect};

pub struct Trigger {}

impl Trigger {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(19);
        root.push_back("enabled", 1 as u32);
        root.push_back("looping", 0 as i8);
        root.push_back("description_string_table_id", 0 as i32);
        root.push_back("display_as_objective", 0 as u8);
        root.push_back("objective_description_order", 0 as u32);
        root.push_back("make_header", 0 as u8);
        root.push_back("short_description_string_table_id", 0 as i32);
        root.push_back("display_on_screen", 0 as u8);
        root.push_back("unknown", vec![(0 as u8).into(); 5]);
        root.push_back("mute_objectives", 0 as u8);
        root.push_back("trigger_description", DynString::new(0 as u32, ""));
        root.push_back("trigger_name", DynString::new(0 as u32, ""));
        root.push_back("short_description", DynString::new(0 as u32, ""));
        root.push_back("number_of_effects", 0 as i32);
        root.push_back("effect_data", vec![Effect::template(); 1]);
        root.patchs.insert(
            "effect_data".to_string(),
            NumericPatch {
                source: vec!["number_of_effects".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );
        root.push_back("effect_display_order_array", vec![(0 as i32).into()]);
        root.patchs.insert(
            "effect_display_order_array".to_string(),
            NumericPatch {
                source: vec!["number_of_effects".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("number_of_conditions", 0 as i32);
        root.push_back("condition_data", vec![Condition::template(); 1]);
        root.patchs.insert(
            "condition_data".to_string(),
            NumericPatch {
                source: vec!["number_of_conditions".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("condition_display_order_array", vec![(0 as i32).into()]);
        root.patchs.insert(
            "condition_display_order_array".to_string(),
            NumericPatch {
                source: vec!["number_of_conditions".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.into()
    }
}