use crate::{
    parse::{Censor, Token},
    prebuilt::{ver1_46, EffectConfig, EFFECT_SCHEME},
    AttrTweak, Scenario,
};

pub struct EffectTweak;

impl EffectTweak {
    pub fn translate(
        scenario: &Scenario,
        effect: &Token,
    ) -> Result<(String, String), &'static str> {
        let version = scenario.version();
        match version {
            "1.46" | "1.47" => {
                Self::is_effect(effect, version)?;

                let type_id = *effect.get_by_path("/effect_type").try_i32();
                if type_id >= 0 && type_id < EFFECT_SCHEME.len() as i32 {
                    let scheme = &EFFECT_SCHEME[type_id as usize];
                    let name = scheme.name.to_string();
                    let attrs: Vec<String> = scheme
                        .attrs
                        .iter()
                        .map(|&path| match path {
                            "message" | "sound_name" => {
                                format!(
                                    "{}: {}",
                                    path,
                                    effect.get_by_path(path).try_str32().content()
                                )
                            }
                            "class" | "quantity" => {
                                format!("{}: {}", path, effect.get_by_path(path).try_i16())
                            }
                            "selected_object_ids" => {
                                format!("{}: {:?}", path, effect.get_by_path(path).try_vec())
                            }
                            "object_attributes" => {
                                let attr_id = effect.get_by_path(path).try_i32();
                                let content = AttrTweak::translate(attr_id, version).unwrap();
                                format!("{}: {}", path, content)
                            }
                            _ => {
                                format!("{}: {}", path, effect.get_by_path(path).try_i32())
                            }
                        })
                        .collect();

                    Ok((name, attrs.join(" ")))
                } else {
                    Ok(("Unknown Effect!".to_string(), "".to_string()))
                }
            }
            _ => Err("Incompatible version!"),
        }
    }

    pub fn scheme(version: &str, effect: &Token) -> Result<&'static EffectConfig, &'static str> {
        match version {
            "1.46" | "1.47" => {
                Self::is_effect(effect, version)?;

                let type_id = *effect.get_by_path("/effect_type").try_i32();
                if type_id >= 0 && type_id < EFFECT_SCHEME.len() as i32 {
                    let scheme = &EFFECT_SCHEME[type_id as usize];

                    Ok(scheme)
                } else {
                    Err("Unknown Effect!")
                }
            }
            _ => Err("Incompatible version!"),
        }
    }

    pub fn is_effect(effect: &Token, version: &str) -> Result<(), &'static str> {
        match version {
            "1.46" | "1.47" => {
                let template = ver1_46::Effect::template();
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
