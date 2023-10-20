use std::collections::BTreeMap;
use serde::Serialize;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::database::r#type::DatabaseType;
use crate::model::field::Index;
use crate::object::Object;
use crate::optionality::Optionality;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub struct Property {
    pub name: String,
    pub comment: Option<Comment>,
    pub optionality: Optionality,
    pub r#type: Type,
    pub database_type: DatabaseType,
    pub dependencies: Vec<String>,
    pub setter: Option<Pipeline>,
    pub getter: Option<Pipeline>,
    pub input_omissible: bool,
    pub output_omissible: bool,
    pub cached: bool,
    pub index: Option<Index>,
    pub data: BTreeMap<String, Object>,
}

impl Property {

    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            comment: None,
            optionality: Optionality::Required,
            r#type: Type::Undetermined,
            database_type: DatabaseType::Undetermined,
            dependencies: vec![],
            setter: None,
            getter: None,
            input_omissible: false,
            output_omissible: false,
            cached: false,
            index: None,
            data: Default::default(),
        }
    }
}
