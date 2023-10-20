use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MongoDBType {
    String,
    Bool,
    Int,
    Long,
    Double,
    Date,
    Timestamp,
    BinData,
}
