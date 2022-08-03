use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap, C4},
};

pub struct FileHeader {}

impl FileHeader {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(12);
        root.push_back("version", C4::new("1.46"));
        root.push_back("header_length", 0 as u32);
        root.push_back("savable", 6 as i32);
        root.push_back("timestamp_of_last_save", 1610675127 as u32);
        root.push_back(
            "scenario_instructions",
            DynString::with_capacity(0 as u32, ""),
        );
        root.push_back("player_count", 2 as u32);
        root.push_back("unknown_value", 1000 as u32);
        root.push_back("unknown_value_2", 1 as u32);
        root.push_back("amount_of_unknown_numbers", 6 as u32);

        let unknown_numbers: Vec<Token> = vec![
            (2 as u32).into(),
            (3 as u32).into(),
            (4 as u32).into(),
            (5 as u32).into(),
            (6 as u32).into(),
            (7 as u32).into(),
        ];

        root.push_back("unknown_numbers", unknown_numbers);
        root.push_back(
            "creator_name",
            DynString::<u32>::with_capacity(12, "Unknown"),
        );
        root.push_back("trigger_count", 0 as u32);

        root.into()
    }
}
