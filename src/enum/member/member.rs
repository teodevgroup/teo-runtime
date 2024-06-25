
use maplit::btreemap;
use std::collections::BTreeMap;
use serde::Serialize;
use crate::value::Value;
use crate::comment::Comment;
use crate::traits::documentable::Documentable;
use crate::traits::named::Named;

#[derive(Debug, Serialize, Clone)]
pub struct Member {
    pub name: String,
    pub comment: Option<Comment>,
    pub value: Value,
    pub data: BTreeMap<String, Value>,
}

impl Member {

    pub fn new(name: String, value: Value, comment: Option<Comment>) -> Self {
        Self { name, value, comment, data: btreemap! {} }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.data
    }
}

impl Named for Member {

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Documentable for Member {

    fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "enum member"
    }
}