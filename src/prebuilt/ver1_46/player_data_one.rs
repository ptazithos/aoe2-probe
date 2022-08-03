use crate::{parse::Token, utils::LinkedHashMap};

pub struct PlayerDataOne {}
impl PlayerDataOne {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(5);
        root.push_back("active", 0_u32);
        root.push_back("human", 0_u32);
        root.push_back("civilization", 40_u32);
        root.push_back("architecture_set", 40_u32);
        root.push_back("cty_mode", 4_u32);

        root.into()
    }
}
