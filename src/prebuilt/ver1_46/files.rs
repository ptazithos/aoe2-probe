use crate::{
    parse::Token,
    utils::{map::*, DynString},
};

use super::AI2;

pub struct Files {}

impl Files {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(7);
        root.push_back("script_file_path", DynString::with_capacity(0 as u16, ""));
        root.push_back(
            "script_file_content",
            DynString::with_capacity(0 as u32, ""),
        );
        root.push_back("ai_files_present", 0 as u32);
        root.push_back("unknown", vec![(0 as u8).into(); 4]);
        root.push_back("number_of_ai_files", vec![(0 as u32).into()]);
        root.patchs.insert(
            "number_of_ai_files".to_string(),
            NumericPatch {
                source: vec!["ai_files_present".to_string()],
                dep_type: DepType::Exist,
                manipulation: Manipulation::Equal,
            },
        );

        root.push_back("ai_files", vec![AI2::template(); 1]);
        root.patchs.insert(
            "ai_files".to_string(),
            NumericPatch {
                source: vec!["number_of_ai_files".to_string()],
                dep_type: DepType::Calculate,
                manipulation: Manipulation::Equal,
            },
        );

        root.into()
    }
}
