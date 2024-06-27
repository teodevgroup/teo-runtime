use std::collections::BTreeMap;
use std::sync::Arc;
use serde::{Serialize, Serializer};
use crate::handler::Handler;
use crate::traits::named::Named;

#[derive(Debug)]
pub struct Group {
    pub(super) inner: Arc<Inner>
}

#[derive(Serialize, Debug)]
pub(super) struct Inner {
    pub(super) path: Vec<String>,
    pub(super) handlers: BTreeMap<String, Handler>,
}

impl Group {

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn handler(&self, name: &str) -> Option<&Handler> {
        self.inner.handlers.get(name)
    }
}

impl Named for Group {
    fn name(&self) -> &str {
        self.inner.path.last().unwrap().as_str()
    }
}

impl Serialize for Group {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self.inner.serialize(serializer)
    }
}