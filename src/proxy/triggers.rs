use crate::{
    parse::{Censor, Token},
    ver1_46,
};

pub struct TriggersProxy<'a> {
    version: String,
    versio: &'a mut Token,
}

impl<'a> TriggersProxy<'a> {
    pub fn new(ver: &str, token: &'a mut Token) -> TriggersProxy<'a> {
        match ver {
            "1.46" => {
                let template = ver1_46::Versio::template();
                if !Censor::is_template(token, &template, 2) {
                    panic!("The given versio doesn't have a correct format");
                }

                TriggersProxy {
                    version: ver.to_string(),
                    versio: token,
                }
            }
            _ => panic!("Unsupport version!"),
        }
    }

    pub fn push(&mut self, value: Token) {
        match self.version.as_str() {
            "1.46" => {
                let template = ver1_46::Trigger::template();
                if !Censor::is_template(&value, &template, 2) {
                    panic!("The given trigger doesn't have a correct format");
                }

                let trigger_data = self
                    .versio
                    .get_by_path_mut("/triggers/trigger_data")
                    .try_mut_vec();
                trigger_data.push(value);

                let trigger_display_order_array = self
                    .versio
                    .get_by_path_mut("/triggers/trigger_display_order_array")
                    .try_mut_vec();
                trigger_display_order_array.push((trigger_display_order_array.len() as u32).into());

                let trigger_count = self
                    .versio
                    .get_by_path("/triggers/number_of_triggers")
                    .try_u32();

                self.set_trigger_count(*trigger_count + 1);
            }
            _ => panic!("Unsupport version!"),
        };
    }

    pub fn insert(&mut self, display_index: usize, execute_index: usize, value: Token) {
        match self.version.as_str() {
            "1.46" => {
                let trigger_display_order_array = self
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

                let trigger_data = self
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
                            map.update("trigger_id", target_id + 1);
                        }
                    }
                }

                trigger_data.insert(execute_index, value);

                let trigger_count = self
                    .versio
                    .get_by_path("/triggers/number_of_triggers")
                    .try_u32();

                self.set_trigger_count(*trigger_count + 1);
            }
            _ => panic!("Unsupport version!"),
        }
    }

    fn set_trigger_count(&mut self, count: u32) {
        match self.version.as_str() {
            "1.46" => {
                let trigger_count = self
                    .versio
                    .get_by_path_mut("/file_header/trigger_count")
                    .try_mut_u32();
                *trigger_count = count;

                let number_of_triggers = self
                    .versio
                    .get_by_path_mut("/options/number_of_triggers")
                    .try_mut_u32();
                *number_of_triggers = count;

                let number_of_triggers = self
                    .versio
                    .get_by_path_mut("/triggers/number_of_triggers")
                    .try_mut_u32();
                *number_of_triggers = count;
            }
            _ => panic!("Unsupport version!"),
        }
    }
}
