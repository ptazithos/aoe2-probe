use std::sync::Arc;

use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

use super::AI2;

pub struct Files;

impl Files {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(7);
        root.push_back("script_file_path", DynString::with_capacity(0_u16, ""));
        root.push_back("script_file_content", DynString::with_capacity(0_u32, ""));
        root.push_back("ai_files_present", 0_u32);
        root.push_back("unknown", vec![0_u8.into(); 4]);
        root.push_back("number_of_ai_files", vec![0_u32.into()]);

        root.patches.insert(
            "number_of_ai_files".to_string(),
            Arc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("ai_files_present") {
                    let count = *map["ai_files_present"].try_u32();
                    let unit = template.try_vec()[0].clone();
                    let vec = template.try_mut_vec();
                    vec.clear();

                    for _ in 0..count {
                        vec.push(unit.clone());
                    }
                }
            }),
        );

        root.push_back("ai_files", vec![AI2::template(); 1]);

        root.patches.insert(
            "ai_files".to_string(),
            Arc::new(|map: &mut PatchedMap, template: &mut Token| {
                if map.contains("number_of_ai_files") {
                    let count = *map["number_of_ai_files"].try_vec()[0].try_u32();
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
