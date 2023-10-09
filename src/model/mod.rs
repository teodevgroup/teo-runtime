pub mod decorator;
pub mod field;
pub mod relation;
pub mod property;
pub mod object;
pub mod index;

use indexmap::IndexMap;
pub use object::Object;
pub use decorator::Decorator;
use serde::Serialize;
use crate::comment::Comment;
use crate::model::field::Field;
pub use crate::model::index::Index;
use crate::model::property::Property;
use crate::model::relation::Relation;

#[derive(Debug, Serialize)]
pub struct Model {
    path: Vec<String>,
    table_name: String,
    comment: Option<Comment>,
    fields: IndexMap<String, Field>,
    relations: IndexMap<String, Relation>,
    properties: IndexMap<String, Property>,
    indexes: IndexMap<String, Index>,
    primary_index: String,
    // before_save: Pipeline,
    // after_save: Pipeline,
    // before_delete: Pipeline,
    // after_delete: Pipeline,
    // can_read: Pipeline,
    // can_mutate: Pipeline,
}

impl Model {

}