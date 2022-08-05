use std::{
    collections::HashMap,
    fmt,
    ops::{Index, IndexMut},
};

use crate::parse::{serde::Serialize, Token};

#[derive(Clone)]
pub enum DepType {
    Exist,
    Calculate,
}

#[derive(Clone)]
pub enum Manipulation {
    Equal,
    Multiple,
}

#[derive(Clone)]
pub struct NumericPatch {
    pub source: Vec<String>,
    pub dep_type: DepType,
    pub manipulation: Manipulation,
}

#[derive(Clone, Default)]
pub struct LinkedHashMap {
    raw_list: Vec<String>,
    raw_hashmap: HashMap<String, Token>,
    pub patchs: HashMap<String, NumericPatch>,
}

impl LinkedHashMap {
    pub fn new() -> Self {
        LinkedHashMap {
            raw_list: Vec::new(),
            raw_hashmap: HashMap::new(),
            patchs: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        LinkedHashMap {
            raw_list: Vec::with_capacity(capacity),
            raw_hashmap: HashMap::with_capacity(capacity),
            patchs: HashMap::new(),
        }
    }

    pub fn push_back<T: Into<Token>>(&mut self, key_str: &str, value: T) -> bool {
        let key = key_str.to_string();

        if self.raw_hashmap.contains_key(&key) {
            return false;
        }

        self.raw_list.push(key.clone());
        self.raw_hashmap.insert(key, value.into());
        true
    }

    pub fn update<T: Into<Token>>(&mut self, key_str: &str, value: T) -> bool {
        let key = key_str.to_string();

        if !self.raw_hashmap.contains_key(&key) {
            return false;
        }

        self.raw_hashmap.insert(key, value.into());
        true
    }

    pub fn contains(&self, key: &str) -> bool {
        self.raw_list.contains(&key.to_string())
    }

    pub fn keys(&self) -> &Vec<String> {
        &self.raw_list
    }

    pub fn iter(&self) -> SeqIter {
        SeqIter {
            index: 0,
            ele: self,
        }
    }
}

impl Index<&str> for LinkedHashMap {
    type Output = Token;
    fn index(&self, index: &str) -> &Self::Output {
        &self.raw_hashmap[&index.to_string()]
    }
}

impl IndexMut<&str> for LinkedHashMap {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.raw_hashmap.get_mut(&index.to_string()).unwrap()
    }
}

pub struct SeqIter<'a> {
    index: usize,
    ele: &'a LinkedHashMap,
}

impl<'a> Iterator for SeqIter<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let keys = self.ele.keys();
        if index < keys.len() {
            self.index += 1;
            let key = &keys[index];
            let res = &self.ele[key];
            Some(res)
        } else {
            None
        }
    }
}

impl Serialize for LinkedHashMap {
    fn to_le_vec(&self) -> Vec<u8> {
        self.iter().flat_map(|token| token.to_le_vec()).collect()
    }
}

impl fmt::Debug for LinkedHashMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LinkedHashMap")?;
        f.debug_map()
            .entries(
                self.iter()
                    .enumerate()
                    .map(|(index, token)| (&self.keys()[index], token)),
            )
            .finish()
    }
}
