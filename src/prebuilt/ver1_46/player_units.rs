use crate::{parse::Token, utils::map::*};

use super::Unit;

pub struct PlayerUnits {}

impl PlayerUnits {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(2);
        root.push_back("unit_count", 0 as u32);
        root.push_back("units", vec![Unit::template(); 1]);
        root.patchs.insert(
            "units".to_string(),
            NumericPatch {
                source: vec!["unit_count".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.into()
    }
}
