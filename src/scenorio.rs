use miniz_oxide::inflate::decompress_to_vec;

use crate::{
    io::Source,
    parse::{serde::Serialize, Token, TokenBuilder},
    prebuilt::ver1_46,
};
use std::{
    fs::{self, File},
    io::Read,
};

pub struct Scenorio {
    pub versio: Token,
}

impl Scenorio {
    pub fn from_file(filename: &str) -> Self {
        let mut file = File::open(&filename).expect("File not found");
        let metadata = fs::metadata(&filename).expect("Unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("buffer overflow");

        Self::from_vec(buffer)
    }

    pub fn from_vec(buffer: Vec<u8>) -> Self {
        let version = Self::get_scenorio_version(&buffer);
        let mut source = Source::new(buffer);
        match version.as_str() {
            "1.46" => {
                let header = TokenBuilder::create_from_template(
                    &ver1_46::FileHeader::template(),
                    &mut source,
                );

                let mut uncompressed = header.to_le_vec();
                let content = decompress_to_vec(&source.get_rest_vec()).unwrap();
                uncompressed.extend(content);

                let mut source = Source::new(uncompressed);
                Scenorio {
                    versio: TokenBuilder::create_from_template(
                        &ver1_46::Versio::template(),
                        &mut source,
                    ),
                }
            }
            _ => panic!("Unsupport version!"),
        }
    }

    fn get_scenorio_version(buffer: &Vec<u8>) -> String {
        std::str::from_utf8(&buffer[0..4]).unwrap().to_string()
    }
}
