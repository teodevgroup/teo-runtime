use serde::Serialize;
use teo_parser::availability::Availability;
use crate::database::mongo::r#type::MongoDBType;
use crate::database::mysql::r#type::MySQLType;
use crate::database::postgres::r#type::PostgreSQLType;
use crate::database::sqlite::r#type::SQLiteType;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use teo_result::{Result, Error};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum DatabaseType {
    Undetermined,
    MySQLType(MySQLType),
    PostgreSQLType(PostgreSQLType),
    SQLiteType(SQLiteType),
    MongoDBType(MongoDBType),
}

impl DatabaseType {

    pub fn is_undetermined(&self) -> bool {
        match self {
            DatabaseType::Undetermined => true,
            _ => false,
        }
    }

    pub fn is_mysql(&self) -> bool {
        self.as_mysql().is_some()
    }

    pub fn as_mysql(&self) -> Option<&MySQLType> {
        match self {
            DatabaseType::MySQLType(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_postgres(&self) -> bool {
        self.as_postgres().is_some()
    }

    pub fn as_postgres(&self) -> Option<&PostgreSQLType> {
        match self {
            DatabaseType::PostgreSQLType(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_sqlite(&self) -> bool {
        self.as_sqlite().is_some()
    }

    pub fn as_sqlite(&self) -> Option<&SQLiteType> {
        match self {
            DatabaseType::SQLiteType(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_mongo(&self) -> bool {
        self.as_mongo().is_some()
    }

    pub fn as_mongo(&self) -> Option<&MongoDBType> {
        match self {
            DatabaseType::MongoDBType(t) => Some(t),
            _ => None,
        }
    }

    pub fn from_interface_enum_variant(interface_enum_variant: &InterfaceEnumVariant, availability: Availability) -> Result<Self> {
        if availability == Availability::mysql() {
            match interface_enum_variant.value.as_str() {
                "varChar" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    Ok(DatabaseType::MySQLType(MySQLType::VarChar(len)))
                },
                "text" => Ok(DatabaseType::MySQLType(MySQLType::Text)),
                "char" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    Ok(DatabaseType::MySQLType(MySQLType::Char(len)))
                },
                "tinyText" => Ok(DatabaseType::MySQLType(MySQLType::TinyText)),
                "mediumText" => Ok(DatabaseType::MySQLType(MySQLType::MediumText)),
                "longText" => Ok(DatabaseType::MySQLType(MySQLType::LongText)),
                "bit" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    Ok(DatabaseType::MySQLType(MySQLType::Bit(Some(len))))
                },
                "tinyInt" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    let signed: bool = interface_enum_variant.args.as_ref().unwrap().get("signed")?;
                    Ok(DatabaseType::MySQLType(MySQLType::TinyInt(Some(len), signed)))
                },
                "int" => {
                    let len: Option<i32> = interface_enum_variant.args.as_ref().unwrap().get_optional("len")?;
                    let signed: bool = interface_enum_variant.args.as_ref().unwrap().get("signed")?;
                    Ok(DatabaseType::MySQLType(MySQLType::Int(len, signed)))
                },
                "smallInt" => {
                    let len: Option<i32> = interface_enum_variant.args.as_ref().unwrap().get_optional("len")?;
                    let signed: bool = interface_enum_variant.args.as_ref().unwrap().get("signed")?;
                    Ok(DatabaseType::MySQLType(MySQLType::SmallInt(len, signed)))
                },
                "mediumInt" => {
                    let len: Option<i32> = interface_enum_variant.args.as_ref().unwrap().get_optional("len")?;
                    let signed: bool = interface_enum_variant.args.as_ref().unwrap().get("signed")?;
                    Ok(DatabaseType::MySQLType(MySQLType::MediumInt(len, signed)))
                },
                "bigInt" => {
                    let len: Option<i32> = interface_enum_variant.args.as_ref().unwrap().get_optional("len")?;
                    let signed: bool = interface_enum_variant.args.as_ref().unwrap().get("signed")?;
                    Ok(DatabaseType::MySQLType(MySQLType::BigInt(len, signed)))
                },
                "year" => Ok(DatabaseType::MySQLType(MySQLType::Year)),
                "float" => Ok(DatabaseType::MySQLType(MySQLType::Float)),
                "double" => Ok(DatabaseType::MySQLType(MySQLType::Double)),
                "decimal" => {
                    let precision: i32 = interface_enum_variant.args.as_ref().unwrap().get("precision")?;
                    let scale: i32 = interface_enum_variant.args.as_ref().unwrap().get("scale")?;
                    Ok(DatabaseType::MySQLType(MySQLType::Decimal(precision, scale)))
                },
                "dateTime" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    Ok(DatabaseType::MySQLType(MySQLType::DateTime(len)))
                },
                "date" => Ok(DatabaseType::MySQLType(MySQLType::Date)),
                "time" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    Ok(DatabaseType::MySQLType(MySQLType::Time(len)))
                },
                "timestamp" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
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
        } else if availability == Availability::postgres() {
            match interface_enum_variant.value.as_str() {
                "varChar" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    Ok(DatabaseType::PostgreSQLType(PostgreSQLType::VarChar(len)))
                },
                "text" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Text)),
                "char" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Char(len)))
                },
                "bit" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
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
                    let precision: i32 = interface_enum_variant.args.as_ref().unwrap().get("precision")?;
                    let scale: i32 = interface_enum_variant.args.as_ref().unwrap().get("scale")?;
                    Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Decimal(precision, scale)))
                },
                "money" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Money)),
                "timestamp" => {
                    let len: i32 = interface_enum_variant.args.as_ref().unwrap().get("len")?;
                    let tz: bool = interface_enum_variant.args.as_ref().unwrap().get("tz")?;
                    Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Timestamp(len, tz)))
                },
                "date" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Date)),
                "time" => {
                    let tz: bool = interface_enum_variant.args.as_ref().unwrap().get("tz")?;
                    Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Time(tz)))
                },
                "json" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Json)),
                "jsonB" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::JsonB)),
                "byteA" => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::ByteA)),
                _ => panic!(),
            }
        } else if availability == Availability::sqlite() {
            match interface_enum_variant.value.as_str() {
                "text" => Ok(DatabaseType::SQLiteType(SQLiteType::Text)),
                "integer" => Ok(DatabaseType::SQLiteType(SQLiteType::Integer)),
                "real" => Ok(DatabaseType::SQLiteType(SQLiteType::Real)),
                "decimal" => Ok(DatabaseType::SQLiteType(SQLiteType::Decimal)),
                "blob" => Ok(DatabaseType::SQLiteType(SQLiteType::Blob)),
                _ => panic!(),
            }
        } else if availability == Availability::mongo() {
            match interface_enum_variant.value.as_str() {
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
        } else {
            Err(Error::new("invalid availability when fetching database type"))
        }
    }
}