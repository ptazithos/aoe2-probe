use std::collections::HashMap;

use crate::{parse::Token, Scenario, TriggerTweak};

pub struct TriggersTweak {}

impl TriggersTweak {
    /// Push the given trigger to the end of the trigger data
    /// This method will check the correctness and version compatible.
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::{Scenario, TriggersTweak};
    /// use aoe2_probe::ver1_46::Trigger;
    ///
    /// let mut scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    /// let trigger = Trigger::default();
    ///
    /// TriggersTweak::push(&mut scenario, trigger).unwrap();
    /// ```
    pub fn push(scenario: &mut Scenario, trigger: Token) -> Result<(), String> {
        let version = scenario.version().to_string();
        match version.as_str() {
            "1.46" | "1.47" => {
                TriggerTweak::is_trigger(&trigger, &version)?;

                let trigger_data = scenario
                    .versio
                    .get_by_path_mut("/triggers/trigger_data")
                    .try_mut_vec();
                trigger_data.push(trigger);

                let trigger_display_order_array = scenario
                    .versio
                    .get_by_path_mut("/triggers/trigger_display_order_array")
                    .try_mut_vec();
                trigger_display_order_array.push((trigger_display_order_array.len() as u32).into());

                let trigger_count = scenario
                    .versio
                    .get_by_path("/triggers/number_of_triggers")
                    .try_u32();
                Self::set_trigger_count(scenario, &version, *trigger_count + 1).unwrap();

                return Ok(());
            }
            _ => Err("Incompatible version!".to_string()),
        }
    }

    pub fn insert(
        scenario: &mut Scenario,
        trigger: Token,
        display_index: usize,
        execute_index: usize,
    ) -> Result<(), String> {
        let version = scenario.version().to_string();
        match version.as_str() {
            "1.46" | "1.47" => {
                TriggerTweak::is_trigger(&trigger, &version)?;

                let trigger_display_order_array = scenario
                    .versio
                    .get_by_path_mut("/triggers/trigger_display_order_array")
                    .try_mut_vec();

                for index in trigger_display_order_array.iter_mut() {
                    let index = index.try_mut_u32();
                    if *index >= execute_index as u32 {
                        *index += 1;
                    }
                }
                trigger_display_order_array.insert(display_index, (execute_index as u32).into());

                let trigger_data = scenario
                    .versio
                    .get_by_path_mut("/triggers/trigger_data")
                    .try_mut_vec();

                for trigger in trigger_data.iter_mut() {
                    let map = trigger.try_mut_map();
                    let effects = map["effect_data"].try_mut_vec();
                    for effect in effects.iter_mut() {
                        let map = effect.try_mut_map();
                        let target_id = *map["trigger_id"].try_i32();
                        if target_id >= execute_index.try_into().unwrap() {
                            map.update("trigger_id", target_id + 1).unwrap();
                        }
                    }
                }

                trigger_data.insert(execute_index, trigger);

                let trigger_count = scenario
                    .versio
                    .get_by_path("/triggers/number_of_triggers")
                    .try_u32();
                Self::set_trigger_count(scenario, &version, *trigger_count + 1)?;

                return Ok(());
            }
            _ => Err("Incompatible version!".to_string()),
        }
    }

    /// Sort the trigger data by the display order
    /// This method will check the correctness and version compatible.
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::{Scenario, TriggersTweak};
    /// use aoe2_probe::ver1_46::Trigger;
    ///
    /// let mut scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    ///
    /// TriggersTweak::sort_by_display_order(&mut scenario).unwrap();
    /// ```
    pub fn sort_by_display_order(scenario: &mut Scenario) -> Result<(), String> {
        let version = scenario.version().to_string();
        match version.as_str() {
            "1.46" | "1.47" => {
                let display_order: Vec<u32> = scenario
                    .versio
                    .get_by_path("/triggers/trigger_display_order_array")
                    .try_vec()
                    .iter()
                    .map(|token| *token.try_u32())
                    .collect();

                let trigger_data = scenario
                    .versio
                    .get_by_path("/triggers/trigger_data")
                    .try_vec();

                let mut sorted_triggers = vec![];
                let mut order: Vec<Token> = vec![];
                let mut map = HashMap::new();

                for index in 0..display_order.len() as u32 {
                    let position = display_order
                        .iter()
                        .position(|&value| value == index)
                        .unwrap();
                    map.insert(position, index);
                    sorted_triggers.push(trigger_data[position].clone());
                    order.push(index.into());
                }

                for token in sorted_triggers.iter_mut() {
                    let effects = token.get_by_path_mut("/effect_data").try_mut_vec();
                    for token in effects.iter_mut() {
                        let target_trigger_id = token.get_by_path_mut("trigger_id").try_mut_i32();
                        if *target_trigger_id >= 0 {
                            let new_id = map[&(*target_trigger_id as usize)] as i32;
                            *target_trigger_id = new_id;
                        }
                    }
                }

                let triggers = scenario.versio.get_by_path_mut("/triggers").try_mut_map();
                triggers.update("trigger_data", sorted_triggers).unwrap();
                triggers
                    .update("trigger_display_order_array", order)
                    .unwrap();

                return Ok(());
            }
            _ => Err("Incompatible version!".to_string()),
        }
    }

    fn set_trigger_count(scenario: &mut Scenario, version: &str, count: u32) -> Result<(), String> {
        match version {
            "1.46" | "1.47" => {
                let trigger_count = scenario
                    .versio
                    .get_by_path_mut("/file_header/trigger_count")
                    .try_mut_u32();
                *trigger_count = count;

                let number_of_triggers = scenario
                    .versio
                    .get_by_path_mut("/options/number_of_triggers")
                    .try_mut_u32();
                *number_of_triggers = count;

                let number_of_triggers = scenario
                    .versio
                    .get_by_path_mut("/triggers/number_of_triggers")
                    .try_mut_u32();
                *number_of_triggers = count;
                return Ok(());
            }
            _ => Err("Incompatible version!".to_string()),
        }
    }
}
