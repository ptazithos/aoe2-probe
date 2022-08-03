use crate::{parse::Token, utils::LinkedHashMap};

pub struct PlayerDiplomacy {}

impl PlayerDiplomacy {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(1);
        root.push_back("stance_with_each_player", vec![(3_u32).into(); 16]);

        root.into()
    }
}
