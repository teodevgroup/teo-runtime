use serde::Serialize;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::model::field::is_optional::IsOptionalMut;
use crate::optionality::Optionality;

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

impl IsOptionalMut for &mut Field {

    fn set_optional(&mut self) {
        self.optionality = Optionality::Optional;
    }

    fn set_required(&mut self) {
        self.optionality = Optionality::Required;
    }
}

impl IsOptionalMut for Field {

    fn set_optional(&mut self) {
        self.set_optional()
    }

    fn set_required(&mut self) {
        self.set_required()
    }
}