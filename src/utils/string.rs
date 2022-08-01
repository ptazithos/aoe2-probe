use crate::{
    io::Source,
    parse::serde::{Deserialize, Serialize},
};

#[derive(Clone)]
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

pub trait Countable {
    fn new() -> Self;
    fn count() -> usize;
}

#[derive(Clone)]
pub struct Short {}
impl Countable for Short {
    fn new() -> Self {
        Short {}
    }

    fn count() -> usize {
        4
    }
}

#[derive(Clone)]
pub struct Long {}
impl Countable for Long {
    fn new() -> Self {
        Long {}
    }

    fn count() -> usize {
        256
    }
}

#[derive(Clone)]
pub struct Chars<T: Countable> {
    capacity: usize,
    raw: String,
    tag: T,
}

impl<T> Chars<T>
where
    T: Countable,
{
    pub fn new(content: &str) -> Self {
        if content.len() > T::count() {
            panic!("Out of the fixed capacity!")
        }
        Chars {
            capacity: T::count(),
            raw: content.to_string(),
            tag: T::new(),
        }
    }

    pub fn content(&self) -> &String {
        &self.raw
    }

    pub fn set_content(&mut self, content: &str) {
        if content.len() > self.capacity {
            panic!("Out of the fixed capacity!")
        }
        self.raw = content.to_string();
    }

    pub fn tag(&self) -> &T {
        &self.tag
    }
}

impl<T> Deserialize for Chars<T>
where
    T: Countable,
{
    fn from_le_vec(source: &mut Source) -> Self {
        let capacity = T::count();
        let raw = String::from_utf8_lossy(&source.get_vec(capacity as usize)[..]).to_string();

        Chars::<T>::new(&raw)
    }
}

impl<T> Serialize for Chars<T>
where
    T: Countable,
{
    fn to_le_vec(&self) -> Vec<u8> {
        let content = self.raw.clone().into_bytes();
        let mut container = vec![0; self.capacity];
        for (index, _) in content.iter().enumerate() {
            container[index] = content[index];
        }
        container
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
