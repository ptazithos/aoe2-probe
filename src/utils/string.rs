use crate::{
    element::serde::{Deserialize, Serialize},
    io::Source,
};

#[derive(Clone)]
pub struct PrefixString<T: Numeric> {
    capacity: T,
    pub raw: String,
}

impl<T> PrefixString<T>
where
    T: Numeric,
{
    pub fn new(capacity: T, raw: &str) -> Self {
        if capacity.to_usize() < raw.len() {
            panic!("Content is over capacity!")
        }
        PrefixString {
            capacity: capacity,
            raw: raw.to_string(),
        }
    }

    pub fn capacity(&self) -> T {
        self.capacity
    }
}

impl<T> Serialize for PrefixString<T>
where
    T: Numeric,
{
    fn to_le_vec(&self) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();
        let mut prefix = self.capacity.to_le_vec();
        let mut content = self.raw.clone().into_bytes();
        let mut container = vec![0; self.capacity.to_usize()];
        for (index, _) in content.iter().enumerate() {
            container[index] = content[index];
        }
        vec.append(&mut prefix);
        vec.append(&mut content);
        vec
    }
}

impl Deserialize for PrefixString<u16> {
    fn from_le_vec(source: &mut Source) -> Self {
        let capacity = u16::from_le_vec(source);
        let raw = String::from_utf8_lossy(&source.get_vec(capacity as usize)[..]).to_string();

        PrefixString::new(capacity, &raw)
    }
}

impl Deserialize for PrefixString<u32> {
    fn from_le_vec(source: &mut Source) -> Self {
        let capacity = u32::from_le_vec(source);
        let raw = String::from_utf8_lossy(&source.get_vec(capacity as usize)[..]).to_string();

        PrefixString::new(capacity, &raw)
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
