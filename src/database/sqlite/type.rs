use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum SQLiteType {
    Text,
    Integer,
    Real,
    Decimal,
    Blob,
}
