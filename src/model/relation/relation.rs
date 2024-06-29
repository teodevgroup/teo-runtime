use std::collections::BTreeMap;
use std::sync::Arc;
use serde::Serialize;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::model::field::is_optional::IsOptional;
use crate::model::field::typed::Typed;
use crate::traits::named::Named;
use crate::model::relation::delete::Delete;
use crate::model::relation::update::Update;
use crate::optionality::Optionality;
use crate::traits::documentable::Documentable;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Relation {
    pub(super) inner: Arc<Inner>
}

#[derive(Debug, Serialize)]
pub(super) struct Inner {
    pub(super) name: String,
    pub(super) comment: Option<Comment>,
    pub(super) r#type: Type,
    pub(super) optionality: Optionality,
    pub(super) model: Vec<String>,
    pub(super) through: Option<Vec<String>>,
    pub(super) local: Option<String>,
    pub(super) foreign: Option<String>,
    pub(super) is_vec: bool,
    pub(super) fields: Vec<String>,
    pub(super) references: Vec<String>,
    pub(super) delete: Delete,
    pub(super) update: Update,
    pub(super) has_foreign_key: bool,
    pub(super) data: BTreeMap<String, Value>,
}

impl Relation {

    pub fn iter(&self) -> RelationIter {
        RelationIter { index: 0, relation: self }
    }

    pub fn optionality(&self) -> &Optionality {
        &self.inner.optionality
    }

    pub fn delete(&self) -> Delete {
        self.inner.delete
    }

    pub fn update(&self) -> Update {
        self.inner.update
    }

    pub fn has_foreign_key(&self) -> bool {
        self.inner.has_foreign_key
    }

    pub fn model(&self) -> &Vec<String> {
        &self.inner.model
    }

    pub fn is_vec(&self) -> bool {
        self.inner.is_vec
    }

    pub fn through(&self) -> Option<&Vec<String>> {
        self.inner.through.as_ref()
    }

    pub fn len(&self) -> usize {
        self.inner.fields.len()
    }

    pub fn model_path(&self) -> &Vec<String> {
        &self.inner.model
    }

    pub fn through_path(&self) -> Option<&Vec<String>> {
        self.inner.through.as_ref()
    }

    pub fn local(&self) -> Option<&str> {
        self.inner.local.as_ref().map(AsRef::as_ref)
    }

    pub fn foreign(&self) -> Option<&str> {
        self.inner.foreign.as_ref().map(AsRef::as_ref)
    }

    pub fn has_join_table(&self) -> bool {
        self.through_path().is_some()
    }

    pub fn fields(&self) -> &Vec<String> {
        &self.inner.fields
    }

    pub fn references(&self) -> &Vec<String> {
        &self.inner.references
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.inner.data
    }
}

impl Named for Relation {

    fn name(&self) -> &str {
        self.inner.name.as_str()
    }
}

impl IsOptional for Relation {

    fn is_optional(&self) -> bool {
        self.inner.optionality.is_any_optional()
    }

    fn is_required(&self) -> bool {
        self.inner.optionality.is_required()
    }
}

pub struct RelationIter<'a> {
    index: usize,
    relation: &'a Relation,
}

impl<'a> Iterator for RelationIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.relation.fields().get(self.index) {
            let result = Some((f.as_str(), self.relation.references().get(self.index).unwrap().as_str()));
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl Documentable for Relation {

    fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "relation"
    }
}

impl Typed for Relation {

    fn r#type(&self) -> &Type {
        &self.inner.r#type
    }
}

impl Serialize for Relation {

        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
            self.inner.serialize(serializer)
        }
}