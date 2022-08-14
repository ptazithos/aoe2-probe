use crate::{
    parse::{Censor, Token},
    prebuilt::CONDITION_SCHEME,
    ver1_46, Scenario,
};

pub struct ConditionTweak;

impl ConditionTweak {
    pub fn translate(
        scenario: &Scenario,
        condition: &Token,
    ) -> Result<(String, String), &'static str> {
        match scenario.version() {
            "1.46" | "1.47" => {
                Self::is_condition(condition, scenario.version())?;

                let type_id = *condition.get_by_path("/condition_type").try_i32();
                if CONDITION_SCHEME.contains_key(&type_id) {
                    let scheme = &CONDITION_SCHEME[&type_id];
                    let name = scheme.name.to_string();
                    let attrs: Vec<String> = scheme
                        .attrs
                        .iter()
                        .map(|&path| match path {
                            "xs_function" => {
                                format!(
                                    "{}: {:?}",
                                    path,
                                    condition.get_by_path(path).try_str32().content()
                                )
                            }
                            _ => {
                                format!("{}: {:?}", path, condition.get_by_path(path).try_i32())
                            }
                        })
                        .collect();

                    Ok((name, attrs.join(" ")))
                } else {
                    Err("Unknown Condition!")
                }
            }
            _ => Err("Incompatible version!"),
        }
    }

    pub fn is_condition(effect: &Token, version: &str) -> Result<(), &'static str> {
        match version {
            "1.46" | "1.47" => {
                let template = ver1_46::Condition::template();
                let res = Censor::is_template(effect, &template, 2);

                if res {
                    Ok(())
                } else {
                    Err("Not a effect!")
                }
            }
            _ => Err("Incompatible version!"),
        }
    }
}
