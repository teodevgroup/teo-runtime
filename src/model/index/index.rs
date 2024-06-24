use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::model::index::item::Item;
use crate::model::index::r#type;

#[derive(Educe)]
#[educe(Debug, PartialEq, Eq, Hash)]
#[derive(Clone)]
pub struct Index {
    inner: Arc<Inner>
}

#[derive(Educe)]
#[educe(Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Clone)]
struct Inner {
    pub r#type: r#type::Type,
    pub name: String,
    pub items: Vec<Item>,
    #[serde(skip)]
    #[educe(PartialEq(ignore))] #[educe(Hash(ignore))]
    pub keys: Vec<String>,
}

impl Index {

    pub fn new(r#type: r#type::Type, name: String, items: Vec<Item>) -> Self {
        let keys = items.iter().map(|i| i.field.clone()).collect();
        Self {
            inner: Arc::new(Inner {
                r#type,
                name,
                items,
                keys,
            })
        }
    }

    pub fn r#type(&self) -> r#type::Type {
        self.inner.r#type
    }

    pub fn name(&self) -> &str {
        self.inner.name.as_str()
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.inner.items
    }

    pub fn keys(&self) -> &Vec<String> {
        &self.inner.keys
    }
}