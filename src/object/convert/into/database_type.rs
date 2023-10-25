use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;

use teo_result::Error;
use crate::database::mysql::r#type::MySQLType;
use crate::database::postgres::r#type::PostgreSQLType;
use crate::database::sqlite::r#type::SQLiteType;
use crate::database::mongo::r#type::MongoDBType;
use crate::database::r#type::DatabaseType;
use crate::object::Object;

impl TryFrom<&Object> for DatabaseType {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        let teon: Value = value.try_into()?;
        let enum_variant: EnumVariant = teon.try_into()?;
        if !enum_variant.value.is_string() {
            Err(Error::new(format!("object is not enum variant: {:?}", value)))?
        }
        match enum_variant.path.last().unwrap().as_str() {
            "MySQLDatabaseType" => {
                match enum_variant.value.as_str().unwrap() {
                    "varChar" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::VarChar(len)))
                    },
                    "text" => Ok(DatabaseType::MySQLType(MySQLType::Text)),
                    "char" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::Char(len)))
                    },
                    "tinyText" => Ok(DatabaseType::MySQLType(MySQLType::TinyText)),
                    "mediumText" => Ok(DatabaseType::MySQLType(MySQLType::MediumText)),
                    "longText" => Ok(DatabaseType::MySQLType(MySQLType::LongText)),
                    "bit" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::Bit(len)))
                    },
                    "tinyInt" => {
                        let len = enum_variant.args.as_ref().unwrap().get("len").unwrap().as_int().unwrap();
                        let signed = enum_variant.args.unwrap().get("signed").unwrap().as_bool().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::TinyInt(len, signed)))
                    },
                    "int" => {
                        let len = enum_variant.args.as_ref().unwrap().get("len").map(|v| v.as_int()).flatten();
                        let signed = enum_variant.args.unwrap().get("signed").unwrap().as_bool().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::Int(len, signed)))
                    },
                    "smallInt" => {
                        let len = enum_variant.args.as_ref().unwrap().get("len").map(|v| v.as_int()).flatten();
                        let signed = enum_variant.args.unwrap().get("signed").unwrap().as_bool().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::SmallInt(len, signed)))
                    },
                    "mediumInt" => {
                        let len = enum_variant.args.as_ref().unwrap().get("len").map(|v| v.as_int()).flatten();
                        let signed = enum_variant.args.unwrap().get("signed").unwrap().as_bool().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::MediumInt(len, signed)))
                    },
                    "bigInt" => {
                        let len = enum_variant.args.as_ref().unwrap().get("len").map(|v| v.as_int()).flatten();
                        let signed = enum_variant.args.unwrap().get("signed").unwrap().as_bool().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::BigInt(len, signed)))
                    },
                    "year" => Ok(DatabaseType::MySQLType(MySQLType::Year)),
                    "float" => Ok(DatabaseType::MySQLType(MySQLType::Float)),
                    "double" => Ok(DatabaseType::MySQLType(MySQLType::Double)),
                    "decimal" => {
                        let precision = enum_variant.args.as_ref().unwrap().get("precision").unwrap().as_int().unwrap();
                        let scale = enum_variant.args.unwrap().get("scale").unwrap().as_int().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::Decimal(precision, scale)))
                    },
                    "dateTime" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::DateTime(len)))
                    },
                    "date" => Ok(DatabaseType::MySQLType(MySQLType::Date)),
                    "time" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::Time(len)))
                    },
                    "timestamp" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::MySQLType(MySQLType::Timestamp(len)))
                    },
                    "json" => Ok(DatabaseType::MySQLType(MySQLType::Json)),
                    "longBlob" => Ok(DatabaseType::MySQLType(MySQLType::LongBlob)),
                    "binary" => Ok(DatabaseType::MySQLType(MySQLType::Binary)),
                    "varBinary" => Ok(DatabaseType::MySQLType(MySQLType::VarBinary)),
                    "tinyBlob" => Ok(DatabaseType::MySQLType(MySQLType::TinyBlob)),
                    "blob" => Ok(DatabaseType::MySQLType(MySQLType::Blob)),
                    "mediumBlob" => Ok(DatabaseType::MySQLType(MySQLType::MediumBlob)),
                    _ => panic!(),
                }
            },
            "PostgreSQLDatabaseType" => {
                match enum_variant.value.as_str().unwrap() {
                    "varChar" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::PostgreSQLType(PostgreSQLType::VarChar(len)))
                    },
                    "text" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Text)),
                    "char" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Char(len)))
                    },
                    "bit" => {
                        let len = enum_variant.args.unwrap().get("len").unwrap().as_int().unwrap();
                        Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Bit(len)))
                    },
                    "varBit" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::VarBit)),
                    "uuid" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::UUID)),
                    "xml" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Xml)),
                    "inet" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Inet)),
                    "boolean" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Boolean)),
                    "integer" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Integer)),
                    "smallInt" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::SmallInt)),
                    "int" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Int)),
                    "bigInt" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::BigInt)),
                    "oid" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Oid)),
                    "doublePrecision" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::DoublePrecision)),
                    "real" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Real)),
                    "decimal" => {
                        let precision = enum_variant.args.as_ref().unwrap().get("precision").unwrap().as_int().unwrap();
                        let scale = enum_variant.args.unwrap().get("scale").unwrap().as_int().unwrap();
                        Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Decimal(precision, scale)))
                    },
                    "money" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Money)),
                    "timestamp" => {
                        let len  = enum_variant.args.clone().unwrap().get("len").unwrap().as_int().unwrap();
                        let tz = enum_variant.args.unwrap().get("tz").unwrap().as_bool().unwrap();
                        Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Timestamp(len, tz)))
                    },
                    "date" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Date)),
                    "time" => {
                        let tz = enum_variant.args.clone().unwrap().get("tz").unwrap().as_bool().unwrap();
                        Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Time(tz)))
                    },
                    "json" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Json)),
                    "jsonB" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::JsonB)),
                    "byteA" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::ByteA)),
                    _ => panic!(),
                }
            },
            "SQLiteDatabaseType" => {
                match enum_variant.value.as_str().unwrap() {
                    "text" => Ok(DatabaseType::SQLiteType(SQLiteType::Text)),
                    "integer" => Ok(DatabaseType::SQLiteType(SQLiteType::Integer)),
                    "real" => Ok(DatabaseType::SQLiteType(SQLiteType::Real)),
                    "decimal" => Ok(DatabaseType::SQLiteType(SQLiteType::Decimal)),
                    "blob" => Ok(DatabaseType::SQLiteType(SQLiteType::Blob)),
                    _ => panic!(),
                }
            },
            "MongoDBDatabaseType" => {
                match enum_variant.value.as_str().unwrap() {
                    "string" => Ok(DatabaseType::MongoDBType(MongoDBType::String)),
                    "bool" => Ok(DatabaseType::MongoDBType(MongoDBType::Bool)),
                    "int" => Ok(DatabaseType::MongoDBType(MongoDBType::Int)),
                    "long" => Ok(DatabaseType::MongoDBType(MongoDBType::Long)),
                    "double" => Ok(DatabaseType::MongoDBType(MongoDBType::Double)),
                    "date" => Ok(DatabaseType::MongoDBType(MongoDBType::Date)),
                    "timestamp" => Ok(DatabaseType::MongoDBType(MongoDBType::Timestamp)),
                    "binData" => Ok(DatabaseType::MongoDBType(MongoDBType::BinData)),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }
}