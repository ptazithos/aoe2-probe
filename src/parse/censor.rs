use crate::{parse::Token, utils::PatchedMap};

pub struct Censor {}

impl Censor {
    pub fn is_template(root: &Token, template: &Token, depth: usize) -> bool {
        if depth == 0 {
            return true;
        }
        match root {
            Token::Union(map) => {
                let root_type = root.eq(template);
                if root_type {
                    let template_map = template.try_map();

                    let contains_key = template_map
                        .keys()
                        .map(|key| map.contains(key))
                        .fold(true, |acc, current| acc & current);
                    if contains_key {
                        let value_equal = template_map
                            .keys()
                            .map(|key| Self::is_template(&map[key], &template_map[key], depth - 1))
                            .fold(true, |acc, current| acc & current);
                        return value_equal;
                    }
                }
                false
            }
            _ => root.eq(template),
        }
    }

    pub fn is_map(token: &Token) -> bool {
        let map: Token = PatchedMap::new().into();
        map.eq(token)
    }
}
