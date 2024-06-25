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
    pub(super) inner: Arc<Inner>
}

#[derive(Debug)]
pub(super) struct Inner {
    pub(super) path: Vec<String>,
    pub(super) comment: Option<Comment>,
    pub(super) option: bool,
    pub(super) interface: bool,
    pub(super) members: Vec<Member>,
    pub(super) data: BTreeMap<String, Value>,
    pub(super) member_names: Vec<String>,
}

impl Enum {

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn option(&self) -> bool {
        self.inner.option
    }

    pub fn interface(&self) -> bool {
        self.inner.interface
    }

    pub fn members(&self) -> &Vec<Member> {
        &self.inner.members
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.inner.data
    }

    pub fn member_names(&self) -> &Vec<String> {
        &self.inner.member_names
    }
}

impl Named for Enum {

    fn name(&self) -> &str {
        self.inner.path.last().unwrap().as_str()
    }
}

impl Documentable for Enum {

    fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "enum"
    }
}