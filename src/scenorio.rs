use crate::generator::IncrementalGenerator;
use crate::versio::ver1_46;
use crate::versio::VersioIsh;
use std::cell::Ref;
use std::cell::RefCell;

use std::cell::RefMut;
use std::fs::OpenOptions;
use std::io::Write;
use std::rc::Rc;
use std::{
    fs::{self, File},
    io::Read,
};

pub struct Scenorio<'a> {
    versio: Rc<RefCell<dyn VersioIsh + 'a>>,
}

impl<'a> Scenorio<'a> {
    pub fn from_file(filename: &str) -> Self {
        let mut file = File::open(&filename).expect("File not found");
        let metadata = fs::metadata(&filename).expect("Unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("buffer overflow");

        let version = Self::get_scenorio_version(&mut buffer);
        let mut generator = IncrementalGenerator::init(buffer);

        let versio = match version.as_str() {
            "1.46" => ver1_46::Versio::from_buffer(&mut generator),
            _ => panic!("Unsupport version!"),
        };
        Scenorio {
            versio: Rc::new(RefCell::new(versio)),
        }
    }

    pub fn from_buffer(buffer: &Vec<u8>) -> Self {
        let mut buffer = buffer.clone();
        let version = Self::get_scenorio_version(&mut buffer);
        let mut generator = IncrementalGenerator::init(buffer);

        let versio = match version.as_str() {
            "1.46" => ver1_46::Versio::from_buffer(&mut generator),
            _ => panic!("Unsupport version!"),
        };
        Scenorio {
            versio: Rc::new(RefCell::new(versio)),
        }
    }

    pub fn to_file(&self, file_full_path: &str) {
        let mut buffer = Vec::<u8>::new();
        self.versio.borrow().to_buffer(&mut buffer);

        let path = std::path::Path::new(file_full_path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_full_path)
            .unwrap();

        file.write_all(&buffer).unwrap();
    }

    fn get_scenorio_version(buffer: &Vec<u8>) -> String {
        std::str::from_utf8(&buffer[0..4]).unwrap().to_string()
    }

    pub fn versio(&self) -> Ref<dyn VersioIsh + 'a> {
        self.versio.borrow()
    }

    pub fn versio_mut(&self) -> RefMut<dyn VersioIsh + 'a> {
        self.versio.borrow_mut()
    }
}
