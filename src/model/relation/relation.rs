use std::collections::BTreeMap;
use maplit::btreemap;
use serde::Serialize;
use crate::comment::Comment;
use crate::model::Field;
use crate::model::relation::delete::Delete;
use crate::model::relation::update::Update;
use crate::object::Object;
use crate::optionality::Optionality;

#[derive(Debug, Serialize)]
pub struct Relation {
    pub name: String,
    pub comment: Option<Comment>,
    pub optionality: Optionality,
    pub model: Vec<String>,
    pub through: Option<Vec<String>>,
    pub local: Option<String>,
    pub foreign: Option<String>,
    pub is_vec: bool,
    pub fields: Vec<String>,
    pub references: Vec<String>,
    pub delete_rule: Delete,
    pub update_rule: Update,
    pub has_foreign_key: bool,
    data: BTreeMap<String, Object>,
}

impl Relation {

    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            comment: None,
            optionality: Optionality::Optional,
            model: vec![],
            through: None,
            is_vec: false,
            fields: vec![],
            references: vec![],
            foreign: None,
            local: None,
            delete_rule: Delete::Default,
            update_rule: Update::Default,
            has_foreign_key: false,
            data: btreemap! {},
        }
    }

    pub fn finalize(&mut self, fields: Vec<&Field>) {

    }
}