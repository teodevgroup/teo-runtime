use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
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
