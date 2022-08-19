use std::{
    collections::HashMap,
    fmt,
    ops::{Index, IndexMut},
    sync::Arc,
};

use linked_hash_map::{Iter, LinkedHashMap};
use serde::{Deserialize, Serialize};

use crate::parse::{Encode, Token};

type Patch = Arc<dyn Fn(&mut PatchedMap, &mut Token) + Send>;
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PatchedMap {
    raw_hashmap: LinkedHashMap<String, Token>,
    #[serde(skip)]
    pub patches: HashMap<String, Patch>,
}

impl PatchedMap {
    pub fn new() -> Self {
        PatchedMap {
            raw_hashmap: LinkedHashMap::new(),
            patches: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        PatchedMap {
            raw_hashmap: LinkedHashMap::with_capacity(capacity),
            patches: HashMap::new(),
        }
    }

    pub fn push_back<T: Into<Token>>(&mut self, key_str: &str, value: T) -> bool {
        let key = key_str.to_string();

        if self.raw_hashmap.contains_key(&key) {
            return false;
        }

        self.raw_hashmap.insert(key, value.into());
        true
    }

    pub fn update<T: Into<Token>>(&mut self, key: &str, value: T) -> Result<(), String> {
        if !self.raw_hashmap.contains_key(key) {
            return Err("Key doesn't exist!".to_string());
        }

        self.raw_hashmap[key] = value.into();
        Ok(())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.raw_hashmap.contains_key(&key.to_string())
    }

    pub fn keys(&self) -> linked_hash_map::Keys<String, Token> {
        self.raw_hashmap.keys()
    }

    pub fn iter(&self) -> SeqIter {
        SeqIter {
            index: 0,
            ele: self.raw_hashmap.iter(),
        }
    }
}

impl Index<&str> for PatchedMap {
    type Output = Token;
    fn index(&self, index: &str) -> &Self::Output {
        &self.raw_hashmap[&index.to_string()]
    }
}

impl IndexMut<&str> for PatchedMap {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.raw_hashmap.get_mut(&index.to_string()).unwrap()
    }
}

pub struct SeqIter<'a> {
    index: usize,
    ele: Iter<'a, String, Token>,
}

impl<'a> Iterator for SeqIter<'a> {
    type Item = (&'a String, &'a Token);

    fn next(&mut self) -> Option<Self::Item> {
        match self.ele.next() {
            Some((key, value)) => {
                self.index += 1;
                Some((key, value))
            }
            None => None,
        }
    }
}

impl Encode for PatchedMap {
    fn to_le_vec(&self) -> Vec<u8> {
        self.iter()
            .flat_map(|(_, token)| token.to_le_vec())
            .collect()
    }
}

impl fmt::Debug for PatchedMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PatchedMap")?;
        f.debug_map()
            .entries(self.iter().map(|(key, value)| (key, value)))
            .finish()
    }
}
