use std::collections::{BTreeMap, BTreeSet};
use maplit::{btreemap, btreeset};
use serde::Serialize;
use crate::comment::Comment;
use crate::database::database::Database;
use crate::model::Field;
use crate::model::field::is_optional::IsOptional;
use crate::model::field::named::Named;
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
    pub fields: BTreeSet<String>,
    pub references: BTreeSet<String>,
    pub delete: Delete,
    pub update: Update,
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
            fields: btreeset!{},
            references: btreeset!{},
            foreign: None,
            local: None,
            delete: Delete::Default,
            update: Update::Default,
            has_foreign_key: false,
            data: btreemap! {},
        }
    }

    pub fn finalize(&mut self, database: Database, fields: Vec<&Field>) {
        self.has_foreign_key = if self.through.is_some() {
            false
        } else {
            self.fields.iter().find(|name| fields.iter().find(|f| f.name() == name.as_str() && f.foreign_key).is_some()).is_some()
        }
    }

    pub fn model_path(&self) -> Vec<&str> {
        self.model.iter().map(AsRef::as_ref).collect()
    }

    pub fn through_path(&self) -> Option<Vec<&str>> {
        self.through.map(|t| t.iter().map(AsRef::as_ref).collect())
    }
}

impl Named for &Relation {

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl IsOptional for &mut Relation {

    fn is_optional(&self) -> bool {
        self.optionality.is_any_optional()
    }

    fn is_required(&self) -> bool {
        self.optionality.is_required()
    }

    fn set_optional(&mut self) {
        self.optionality = Optionality::Optional;
    }

    fn set_required(&mut self) {
        self.optionality = Optionality::Required;
    }
}