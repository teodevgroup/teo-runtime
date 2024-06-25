use std::collections::BTreeMap;
use serde::Serialize;
use teo_parser::availability::Availability;
use teo_parser::r#type::Type;
pub use super::decorator::Decorator;
use crate::comment::Comment;
use crate::database::r#type::DatabaseType;
use crate::model::field::column_named::ColumnNamed;
use crate::model::field::Index;
use crate::model::field::indexable::{Indexable};
use crate::model::field::is_optional::IsOptional;
use crate::model::field::Migration;
use crate::traits::named::Named;
use crate::model::field::typed::Typed;
use crate::optionality::Optionality;
use crate::pipeline::pipeline::Pipeline;
use crate::readwrite::read::Read;
use crate::readwrite::write::Write;
use crate::traits::documentable::Documentable;
use crate::value::Value;

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub comment: Option<Comment>,
    pub column_name: String,
    pub foreign_key: bool,
    pub dropped: bool,
    pub migration: Option<Migration>,
    pub r#type: Type,
    pub database_type: DatabaseType,
    pub optionality: Optionality,
    pub copy: bool,
    pub read: Read,
    pub write: Write,
    pub atomic: bool,
    pub r#virtual: bool,
    pub input_omissible: bool,
    pub output_omissible: bool,
    pub index: Option<Index>,
    pub queryable: bool,
    pub sortable: bool,
    pub auto: bool,
    pub auto_increment: bool,
    pub default: Option<Value>,
    pub on_set: Pipeline,
    pub on_save: Pipeline,
    pub on_output: Pipeline,
    pub can_mutate: Pipeline,
    pub can_read: Pipeline,
    pub data: BTreeMap<String, Value>,
    pub availability: Availability,
}

impl Field {

    pub fn migration(&self) -> Option<&Migration> {
        self.migration.as_ref()
    }
}

impl Named for Field {

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ColumnNamed for Field {

    fn column_name(&self) -> &str {
        self.column_name.as_str()
    }

}

impl Indexable for Field {

    fn index(&self) -> Option<&Index> {
        self.index.as_ref()
    }

    fn set_index(&mut self, index: Index) {
        self.index = Some(index);
    }
}

impl IsOptional for Field {

    fn is_optional(&self) -> bool {
        self.optionality.is_any_optional()
    }

    fn is_required(&self) -> bool {
        self.optionality.is_required()
    }
}

impl Typed for Field {

    fn r#type(&self) -> &Type {
        &self.r#type
    }
}

impl Documentable for Field {

    fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "field"
    }
}