use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;

use teo_result::Error;
use crate::database::database::Database;
use crate::database::r#type::DatabaseType;
use crate::object::Object;

impl TryFrom<Object> for DatabaseType {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("object is not enum variant: {:?}", value)))?
        }
        match enum_variant.path.last().unwrap() {
            "MySQLDatabaseType" => {
                match enum_variant.value.as_str().unwrap() {

                }
            },
            "PostgreSQLDatabaseType" => {
                match enum_variant.value.as_str().unwrap() {

                }
            },
            "SQLiteDatabaseType" => {
                match enum_variant.value.as_str().unwrap() {

                }
            },
            "MongoDBDatabaseType" => {
                match enum_variant.value.as_str().unwrap() {

                }
            },
        }
    }
}