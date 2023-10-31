use std::collections::BTreeMap;
use serde::Serialize;
use teo_parser::ast::interface::InterfaceDeclarationShapeResolved;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::interface::field::Field;

#[derive(Debug, Serialize)]
pub struct Interface {
    pub path: Vec<String>,
    pub comment: Option<Comment>,
    pub fields: BTreeMap<String, Field>,
    pub generic_names: Vec<String>,
    pub extends: Vec<Type>,
    pub cache: InterfaceCache,
}

impl Interface {
    
    pub fn new() -> Self {
        Self {
            path: vec![],
            comment: None,
            fields: Default::default(),
            generic_names: vec![],
            extends: vec![],
            cache: InterfaceCache::new()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InterfaceCache {
    pub shape: InterfaceDeclarationShapeResolved,
}

impl InterfaceCache {

    pub fn new() -> Self {
        Self {
            shape: InterfaceDeclarationShapeResolved::new(),
        }
    }
}