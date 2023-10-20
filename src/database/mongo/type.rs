use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum MongoDBType {
    String,
    Bool,
    Int,
    Long,
    Double,
    Date,
    Timestamp,
    BinData,
    ObjectId,
}
