use serde::Serialize;
use crate::database::mongo::r#type::MongoDBType;
use crate::database::mysql::r#type::MySQLType;
use crate::database::postgres::r#type::PostgreSQLType;
use crate::database::sqlite::r#type::SQLiteType;

#[derive(Debug, Serialize, Clone)]
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
}