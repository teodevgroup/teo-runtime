use std::collections::BTreeMap;
use indexmap::indexmap;
use serde::Serialize;
use teo_parser::ast::interface::InterfaceDeclarationResolved;
use teo_parser::r#type::synthesized_shape::SynthesizedShape;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::interface::field::Field;
use crate::traits::documentable::Documentable;
use crate::traits::named::Named;

#[derive(Debug, Serialize)]
pub struct Interface {
    pub path: Vec<String>,
    pub comment: Option<Comment>,
    pub fields: BTreeMap<String, Field>,
    pub generic_names: Vec<String>,
    pub extends: Vec<Type>,
    pub resolved: InterfaceDeclarationResolved,
}

impl Interface {
    
    pub fn new() -> Self {
        Self {
            path: vec![],
            comment: None,
            fields: Default::default(),
            generic_names: vec![],
            extends: vec![],
            resolved: InterfaceDeclarationResolved::new(SynthesizedShape::new(indexmap! {})),
        }
    }

    pub fn generic_names(&self) -> Vec<&str> {
        self.generic_names.iter().map(|g| g.as_str()).collect()
    }

    pub fn extends(&self) -> &Vec<Type> {
        &self.extends
    }
}

impl Named for Interface {

    fn name(&self) -> &str {
        self.path.last().map(|s| s.as_str()).unwrap()
    }
}

impl Documentable for Interface {

    fn comment(&self) -> Option<&Comment> {
        self.comment()
    }

    fn kind(&self) -> &'static str {
        "interface"
    }
}