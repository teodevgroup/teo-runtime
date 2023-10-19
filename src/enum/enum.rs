use serde::Serialize;
use crate::comment::Comment;
use crate::r#enum::member::Member;

#[derive(Debug, Serialize)]
pub struct Enum {
    pub path: Vec<String>,
    pub comment: Option<Comment>,
    pub members: Vec<Member>,
    pub(crate) cache: Cache,
}

impl Enum {

    pub fn new() -> Self {
        Self {
            path: vec![],
            comment: None,
            members: vec![],
            cache: Cache {
                member_names: vec![]
            }
        }
    }

    pub fn finalize(&mut self) {
        self.cache.member_names = self.members.iter().map(|m| m.name.clone()).collect();
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Cache {
    pub(crate) member_names: Vec<String>,
}