use indexmap::IndexMap;
use serde::Serialize;
use crate::action::Action;
use crate::comment::Comment;
use crate::model::field::Field;
use crate::model::Index;
use crate::model::property::Property;
use crate::model::relation::Relation;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub struct Model {
    path: Vec<String>,
    table_name: String,
    internal: bool,
    comment: Option<Comment>,
    fields: IndexMap<String, Field>,
    relations: IndexMap<String, Relation>,
    properties: IndexMap<String, Property>,
    indexes: IndexMap<String, Index>,
    primary_index: String,
    before_save: Vec<Pipeline>,
    after_save: Vec<Pipeline>,
    before_delete: Vec<Pipeline>,
    after_delete: Vec<Pipeline>,
    can_read: Vec<Pipeline>,
    can_mutate: Vec<Pipeline>,
    actions: Vec<Action>,
    //migration: Migration,
}

impl Model {

}