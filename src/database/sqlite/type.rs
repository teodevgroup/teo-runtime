use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum SQLiteType {
    Text,
    Integer,
    Real,
    Decimal,
    Blob,
}
