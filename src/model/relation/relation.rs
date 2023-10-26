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
    pub fields: Vec<String>,
    pub references: Vec<String>,
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
            fields: vec![],
            references: vec![],
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

    pub fn iter(&self) -> RelationIter {
        RelationIter { index: 0, relation: self }
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn model_path(&self) -> Vec<&str> {
        self.model.iter().map(AsRef::as_ref).collect()
    }

    pub fn through_path(&self) -> Option<Vec<&str>> {
        self.through.as_ref().map(|t| t.iter().map(AsRef::as_ref).collect())
    }

    pub fn local(&self) -> Option<&str> {
        self.local.as_ref().map(AsRef::as_ref)
    }

    pub fn foreign(&self) -> Option<&str> {
        self.foreign.as_ref().map(AsRef::as_ref)
    }

    pub fn has_join_table(&self) -> bool {
        self.through_path().is_some()
    }

    pub fn fields(&self) -> Vec<&str> {
        self.fields.iter().map(AsRef::as_ref).collect()
    }

    pub fn references(&self) -> Vec<&str> {
        self.references.iter().map(AsRef::as_ref).collect()
    }
}

impl Named for Relation {

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl IsOptional for Relation {

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

pub struct RelationIter<'a> {
    index: usize,
    relation: &'a Relation,
}

impl<'a> Iterator for RelationIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.relation.fields.get(self.index) {
            let result = Some((f.as_str(), self.relation.references.get(self.index).unwrap().as_str()));
            self.index += 1;
            result
        } else {
            None
        }
    }
}