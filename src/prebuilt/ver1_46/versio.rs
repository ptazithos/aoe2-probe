use crate::{parse::Token, utils::PatchedMap};

use super::{
    Background, Cinematics, DataHeader, Diplomacy, FileHeader, Files, GlobalVictory, Map, Message,
    Options, PlayerDataTwo, Triggers, Units,
};

pub struct Versio;

#[allow(dead_code)]
impl Versio {
    pub fn template() -> Token {
        let mut root = PatchedMap::new();
        root.push_back("file_header", FileHeader::template());
        root.push_back("data_header", DataHeader::template());
        root.push_back("message", Message::template());
        root.push_back("cinematics", Cinematics::template());
        root.push_back("background_image", Background::template());
        root.push_back("player_data_two", PlayerDataTwo::template());
        root.push_back("global_victory", GlobalVictory::template());
        root.push_back("diplomacy", Diplomacy::template());
        root.push_back("options", Options::template());
        root.push_back("map", Map::template());
        root.push_back("units", Units::template());
        root.push_back("triggers", Triggers::template());
        root.push_back("files", Files::template());
        root.into()
    }
}
