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
        let template = match ver {
            "1.46" => ver1_46::Versio::template(),
            _ => panic!("Unsupport version!"),
        };

        if !Censor::is_template(token, &template, 2) {
            panic!("The given versio doesn't have a correct format");
        }

        TriggersProxy {
            version: ver.to_string(),
            versio: token,
        }
    }

    pub fn push(&mut self, value: Token) {
        let template = match self.version.as_str() {
            "1.46" => ver1_46::Trigger::template(),
            _ => panic!("Unsupport version!"),
        };

        if !Censor::is_template(&value, &template, 2) {
            panic!("The given trigger doesn't have a correct format");
        }

        let trigger_count = self
            .versio
            .get_by_path_mut("/file_header/trigger_count")
            .try_mut_u32();
        *trigger_count += 1;

        let number_of_triggers = self
            .versio
            .get_by_path_mut("/options/number_of_triggers")
            .try_mut_u32();
        *number_of_triggers += 1;

        let number_of_triggers = self
            .versio
            .get_by_path_mut("/triggers/number_of_triggers")
            .try_mut_i32();
        *number_of_triggers += 1;

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
    }

    
}
