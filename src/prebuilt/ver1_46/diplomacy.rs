use crate::{parse::Token, utils::PatchedMap};

use super::PlayerDiplomacy;

pub struct Diplomacy;

impl Diplomacy {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(8);
        root.push_back(
            "per_player_diplomacy",
            vec![PlayerDiplomacy::template(); 16],
        );
        root.push_back("individual_victories", vec![0_u8.into(); 11520]);
        root.push_back("separator", 4294967197_u32);
        root.push_back("per_player_allied_victory", vec![0_u32.into(); 16]);
        root.push_back("lock_teams", 0_u8);
        root.push_back("allow_players_choose_teams", 1_u8);
        root.push_back("random_start_points", 0_u8);
        root.push_back("max_number_of_teams", 4_u8);

        root.into()
    }
}
