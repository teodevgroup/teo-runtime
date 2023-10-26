use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum SQLiteType {
    Text,
    Integer,
    Real,
    Decimal,
    Blob,
}
