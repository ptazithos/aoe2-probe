use std::fmt;

use crate::{
    io::Source,
    parse::serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug)]
pub struct DynString<T: Numeric> {
    capacity: T,
    raw: String,
}

impl<T> DynString<T>
where
    T: Numeric,
{
    pub fn new(capacity: T, raw: &str) -> Self {
        if capacity.to_usize() < raw.len() {
            panic!("Content is over capacity!")
        }
        DynString {
            capacity: capacity,
            raw: raw.to_string(),
        }
    }

    pub fn capacity(&self) -> T {
        self.capacity
    }

    pub fn content(&self) -> &String {
        &self.raw
    }

    pub fn set_content(&mut self, content: &str) {
        self.raw = content.to_string();
    }
}

impl<T> Serialize for DynString<T>
where
    T: Numeric,
{
    fn to_le_vec(&self) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();
        let mut prefix = self.capacity.to_le_vec();
        let content = self.raw.clone().into_bytes();
        let mut container = vec![0; self.capacity.to_usize()];
        for (index, _) in content.iter().enumerate() {
            container[index] = content[index];
        }
        vec.append(&mut prefix);
        vec.append(&mut container);
        vec
    }
}

impl Deserialize for DynString<u16> {
    fn from_le_vec(source: &mut Source) -> Self {
        let capacity = u16::from_le_vec(source);
        let raw = String::from_utf8_lossy(&source.get_vec(capacity as usize)[..]).to_string();

        DynString::new(capacity, &raw)
    }
}

impl Deserialize for DynString<u32> {
    fn from_le_vec(source: &mut Source) -> Self {
        let capacity = u32::from_le_vec(source);
        let raw = String::from_utf8_lossy(&source.get_vec(capacity as usize)[..]).to_string();

        DynString::new(capacity, &raw)
    }
}

pub trait Numeric: Copy + Serialize {
    fn to_usize(&self) -> usize;
    fn from_usize(num: usize) -> Self;
}

impl Numeric for u16 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(num: usize) -> Self {
        num as u16
    }
}
impl Numeric for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(num: usize) -> Self {
        num as u32
    }
}

#[derive(Clone, Debug)]
pub struct C4 {
    raw: String,
}

impl C4 {
    pub fn new(content: &str) -> Self {
        if content.len() > 4 {
            panic!("Out of the fixed capacity!")
        }

        C4 {
            raw: content.to_string(),
        }
    }

    pub fn content(&self) -> &String {
        &self.raw
    }

    pub fn set_content(&mut self, content: &str) {
        if content.len() > 4 {
            panic!("Out of the fixed capacity!")
        }
        self.raw = content.to_string();
    }
}

impl Deserialize for C4 {
    fn from_le_vec(source: &mut Source) -> Self {
        let raw = String::from_utf8_lossy(&source.get_vec(4 as usize)[..]).to_string();

        C4::new(&raw)
    }
}

impl Serialize for C4 {
    fn to_le_vec(&self) -> Vec<u8> {
        let content = self.raw.clone().into_bytes();
        let mut container = vec![0; 4];
        for (index, _) in content.iter().enumerate() {
            container[index] = content[index];
        }
        container
    }
}

#[derive(Clone)]
pub struct C256 {
    raw: String,
}

impl C256 {
    pub fn new(content: &str) -> Self {
        if content.len() > 256 {
            panic!("Out of the fixed capacity!")
        }

        C256 {
            raw: content.to_string(),
        }
    }

    pub fn content(&self) -> &String {
        &self.raw
    }

    pub fn set_content(&mut self, content: &str) {
        if content.len() > 256 {
            panic!("Out of the fixed capacity!")
        }
        self.raw = content.to_string();
    }
}

impl Deserialize for C256 {
    fn from_le_vec(source: &mut Source) -> Self {
        let raw = String::from_utf8_lossy(&source.get_vec(256 as usize)[..]).to_string();
        C256::new(&raw)
    }
}

impl Serialize for C256 {
    fn to_le_vec(&self) -> Vec<u8> {
        let content = self.raw.clone().into_bytes();
        let mut container = vec![0; 256];
        for (index, _) in content.iter().enumerate() {
            container[index] = content[index];
        }
        container
    }
}

impl fmt::Debug for C256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw.trim_matches(char::from(0)))
    }
}
