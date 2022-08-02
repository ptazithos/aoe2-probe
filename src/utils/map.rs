use std::{collections::HashMap, fmt, ops::Index};

use crate::parse::{serde::Serialize, Token};

#[derive(Clone)]
pub struct LinkedHashMap {
    raw_list: Vec<String>,
    raw_hashmap: HashMap<String, Token>,
}

impl LinkedHashMap {
    pub fn new() -> Self {
        LinkedHashMap {
            raw_list: Vec::new(),
            raw_hashmap: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        LinkedHashMap {
            raw_list: Vec::with_capacity(capacity),
            raw_hashmap: HashMap::with_capacity(capacity),
        }
    }

    pub fn push_back<T: Into<Token>>(&mut self, key_str: &str, value: T) -> bool {
        let key = key_str.to_string();

        if self.raw_hashmap.contains_key(&key) {
            return false;
        }

        self.raw_list.push(key.clone());
        self.raw_hashmap.insert(key, value.into());
        return true;
    }

    pub fn update<T: Into<Token>>(&mut self, key_str: &str, value: T) -> bool {
        let key = key_str.to_string();

        if !self.raw_hashmap.contains_key(&key) {
            return false;
        }

        self.raw_hashmap.insert(key, value.into());
        return true;
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
        self.iter()
            .map(|token| token.to_le_vec())
            .flatten()
            .collect()
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
