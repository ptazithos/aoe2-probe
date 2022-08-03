use crate::{parse::Token, utils::map::*};

use super::{Trigger, Variable};

pub struct Triggers {}

impl Triggers {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(9);
        root.push_back("trigger_version", 2.6_f64);
        root.push_back("trigger_instruction_start", 0_i8);
        root.push_back("number_of_triggers", 0_i32);
        root.push_back("trigger_data", vec![Trigger::template()]);
        root.patchs.insert(
            "trigger_data".to_string(),
            NumericPatch {
                source: vec!["number_of_triggers".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );
        root.push_back("trigger_display_order_array", vec![0_u32.into()]);
        root.patchs.insert(
            "trigger_display_order_array".to_string(),
            NumericPatch {
                source: vec!["number_of_triggers".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("unknown_bytes", vec![0_u8.into(); 1028]);
        root.push_back("number_of_variables", 0_u32);
        root.push_back("variable_data", vec![Variable::template()]);
        root.patchs.insert(
            "variable_data".to_string(),
            NumericPatch {
                source: vec!["number_of_variables".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("useless_trigger_data", vec![0_u8.into(); 9]);

        root.into()
    }
}
