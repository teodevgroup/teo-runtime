pub mod item;

use std::collections::BTreeSet;
use educe::Educe;
use serde::Serialize;
use crate::index;
pub use item::Item;

#[derive(Educe)]
#[educe(Debug, PartialEq)]
#[derive(Serialize)]
pub struct Index {
    pub r#type: index::Type,
    pub name: String,
    pub items: Vec<Item>,
    #[serde(skip)]
    #[educe(PartialEq(ignore))]
    pub cache: Cache,
}

#[derive(Debug, Clone)]
pub struct Cache {
    pub keys: BTreeSet<String>,
}

impl Index {

    pub fn new(r#type: index::Type, name: String, items: Vec<Item>) -> Self {
        let keys = items.iter().map(|i| i.field.clone()).collect();
        Self {
            r#type,
            name,
            items,
            cache: Cache { keys }
        }
    }

    pub fn r#type(&self) -> index::Type {
        self.r#type
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn keys(&self) -> &BTreeSet<String> {
        &self.cache.keys
    }
}