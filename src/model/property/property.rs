use std::collections::BTreeMap;
use serde::Serialize;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::comment::Comment;
use crate::database::database::Database;
use crate::database::r#type::DatabaseType;
use crate::model::field::Index;
use crate::object::Object;
use crate::optionality::Optionality;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub struct Property {
    pub name: String,
    pub comment: Option<Comment>,
    pub column_name: String,
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
            column_name: "".to_string(),
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

    pub(crate) fn set_required(&mut self) {
        self.optionality = Optionality::Required;
    }

    pub(crate) fn set_optional(&mut self) {
        self.optionality = Optionality::Optional;
        self.input_omissible = true;
        self.output_omissible = true;
    }

    pub(crate) fn finalize(&mut self, database: Database) -> Result<()> {
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
}
