use maplit::btreemap;
use std::collections::BTreeMap;
use std::sync::Arc;
use serde::Serialize;
use crate::comment::Comment;
use crate::r#enum::member::Member;
use crate::traits::documentable::Documentable;
use crate::traits::named::Named;
use crate::value::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Enum {
    inner: Arc<EnumInner>,
}

#[derive(Debug, Serialize)]
struct EnumInner {
    pub path: Vec<String>,
    pub comment: Option<Comment>,
    pub option: bool,
    pub interface: bool,
    pub members: Vec<Member>,
    pub data: BTreeMap<String, Value>,
    pub cache: Cache,
}

impl Enum {

    pub fn new() -> Self {
        Self {
            inner: Arc::new(EnumInner {
                path: vec![],
                comment: None,
                option: false,
                interface: false,
                members: vec![],
                data: btreemap! {},
                cache: Cache {
                    member_names: vec![]
                }
            })
        }
    }

    pub fn path(&self) -> Vec<&str> {
        self.inner.path.iter().map(AsRef::as_ref).collect()
    }

    pub fn finalize(&mut self) {
        self.inner.cache.member_names = self.inner.members.iter().map(|m| m.name.clone()).collect();
    }

    pub fn members(&self) -> &Vec<Member> {
        &self.inner.members
    }
}

#[derive(Debug, Serialize)]
pub struct Cache {
    pub member_names: Vec<String>,
}

impl Named for Enum {

    fn name(&self) -> &str {
        self.inner.path.last().unwrap().as_str()
    }
}

impl Documentable for Enum {

    fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "enum"
    }
}