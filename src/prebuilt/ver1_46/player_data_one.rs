use crate::{parse::{wrap::Wrappable, Token}, utils::LinkedHashMap};

pub struct PlayerDataOne {}
impl PlayerDataOne {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(5);
        root.push_back("active", (0 as u32).wrap());
        root.push_back("human", (0 as u32).wrap());
        root.push_back("civilization", (40 as u32).wrap());
        root.push_back("architecture_set", (40 as u32).wrap());
        root.push_back("cty_mode", (4 as u32).wrap());
        root.wrap()
    }
}
