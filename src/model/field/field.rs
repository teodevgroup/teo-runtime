use std::collections::BTreeMap;
use std::sync::Arc;
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

#[derive(Debug, Clone)]
pub struct Field {
    pub(super) inner: Arc<Inner>,
}

#[derive(Debug, Serialize)]
pub(super) struct Inner {
    pub(super) name: String,
    pub(super) comment: Option<Comment>,
    pub(super) column_name: String,
    pub(super) foreign_key: bool,
    pub(super) dropped: bool,
    pub(super) migration: Option<Migration>,
    pub(super) r#type: Type,
    pub(super) database_type: DatabaseType,
    pub(super) optionality: Optionality,
    pub(super) copy: bool,
    pub(super) read: Read,
    pub(super) write: Write,
    pub(super) atomic: bool,
    pub(super) r#virtual: bool,
    pub(super) input_omissible: bool,
    pub(super) output_omissible: bool,
    pub(super) index: Option<Index>,
    pub(super) queryable: bool,
    pub(super) sortable: bool,
    pub(super) auto: bool,
    pub(super) auto_increment: bool,
    pub(super) default: Option<Value>,
    pub(super) on_set: Pipeline,
    pub(super) on_save: Pipeline,
    pub(super) on_output: Pipeline,
    pub(super) can_mutate: Pipeline,
    pub(super) can_read: Pipeline,
    pub(super) data: BTreeMap<String, Value>,
    pub(super) availability: Availability,
}

impl Field {

    pub fn foreign_key(&self) -> bool {
        self.inner.foreign_key
    }

    pub fn dropped(&self) -> bool {
        self.inner.dropped
    }

    pub fn database_type(&self) -> &DatabaseType {
        &self.inner.database_type
    }

    pub fn optionality(&self) -> &Optionality {
        &self.inner.optionality
    }

    pub fn copy(&self) -> bool {
        self.inner.copy
    }

    pub fn read(&self) -> &Read {
        &self.inner.read
    }

    pub fn write(&self) -> &Write {
        &self.inner.write
    }

    pub fn atomic(&self) -> bool {
        self.inner.atomic
    }

    pub fn r#virtual(&self) -> bool {
        self.inner.r#virtual
    }

    pub fn input_omissible(&self) -> bool {
        self.inner.input_omissible
    }

    pub fn output_omissible(&self) -> bool {
        self.inner.output_omissible
    }

    pub fn queryable(&self) -> bool {
        self.inner.queryable
    }

    pub fn sortable(&self) -> bool {
        self.inner.sortable
    }

    pub fn auto(&self) -> bool {
        self.inner.auto
    }

    pub fn auto_increment(&self) -> bool {
        self.inner.auto_increment
    }

    pub fn default(&self) -> Option<&Value> {
        self.inner.default.as_ref()
    }

    pub fn on_set(&self) -> &Pipeline {
        &self.inner.on_set
    }

    pub fn on_save(&self) -> &Pipeline {
        &self.inner.on_save
    }

    pub fn on_output(&self) -> &Pipeline {
        &self.inner.on_output
    }

    pub fn can_mutate(&self) -> &Pipeline {
        &self.inner.can_mutate
    }

    pub fn can_read(&self) -> &Pipeline {
        &self.inner.can_read
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.inner.data
    }

    pub fn availability(&self) -> &Availability {
        &self.inner.availability
    }

    pub fn migration(&self) -> Option<&Migration> {
        self.inner.migration.as_ref()
    }
}

impl Named for Field {

    fn name(&self) -> &str {
        self.inner.name.as_str()
    }
}

impl ColumnNamed for Field {

    fn column_name(&self) -> &str {
        self.inner.column_name.as_str()
    }

}

impl Indexable for Field {

    fn index(&self) -> Option<&Index> {
        self.inner.index.as_ref()
    }
}

impl IsOptional for Field {

    fn is_optional(&self) -> bool {
        self.inner.optionality.is_any_optional()
    }

    fn is_required(&self) -> bool {
        self.inner.optionality.is_required()
    }
}

impl Typed for Field {

    fn r#type(&self) -> &Type {
        &self.inner.r#type
    }
}

impl Documentable for Field {

    fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "field"
    }
}

impl Serialize for Field {

        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            self.inner.serialize(serializer)
        }
}