use teo_result::Error;
use crate::database::database::Database;
use crate::interface_enum_variant::InterfaceEnumVariant;
use crate::object::Object;

impl TryFrom<Object> for Database {

    type Error = Error;

    fn try_from(ref value: Object) -> std::result::Result<Self, Self::Error> {
        let enum_variant: InterfaceEnumVariant = value.try_into()?;
        match enum_variant.value.as_str() {
            "mysql" => Ok(Database::MySQL),
            "postgres" => Ok(Database::PostgreSQL),
            "mongo" => Ok(Database::MongoDB),
            "sqlite" => Ok(Database::SQLite),
            _ => Err(Error::new(format!("invalid database name: {:?}", value)))
        }
    }
}