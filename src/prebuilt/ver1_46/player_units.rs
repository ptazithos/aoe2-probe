use std::rc::Rc;

use crate::{parse::Token, utils::map::*};

use super::Unit;

pub struct PlayerUnits;

impl PlayerUnits {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(2);
        root.push_back("unit_count", 0_u32);
        root.push_back("units", vec![Unit::template(); 1]);

        root.patches.insert(
            "units".to_string(),
            Rc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("unit_count") {
                    let count = *map["unit_count"].try_u32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.into()
    }
}
