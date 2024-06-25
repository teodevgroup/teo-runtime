use std::collections::BTreeMap;
use std::sync::Arc;
use serde::Serialize;
use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::comment::Comment;
use crate::database::database::Database;
use crate::database::r#type::DatabaseType;
use crate::model::field::column_named::ColumnNamed;
use crate::model::field::Index;
use crate::model::field::indexable::{Indexable};
use crate::model::field::is_optional::IsOptional;
use crate::traits::named::Named;
use crate::model::field::typed::Typed;
use crate::optionality::Optionality;
use crate::pipeline::pipeline::Pipeline;
use crate::traits::documentable::Documentable;
use crate::value::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Property {
    pub(super) inner: Arc<Inner>
}

#[derive(Debug, Serialize)]
pub(super) struct Inner {
    pub(super) name: String,
    pub(super) comment: Option<Comment>,
    pub(super) column_name: String,
    pub(super) optionality: Optionality,
    pub(super) r#type: Type,
    pub(super) database_type: DatabaseType,
    pub(super) dependencies: Vec<String>,
    pub(super) setter: Option<Pipeline>,
    pub(super) getter: Option<Pipeline>,
    pub(super) input_omissible: bool,
    pub(super) output_omissible: bool,
    pub(super) cached: bool,
    pub(super) index: Option<Index>,
    pub(super) data: BTreeMap<String, Value>,
}

impl Property {

    pub fn optionality(&self) -> &Optionality {
        &self.inner.optionality
    }

    pub fn database_type(&self) -> &DatabaseType {
        &self.inner.database_type
    }

    pub fn dependencies(&self) -> &Vec<String> {
        &self.inner.dependencies
    }

    pub fn setter(&self) -> Option<&Pipeline> {
        self.inner.setter.as_ref()
    }

    pub fn getter(&self) -> Option<&Pipeline> {
        self.inner.getter.as_ref()
    }

    pub fn input_omissible(&self) -> bool {
        self.inner.input_omissible
    }

    pub fn output_omissible(&self) -> bool {
        self.inner.output_omissible
    }

    pub fn cached(&self) -> bool {
        self.inner.cached
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.inner.data
    }
}

impl Named for Property {

    fn name(&self) -> &str {
        self.inner.name.as_str()
    }
}

impl ColumnNamed for Property {

    fn column_name(&self) -> &str {
        self.inner.column_name.as_str()
    }
}

impl Indexable for Property {

    fn index(&self) -> Option<&Index> {
        self.inner.index.as_ref()
    }
}

impl IsOptional for Property {

    fn is_optional(&self) -> bool {
        self.inner.optionality.is_any_optional()
    }

    fn is_required(&self) -> bool {
        self.inner.optionality.is_required()
    }
}

impl Typed for Property {

    fn r#type(&self) -> &Type {
        &self.r#type
    }
}

impl Documentable for Property {

    fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "property"
    }
}