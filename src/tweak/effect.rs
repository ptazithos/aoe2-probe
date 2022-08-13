use crate::{
    parse::{Censor, Token},
    ver1_46, Scenario,
};

pub struct EffectTweak {}

impl EffectTweak {
    pub fn translate(scenario: &Scenario, effect: &Token) -> Result<String, String> {
        match scenario.version() {
            "1.46" | "1.47" => {
                Self::is_effect(effect, scenario.version())?;
                let mut effect_statement = vec![];
                let map = effect.try_map();

                for (key, value) in map.iter() {
                    match key.as_str() {
                        "static_value_46" => {}
                        "message" | "sound_name" => {
                            effect_statement.push(format!(
                                "{}: {}",
                                key,
                                value.try_str32().content()
                            ));
                        }
                        "selected_object_ids" => {
                            effect_statement.push(format!("{}: {:?}", key, value.try_vec()));
                        }
                        _ => {
                            let value = *value.try_i32();
                            if value >= 0 {
                                effect_statement.push(format!("{}: {}", key, value));
                            }
                        }
                    }
                }

                Ok(effect_statement.join(" "))
            }
            _ => Err("Incompatible version!".to_string()),
        }
    }

    pub fn is_effect(effect: &Token, version: &str) -> Result<(), String> {
        match version {
            "1.46" | "1.47" => {
                let template = ver1_46::Effect::template();
                let res = Censor::is_template(effect, &template, 2);

                if res {
                    Ok(())
                } else {
                    Err("Not a effect!".to_string())
                }
            }
            _ => Err("Incompatible version!".to_string()),
        }
    }
}
