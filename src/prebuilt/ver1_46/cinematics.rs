use crate::{
    parse::Token,
    utils::{DynString, PatchedMap},
};

pub struct Cinematics {}

impl Cinematics {
    pub fn template() -> Token {
        let mut root = PatchedMap::with_capacity(3);
        root.push_back("ascii_pregame", DynString::with_capacity(0_u16, ""));
        root.push_back("ascii_victory", DynString::with_capacity(0_u16, ""));
        root.push_back("ascii_loss", DynString::with_capacity(0_u16, ""));

        root.into()
    }
}
