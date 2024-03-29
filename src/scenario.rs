use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    io::Source,
    parse::{Encode, Token, TokenBuilder},
    prebuilt::{ver1_46, ver1_47, ver1_48, ver1_49},
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
    pub fn from_file(filename: &str) -> Result<Self, String> {
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
    /// //Encode a scenario to a little endian vector of uint8
    /// let source_scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    /// let buffer = source_scenario.to_le_export_vec();
    ///
    /// //Decode a scenario from a little endian vector of uint8
    /// let scenario = Scenario::from_le_vec(buffer);
    /// ```
    pub fn from_le_vec(buffer: Vec<u8>) -> Result<Self, String> {
        let version = Self::get_scenario_version(&buffer);
        let mut source = Source::new(buffer);
        match version.as_str() {
            "1.46" => {
                let header = TokenBuilder::create_from_template(
                    &ver1_46::FileHeader::template(),
                    &mut source,
                )?;

                let mut uncompressed = header.to_le_vec();
                let content = decompress_to_vec(&source.get_rest_vec()).unwrap();
                uncompressed.extend(content);

                let mut source = Source::new(uncompressed);
                Ok(Scenario {
                    versio: TokenBuilder::create_from_template(
                        &ver1_46::Versio::template(),
                        &mut source,
                    )?,
                    version,
                })
            }
            "1.47" => {
                let header = TokenBuilder::create_from_template(
                    &ver1_47::FileHeader::template(),
                    &mut source,
                )?;

                let mut uncompressed = header.to_le_vec();
                let content = decompress_to_vec(&source.get_rest_vec()).unwrap();
                uncompressed.extend(content);

                let mut source = Source::new(uncompressed);
                Ok(Scenario {
                    versio: TokenBuilder::create_from_template(
                        &ver1_47::Versio::template(),
                        &mut source,
                    )?,
                    version,
                })
            }
            "1.48" => {
                let header = TokenBuilder::create_from_template(
                    &ver1_48::FileHeader::template(),
                    &mut source,
                )?;

                let mut uncompressed = header.to_le_vec();
                let content = decompress_to_vec(&source.get_rest_vec()).unwrap();
                uncompressed.extend(content);

                let mut source = Source::new(uncompressed);
                Ok(Scenario {
                    versio: TokenBuilder::create_from_template(
                        &ver1_48::Versio::template(),
                        &mut source,
                    )?,
                    version,
                })
            }
            "1.49" => {
                let header = TokenBuilder::create_from_template(
                    &ver1_49::FileHeader::template(),
                    &mut source,
                )?;

                let mut uncompressed = header.to_le_vec();
                let content = decompress_to_vec(&source.get_rest_vec()).unwrap();
                uncompressed.extend(content);

                let mut source = Source::new(uncompressed);
                Ok(Scenario {
                    versio: TokenBuilder::create_from_template(
                        &ver1_49::Versio::template(),
                        &mut source,
                    )?,
                    version,
                })
            }
            _ => Err("Unsupported version!".to_string()),
        }
    }

    pub fn from_versio(versio: &Token) -> Result<Self, String> {
        let version = versio
            .get_by_path("/file_header/version")
            .try_c4()
            .content()
            .clone();

        return Ok(Scenario {
            version,
            versio: versio.clone(),
        });
    }

    /// Encode a scenario struct to little endian vector of uint8
    /// If you want to generate an .aoe2scenario file, then this function are not recommended.
    /// The whole .aoe2scenario except file header part are compressed.
    /// The bytes generated by this function are raw uncompressed.
    /// See function to_export_le_vec for compressed bytes.  
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::Scenario;
    /// //Encode a scenario to a little endian vector of uint8
    /// let source_scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    /// let buffer = source_scenario.to_le_vec();
    /// ```
    pub fn to_le_vec(self) -> Vec<u8> {
        self.versio.to_le_vec()
    }

    /// Encode a scenario struct to little endian vector of uint8 that can be read by AoE2 DE
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::Scenario;
    /// //Encode a scenario to a little endian vector of uint8
    /// let source_scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    /// let buffer = source_scenario.to_le_export_vec();
    /// ```
    pub fn to_le_export_vec(&self) -> Vec<u8> {
        let file_header = &self.versio.try_map()["file_header"];
        let mut content = file_header.to_le_vec();
        let header_size = content.len();

        let uncompressed = &self.versio.to_le_vec()[header_size..];
        let mut compressed = compress_to_vec(uncompressed, 6);

        content.append(&mut compressed);

        content
    }

    /// Save the given scenario to the given path.
    /// The path will be created if not existed.
    /// # Examples
    ///
    /// ```
    /// use aoe2_probe::{Scenario, ExportFormat};
    /// let source_scenario = Scenario::from_file("./resources/chapter_1.aoe2scenario").unwrap();
    /// let buffer = source_scenario.to_file("./resources/temp.aoe2scenario", ExportFormat::AoE2Scenario);
    /// ```
    pub fn to_file(&self, file_path: &str, format: ExportFormat) -> Result<(), String> {
        match format {
            ExportFormat::AoE2Scenario => self.to_aoe2scenario(file_path),
            ExportFormat::JSON => self.to_json_file(file_path),
        }
    }

    pub fn to_aoe2scenario(&self, file_path: &str) -> Result<(), String> {
        let buffer = self.to_le_export_vec();

        let path = std::path::Path::new(file_path);
        let prefix = path.parent().expect("Fail to get parent path!");
        std::fs::create_dir_all(prefix).expect("Fail to create directory!");

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .unwrap();

        file.write_all(&buffer).expect("Fail to write content!");
        Ok(())
    }

    pub fn to_json_file(&self, file_path: &str) -> Result<(), String> {
        let buffer = serde_json::to_string(&self.versio).expect("Fail to convert to json format");

        let path = std::path::Path::new(file_path);
        let prefix = path.parent().expect("Fail to get parent path!");
        std::fs::create_dir_all(prefix).expect("Fail to create directory!");

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .unwrap();

        file.write_all(buffer.as_bytes())
            .expect("Fail to write content!");
        Ok(())
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    fn get_scenario_version(buffer: &[u8]) -> String {
        std::str::from_utf8(&buffer[0..4]).unwrap().to_string()
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum ExportFormat {
    AoE2Scenario = 0,
    JSON = 1,
}
