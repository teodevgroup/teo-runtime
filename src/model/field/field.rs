use serde::Serialize;
use teo_parser::r#type::Type;
use teo_teon::Value;
pub use super::decorator::Decorator;
use crate::comment::Comment;
use crate::database::mysql::r#type::MySQLType;
use crate::database::r#type::DatabaseType;
use crate::model::field::Index;
use crate::model::field::Migration;
use crate::optionality::Optionality;
use crate::pipeline::pipeline::Pipeline;
use crate::previous::Previous;

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub column_name: Option<String>,
    pub foreign_key: bool,
    pub dropped: bool,
    pub migration: Option<Migration>,
    pub comment: Option<Comment>,
    pub r#type: Type,
    pub database_type: DatabaseType,
    pub optionality: Optionality,
    pub previous: Previous,
    pub atomic: bool,
    pub r#virtual: bool,
    pub input_omissible: bool,
    pub output_omissible: bool,
    pub index: Option<Index>,
    pub queryable: bool,
    pub sortable: bool,
    pub auto: bool,
    pub auto_increment: bool,
    pub identity: bool,
    pub identity_checker: Option<Pipeline>,
    pub default: Option<Value>,
    pub on_set_pipeline: Pipeline,
    pub on_save_pipeline: Pipeline,
    pub on_output_pipeline: Pipeline,
    pub can_mutate_pipeline: Pipeline,
    pub can_read_pipeline: Pipeline,
}

impl Field {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            column_name: None,
            foreign_key: false,
            dropped: false,
            migration: None,
            comment: None,
            r#type: Type::Undetermined,
            database_type: DatabaseType::MySQLType(MySQLType::Text),
            optionality: Optionality::Optional,
            previous: Previous::DontKeep,
            atomic: false,
            r#virtual: false,
            input_omissible: false,
            output_omissible: false,
            index: None,
            queryable: false,
            sortable: false,
            auto: false,
            auto_increment: false,
            identity: false,
            identity_checker: None,
            default: None,
            on_set_pipeline: Pipeline::new(),
            on_save_pipeline: Pipeline::new(),
            on_output_pipeline: Pipeline::new(),
            can_mutate_pipeline: Pipeline::new(),
            can_read_pipeline: Pipeline::new(),
        }
    }
}