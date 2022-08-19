use std::sync::Arc;

use crate::{parse::Token, utils::map::*};

use super::{Trigger, Variable};

pub struct Triggers;

impl Triggers {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(9);
        root.push_back("trigger_version", 2.6_f64);
        root.push_back("trigger_instruction_start", 0_i8);
        root.push_back("number_of_triggers", 0_u32);
        root.push_back("trigger_data", vec![Trigger::template()]);

        root.patches.insert(
            "trigger_data".to_string(),
            Arc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("number_of_triggers") {
                    let count = *map["number_of_triggers"].try_u32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.push_back("trigger_display_order_array", vec![0_u32.into()]);
        root.patches.insert(
            "trigger_display_order_array".to_string(),
            Arc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("number_of_triggers") {
                    let count = *map["number_of_triggers"].try_u32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.push_back("unknown_bytes", vec![0_u8.into(); 1028]);
        root.push_back("number_of_variables", 0_u32);
        root.push_back("variable_data", vec![Variable::template()]);

        root.patches.insert(
            "variable_data".to_string(),
            Arc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("number_of_variables") {
                    let count = *map["number_of_variables"].try_u32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.push_back("useless_trigger_data", vec![0_u8.into(); 9]);

        root.into()
    }
}
