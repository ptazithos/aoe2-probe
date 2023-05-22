use crate::prebuilt::ver1_49;
use crate::{parse::Token, utils::PatchedMap};

pub struct Versio;

#[allow(dead_code)]
impl Versio {
    pub fn template() -> Token {
        let mut root = PatchedMap::new();
        root.push_back("file_header", ver1_49::FileHeader::template());
        root.push_back("data_header", ver1_49::DataHeader::template());
        root.push_back("message", ver1_49::Message::template());
        root.push_back("cinematics", ver1_49::Cinematics::template());
        root.push_back("background_image", ver1_49::Background::template());
        root.push_back("player_data_two", ver1_49::PlayerDataTwo::template());
        root.push_back("global_victory", ver1_49::GlobalVictory::template());
        root.push_back("diplomacy", ver1_49::Diplomacy::template());
        root.push_back("options", ver1_49::Options::template());
        root.push_back("map", ver1_49::Map::template());
        root.push_back("units", ver1_49::Units::template());
        root.push_back("triggers", ver1_49::Triggers::template());
        root.push_back("files", ver1_49::Files::template());
        root.into()
    }
}
