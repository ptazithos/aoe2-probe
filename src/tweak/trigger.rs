use crate::{
    parse::{Censor, Token},
    prebuilt::ver1_46,
};

pub struct TriggerTweak {}

impl TriggerTweak {
    pub fn is_trigger(trigger: &Token, version: &str) -> Result<(), String> {
        match version {
            "1.46" | "1.47" => {
                let template = ver1_46::Trigger::template();
                let res = Censor::is_template(trigger, &template, 2);

                if res {
                    Ok(())
                } else {
                    Err("Not a trigger!".to_string())
                }
            }
            _ => Err("Incompatible version!".to_string()),
        }
    }
}
