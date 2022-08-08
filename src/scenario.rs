use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};

use crate::{
    io::Source,
    parse::{serde::Serialize, Token, TokenBuilder},
    prebuilt::{ver1_46, ver1_47},
    proxy::TriggersProxy,
};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

pub struct Scenario {
    pub versio: Token,
    version: String,
}

impl Scenario {
    /// Read scenario data from the given file.
    /// Please check the github homepage for the version support status.
    /// Generally, ver.1.46 and above are well supported.
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::Scenario;
    /// let scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario");
    /// ```
    pub fn from_file(filename: &str) -> Self {
        let mut file = File::open(&filename).expect("File not found");
        let metadata = fs::metadata(&filename).expect("Unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer).expect("buffer overflow");

        Self::from_le_vec(buffer)
    }

    /// Read scenario data from the given little endian buffer.
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::Scenario;
    /// //Serialize a scenario to a little endian vector of uint8
    /// let source_scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario");
    /// let buffer = source_scenario.to_le_vec();
    /// 
    /// //Deserialize a scenario from a little endian vector of uint8
    /// let scenario = Scenario::from_le_vec(buffer);
    /// ```
    pub fn from_le_vec(buffer: Vec<u8>) -> Self {
        let version = Self::get_scenario_version(&buffer);
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
                Scenario {
                    versio: TokenBuilder::create_from_template(
                        &ver1_46::Versio::template(),
                        &mut source,
                    ),
                    version,
                }
            }
            "1.47" => {
                let header = TokenBuilder::create_from_template(
                    &ver1_46::FileHeader::template(),
                    &mut source,
                );

                let mut uncompressed = header.to_le_vec();
                let content = decompress_to_vec(&source.get_rest_vec()).unwrap();
                uncompressed.extend(content);

                let mut source = Source::new(uncompressed);
                Scenario {
                    versio: TokenBuilder::create_from_template(
                        &ver1_47::Versio::template(),
                        &mut source,
                    ),
                    version,
                }
            }
            _ => panic!("Unsupported version!"),
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

    pub fn triggers_proxy(&mut self) -> TriggersProxy {
        TriggersProxy::new(&self.version, &mut self.versio)
    }

    fn get_scenario_version(buffer: &[u8]) -> String {
        std::str::from_utf8(&buffer[0..4]).unwrap().to_string()
    }
}
