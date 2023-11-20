use serde::Serialize;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::model::field::is_optional::{IsOptional};
use crate::model::field::typed::Typed;
use crate::optionality::Optionality;
use crate::traits::documentable::Documentable;
use crate::traits::named::Named;

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub comment: Option<Comment>,
    pub r#type: Type,
    pub optionality: Optionality,
}

impl Field {

    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            comment: None,
            r#type: Type::Undetermined,
            optionality: Optionality::Required,
        }
    }
}

impl IsOptional for Field {

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

impl Named for Field {

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Documentable for Field {

    fn comment(&self) -> Option<&Comment> {
        self.comment()
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