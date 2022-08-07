use crate::{
    parse::Token,
    utils::{DynString, PatchedMap, C256},
};

use super::PlayerDataOne;

pub struct DataHeader {}

impl DataHeader {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(8);
        root.push_back("next_unit_id_to_place", 0_u32);
        root.push_back("version", 1.42_f32);
        root.push_back("tribe_names", vec![C256::new("").into(); 16]);
        root.push_back(
            "string_table_player_names",
            vec![(4294967294_u32).into(); 16],
        );
        root.push_back("player_data_1", vec![PlayerDataOne::template(); 16]);
        root.push_back("per_player_lock_civilization", vec![0_u32.into(); 16]);
        root.push_back("unknown", vec![0_u8.into(); 9]);
        root.push_back("filename", DynString::with_capacity(7_u16, "Unknown"));

        root.into()
    }
}
