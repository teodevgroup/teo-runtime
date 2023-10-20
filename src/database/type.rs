use serde::Serialize;
use crate::database::mongo::r#type::MongoDBType;
use crate::database::mysql::r#type::MySQLType;
use crate::database::postgres::r#type::PostgreSQLType;
use crate::database::sqlite::r#type::SQLiteType;

#[derive(Debug, Serialize)]
pub enum DatabaseType {
    MySQLType(MySQLType),
    PostgreSQLType(PostgreSQLType),
    SQLiteType(SQLiteType),
    MongoDBType(MongoDBType),
}