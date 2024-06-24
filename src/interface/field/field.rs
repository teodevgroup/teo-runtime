use std::collections::BTreeMap;
use serde::Serialize;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::model::field::is_optional::{IsOptional};
use crate::model::field::typed::Typed;
use crate::optionality::Optionality;
use crate::traits::documentable::Documentable;
use crate::traits::named::Named;
use crate::Value;

#[derive(Debug, Serialize)]
pub struct Field {
    pub(super) name: String,
    pub(super) comment: Option<Comment>,
    pub(super) r#type: Type,
    pub(super) optionality: Optionality,
    pub(super) data: BTreeMap<String, Value>,
}

impl Field {

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    pub fn r#type(&self) -> &Type {
        &self.r#type
    }

    pub fn optionality(&self) -> &Optionality {
        &self.optionality
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.data
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

impl Named for Field {

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Documentable for Field {

    fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "interface field"
    }
}

impl Typed for Field {

    fn r#type(&self) -> &Type {
        &self.r#type
    }
}