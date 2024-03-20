use teo_result::Error;
use crate::database::database::Database;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::Value;

impl TryFrom<Value> for Database {

    type Error = Error;

    fn try_from(ref value: Value) -> Result<Self, Self::Error> {
        let interface_enum_variant: InterfaceEnumVariant = value.try_into()?;
        match interface_enum_variant.value.as_str() {
            "mysql" => Ok(Database::MySQL),
            "postgres" => Ok(Database::PostgreSQL),
            "mongo" => Ok(Database::MongoDB),
            "sqlite" => Ok(Database::SQLite),
            _ => Err(Error::new(format!("invalid database name: {:?}", value)))
        }
    }
}