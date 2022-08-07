use crate::prebuilt::ver1_47;
use crate::{parse::Token, utils::LinkedHashMap};

pub struct Versio {}

#[allow(dead_code)]
impl Versio {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::new();
        root.push_back("file_header", ver1_47::FileHeader::template());
        root.push_back("data_header", ver1_47::DataHeader::template());
        root.push_back("message", ver1_47::Message::template());
        root.push_back("cinematics", ver1_47::Cinematics::template());
        root.push_back("background_image", ver1_47::Background::template());
        root.push_back("player_data_two", ver1_47::PlayerDataTwo::template());
        root.push_back("global_victory", ver1_47::GlobalVictory::template());
        root.push_back("diplomacy", ver1_47::Diplomacy::template());
        root.push_back("options", ver1_47::Options::template());
        root.push_back("map", ver1_47::Map::template());
        root.push_back("units", ver1_47::Units::template());
        root.push_back("triggers", ver1_47::Triggers::template());
        root.push_back("files", ver1_47::Files::template());
        root.into()
    }
}
