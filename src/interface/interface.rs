use std::collections::BTreeMap;
use serde::Serialize;
use crate::comment::Comment;
use crate::interface::field::Field;

#[derive(Debug, Serialize)]
pub struct Interface {
    pub path: Vec<String>,
    pub comment: Option<Comment>,
    pub fields: BTreeMap<String, Field>,
}

impl Interface {
    
    pub fn new() -> Self {
        Self {
            path: vec![],
            comment: None,
            fields: Default::default(),
        }
    }
}

