use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use crate::comment::Comment;
use crate::r#enum::{Enum, r#enum};
use crate::r#enum::member::Member;
use crate::Value;

#[derive(Debug, Clone)]
pub struct Builder {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    pub path: Vec<String>,
    pub comment: Option<Comment>,
    pub option: bool,
    pub interface: bool,
    pub members: Vec<Member>,
    pub data: Arc<Mutex<BTreeMap<String, Value>>>,
}

impl Builder {
    pub fn new(path: Vec<String>, comment: Option<Comment>, option: bool, interface: bool, members: Vec<Member>) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                comment,
                option,
                interface,
                members,
                data: Arc::new(Mutex::new(btreemap! {})),
            })
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
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

    pub fn data(&self) -> BTreeMap<String, Value> {
        self.inner.data.lock().unwrap().clone()
    }

    pub fn set_data(&self, data: BTreeMap<String, Value>) {
        *self.inner.data.lock().unwrap() = data;
    }

    pub fn insert_data_entry(&self, key: String, value: Value) {
        self.inner.data.lock().unwrap().insert(key, value);
    }

    pub fn remove_data_entry(&self, key: &str) {
        self.inner.data.lock().unwrap().remove(key);
    }

    pub fn data_entry(&self, key: &str) -> Option<Value> {
        self.inner.data.lock().unwrap().get(key).cloned()
    }

    pub fn build(self) -> Enum {
        Enum {
            inner: Arc::new(r#enum::Inner {
                path: self.inner.path.clone(),
                comment: self.inner.comment.clone(),
                option: self.inner.option,
                interface: self.inner.interface,
                members: self.inner.members.clone(),
                data: self.inner.data.lock().unwrap().clone(),
                member_names: self.members().iter().map(|m| m.name.clone()).collect(),
            })
        }
    }
}