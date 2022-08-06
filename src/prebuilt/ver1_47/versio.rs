use crate::prebuilt::ver1_46;
use crate::prebuilt::ver1_47;
use crate::{parse::Token, utils::LinkedHashMap};

pub struct Versio {}

#[allow(dead_code)]
impl Versio {
    pub fn template() -> Token {
        let mut root = LinkedHashMap::new();
        root.push_back("file_header", ver1_46::FileHeader::template());
        root.push_back("data_header", ver1_46::DataHeader::template());
        root.push_back("message", ver1_46::Message::template());
        root.push_back("cinematics", ver1_46::Cinematics::template());
        root.push_back("background_image", ver1_46::Background::template());
        root.push_back("player_data_two", ver1_46::PlayerDataTwo::template());
        root.push_back("global_victory", ver1_46::GlobalVictory::template());
        root.push_back("diplomacy", ver1_46::Diplomacy::template());
        root.push_back("options", ver1_46::Options::template());
        root.push_back("map", ver1_47::Map::template());
        root.push_back("units", ver1_46::Units::template());
        root.push_back("triggers", ver1_46::Triggers::template());
        root.push_back("files", ver1_46::Files::template());
        root.into()
    }
}
