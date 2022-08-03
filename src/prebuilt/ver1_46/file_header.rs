use crate::{
    parse::Token,
    utils::{DynString, LinkedHashMap, C4},
};

pub struct FileHeader {}

impl FileHeader {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(12);
        root.push_back("version", C4::new("1.46"));
        root.push_back("header_length", 0_u32);
        root.push_back("savable", 6_i32);
        root.push_back("timestamp_of_last_save", 1610675127_u32);
        root.push_back("scenario_instructions", DynString::with_capacity(0_u32, ""));
        root.push_back("player_count", 2_u32);
        root.push_back("unknown_value", 1000_u32);
        root.push_back("unknown_value_2", 1_u32);
        root.push_back("amount_of_unknown_numbers", 6_u32);

        let unknown_numbers: Vec<Token> = vec![
            2_u32.into(),
            3_u32.into(),
            4_u32.into(),
            5_u32.into(),
            6_u32.into(),
            7_u32.into(),
        ];

        root.push_back("unknown_numbers", unknown_numbers);
        root.push_back(
            "creator_name",
            DynString::<u32>::with_capacity(12, "Unknown"),
        );
        root.push_back("trigger_count", 0_u32);

        root.into()
    }
}
