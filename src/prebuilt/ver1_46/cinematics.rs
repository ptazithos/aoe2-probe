use crate::{
    parse::{wrap::Wrappable, Token},
    utils::{DynString, LinkedHashMap},
};

pub struct Cinematics {}

impl Cinematics {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::with_capacity(3);
        root.push_back("ascii_pregame", DynString::new(0 as u16, "").wrap());
        root.push_back("ascii_victory", DynString::new(0 as u16, "").wrap());
        root.push_back("ascii_loss", DynString::new(0 as u16, "").wrap());

        root.wrap()
    }
}
