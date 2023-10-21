use std::collections::BTreeMap;
use maplit::btreemap;
use serde::Serialize;
use teo_result::Result;
use teo_parser::r#type::Type;
pub use super::decorator::Decorator;
use crate::comment::Comment;
use crate::database::database::Database;
use crate::database::mysql::r#type::MySQLType;
use crate::database::r#type::DatabaseType;
use crate::model::field::Index;
use crate::model::field::indexable::{Indexable, IndexableMut};
use crate::model::field::is_optional::IsOptionalMut;
use crate::model::field::Migration;
use crate::model::field::named::Named;
use crate::object::Object;
use crate::optionality::Optionality;
use crate::pipeline::pipeline::Pipeline;
use crate::previous::Previous;
use crate::readwrite::read::Read;
use crate::readwrite::write::Write;

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
    pub read: Read,
    pub write: Write,
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
    pub default: Option<Object>,
    pub on_set_pipeline: Pipeline,
    pub on_save_pipeline: Pipeline,
    pub on_output_pipeline: Pipeline,
    pub can_mutate_pipeline: Pipeline,
    pub can_read_pipeline: Pipeline,
    pub data: BTreeMap<String, Object>,
}

impl Field {

    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            column_name: "".to_string(),
            foreign_key: false,
            dropped: false,
            migration: None,
            comment: None,
            r#type: Type::Undetermined,
            database_type: DatabaseType::MySQLType(MySQLType::Text),
            optionality: Optionality::Optional,
            previous: Previous::DontKeep,
            atomic: false,
            read: Read::Read,
            write: Write::Write,
            r#virtual: false,
            input_omissible: false,
            output_omissible: false,
            index: None,
            queryable: false,
            sortable: false,
            auto: false,
            auto_increment: false,
            default: None,
            on_set_pipeline: Pipeline::new(),
            on_save_pipeline: Pipeline::new(),
            on_output_pipeline: Pipeline::new(),
            can_mutate_pipeline: Pipeline::new(),
            can_read_pipeline: Pipeline::new(),
            data: btreemap! {},
        }
    }

    pub fn finalize(&mut self, database: Database) -> Result<()> {
        // set default column name
        if self.column_name.is_empty() {
            self.column_name = self.name.clone();
        }
        // set default database type
        if self.database_type.is_undetermined() {
            self.database_type = database.default_database_type(&self.r#type)?;
        }
        Ok(())
    }

    pub fn column_name(&self) -> &str {
        self.column_name.as_str()
    }
}

impl Named for &Field {

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl IndexableMut for &mut Field {

    fn set_index(&mut self, index: Index) {
        self.index = Some(index);
    }
}

impl Indexable for &Field {

    fn index(&self) -> Option<&Index> {
        self.index.as_ref()
    }
}

impl IsOptionalMut for &mut Field {

    fn set_optional(&mut self) {
        self.optionality = Optionality::Optional;
        self.input_omissible = true;
        self.output_omissible = true;
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