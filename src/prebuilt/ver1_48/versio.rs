use crate::prebuilt::ver1_48;
use crate::{parse::Token, utils::PatchedMap};

pub struct Versio;

#[allow(dead_code)]
impl Versio {
    pub fn template() -> Token {
        let mut root = PatchedMap::new();
        root.push_back("file_header", ver1_48::FileHeader::template());
        root.push_back("data_header", ver1_48::DataHeader::template());
        root.push_back("message", ver1_48::Message::template());
        root.push_back("cinematics", ver1_48::Cinematics::template());
        root.push_back("background_image", ver1_48::Background::template());
        root.push_back("player_data_two", ver1_48::PlayerDataTwo::template());
        root.push_back("global_victory", ver1_48::GlobalVictory::template());
        root.push_back("diplomacy", ver1_48::Diplomacy::template());
        root.push_back("options", ver1_48::Options::template());
        root.push_back("map", ver1_48::Map::template());
        root.push_back("units", ver1_48::Units::template());
        root.push_back("triggers", ver1_48::Triggers::template());
        root.push_back("files", ver1_48::Files::template());
        root.into()
    }
}
