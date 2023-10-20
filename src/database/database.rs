use serde::Serialize;
use teo_parser::r#type::Type;
use teo_result::{Error, Result};
use crate::database::mongo::r#type::MongoDBType;
use crate::database::mysql::r#type::MySQLType;
use crate::database::postgres::r#type::PostgreSQLType;
use crate::database::sqlite::r#type::SQLiteType;
use crate::database::r#type::DatabaseType;

#[derive(Debug, Serialize)]
pub enum Database {
    MongoDB,
    MySQL,
    PostgreSQL,
    SQLite,
}

impl Database {

    pub fn default_database_type(&self, r#type: &Type) -> Result<DatabaseType> {
        match self {
            Database::MongoDB => self.default_mongo_database_type(r#type),
            Database::MySQL => self.default_mysql_database_type(r#type),
            Database::PostgreSQL => self.default_postgres_database_type(r#type),
            Database::SQLite => self.default_sqlite_database_type(r#type),
        }
    }

    fn default_mongo_database_type(&self, r#type: &Type) -> Result<DatabaseType> {
        match r#type {
            Type::Bool => Ok(DatabaseType::MongoDBType(MongoDBType::Bool)),
            Type::Int => Ok(DatabaseType::MongoDBType(MongoDBType::Int)),
            Type::Int64 => Ok(DatabaseType::MongoDBType(MongoDBType::Long)),
            Type::Float32 => Ok(DatabaseType::MongoDBType(MongoDBType::Double)),
            Type::Float => Ok(DatabaseType::MongoDBType(MongoDBType::Double)),
            Type::String => Ok(DatabaseType::MongoDBType(MongoDBType::String)),
            Type::ObjectId => Ok(DatabaseType::MongoDBType(MongoDBType::ObjectId)),
            Type::Date => Ok(DatabaseType::MongoDBType(MongoDBType::Date)),
            Type::DateTime => Ok(DatabaseType::MongoDBType(MongoDBType::Date)),
            Type::Array(inner) => todo!(),
            Type::EnumVariant(_, _) => Ok(DatabaseType::MongoDBType(MongoDBType::String)),
            Type::Optional(inner) => self.default_mongo_database_type(inner.as_ref()),
            _ => Err(Error::new(format!("unsupported mongo database type {}", r#type))),
        }
    }

    fn default_mysql_database_type(&self, r#type: &Type) -> Result<DatabaseType> {
        match r#type {
            Type::Bool => Ok(DatabaseType::MySQLType(MySQLType::TinyInt(1, false))),
            Type::Int => Ok(DatabaseType::MySQLType(MySQLType::Int(None, false))),
            Type::Int64 => Ok(DatabaseType::MySQLType(MySQLType::BigInt(None, false))),
            Type::Float32 => Ok(DatabaseType::MySQLType(MySQLType::Float)),
            Type::Float => Ok(DatabaseType::MySQLType(MySQLType::Double)),
            Type::Decimal => Ok(DatabaseType::MySQLType(MySQLType::Decimal(65, 30))),
            Type::String => Ok(DatabaseType::MySQLType(MySQLType::VarChar(191))),
            Type::Date => Ok(DatabaseType::MySQLType(MySQLType::Date)),
            Type::DateTime => Ok(DatabaseType::MySQLType(MySQLType::DateTime(3))),
            Type::Optional(inner) => self.default_mysql_database_type(inner.as_ref()),
            _ => Err(Error::new(format!("unsupported mysql database type {}", r#type))),
        }
    }

    fn default_postgres_database_type(&self, r#type: &Type) -> Result<DatabaseType> {
        match r#type {
            Type::Bool => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Boolean)),
            Type::Int => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Int)),
            Type::Int64 => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::BigInt)),
            Type::Float32 => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Real)),
            Type::Float => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::DoublePrecision)),
            Type::Decimal => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Decimal(65, 30))),
            Type::String => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Text)),
            Type::Date => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Date)),
            Type::DateTime => Ok(DatabaseType::PostgreSQLType(PostgreSQLType::Timestamp(3))),
            Type::Array(inner) => Ok(DatabaseType::PostgreSQLType(self.default_postgres_database_type(inner.as_ref())?.as_postgres().unwrap().clone())),
            Type::Optional(inner) => self.default_postgres_database_type(inner.as_ref()),
            _ => Err(Error::new(format!("unsupported postgres database type {}", r#type))),
        }
    }

    fn default_sqlite_database_type(&self, r#type: &Type) -> Result<DatabaseType> {
        match r#type {
            Type::Bool => Ok(DatabaseType::SQLiteType(SQLiteType::Integer)),
            Type::Int => Ok(DatabaseType::SQLiteType(SQLiteType::Integer)),
            Type::Int64 => Ok(DatabaseType::SQLiteType(SQLiteType::Integer)),
            Type::Float32 => Ok(DatabaseType::SQLiteType(SQLiteType::Real)),
            Type::Float => Ok(DatabaseType::SQLiteType(SQLiteType::Real)),
            Type::Decimal => Ok(DatabaseType::SQLiteType(SQLiteType::Decimal)),
            Type::String => Ok(DatabaseType::SQLiteType(SQLiteType::Text)),
            Type::Date => Ok(DatabaseType::SQLiteType(SQLiteType::Text)),
            Type::DateTime => Ok(DatabaseType::SQLiteType(SQLiteType::Text)),
            Type::Optional(inner) => self.default_sqlite_database_type(inner.as_ref()),
            _ => Err(Error::new(format!("unsupported sqlite database type {}", r#type))),
        }
    }
}