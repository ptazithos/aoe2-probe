use crate::{parse::Token, utils::LinkedHashMap};

pub struct GlobalVictory {}

impl GlobalVictory {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(11);
        root.push_back("separator", 4294967197_u32);
        root.push_back("conquest_required", 0_u32);
        root.push_back("ruins", 0_u32);
        root.push_back("artifacts_required", 0_u32);
        root.push_back("discovery", 0_u32);
        root.push_back("explored_percent_of_map_required", 0_u32);
        root.push_back("gold_required", 0_u32);
        root.push_back("all_custom_conditions_required", 0_u32);
        root.push_back("mode", 0_u32);
        root.push_back("required_score_for_score_victory", 900_u32);
        root.push_back("time_for_timed_game_in_10ths_of_a_year", 9000_u32);

        root.into()
    }
}
