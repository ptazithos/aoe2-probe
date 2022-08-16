use crate::prebuilt::ATTR_MAP;

pub struct AttrTweak;

impl AttrTweak {
    pub fn translate(object_attr: &i32, version: &str) -> Result<&'static str, &'static str> {
        match version {
            "1.46" | "1.47" => {
                if ATTR_MAP.contains_key(object_attr) {
                    return Ok(&ATTR_MAP[object_attr]);
                } else {
                    return Ok("Unknown");
                }
            }
            _ => Err("Incompatible version!"),
        }
    }
}
