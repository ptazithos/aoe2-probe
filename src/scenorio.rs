use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};

use crate::{
    io::Source,
    parse::{serde::Serialize, Token, TokenBuilder},
    prebuilt::ver1_46,
};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

pub struct Scenorio {
    pub versio: Token,
}

impl Scenorio {
    pub fn from_file(filename: &str) -> Self {
        let mut file = File::open(&filename).expect("File not found");
        let metadata = fs::metadata(&filename).expect("Unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer).expect("buffer overflow");

        Self::from_le_vec(buffer)
    }

    pub fn from_le_vec(buffer: Vec<u8>) -> Self {
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

    pub fn to_le_vec(self) -> Vec<u8> {
        self.versio.to_le_vec()
    }

    pub fn to_le_export_vec(self) -> Vec<u8> {
        let file_header = &self.versio.try_map()["file_header"];
        let mut content = file_header.to_le_vec();
        let header_size = content.len();

        let uncompressed = &self.versio.to_le_vec()[header_size..];
        let mut compressed = compress_to_vec(uncompressed, 6);

        content.append(&mut compressed);

        content
    }

    pub fn to_file(self, file_path: &str) {
        let buffer = self.to_le_export_vec();

        let path = std::path::Path::new(file_path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .unwrap();

        file.write_all(&buffer).unwrap();
    }

    fn get_scenorio_version(buffer: &[u8]) -> String {
        std::str::from_utf8(&buffer[0..4]).unwrap().to_string()
    }
}
