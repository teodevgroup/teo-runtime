use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MySQLType {
    VarChar(i32),
    Text,
    Char(i32),
    TinyText,
    MediumText,
    LongText,
    Bit(i32),
    TinyInt(i32, bool),
    Int(bool),
    SmallInt(bool),
    MediumInt(bool),
    BigInt(bool),
    Year,
    Float,
    Double,
    Decimal(i32, i32), // p r
    DateTime(i32),
    Date(i32),
    Time(i32),
    Timestamp(i32),
    Json,
    LongBlob,
    Binary,
    VarBinary,
    TinyBlob,
    Blob,
    MediumBlob,
}

impl MySQLType {

    pub fn is_tiny_int(&self) -> bool {
        self.as_tiny_int().is_some()
    }

    pub fn as_tiny_int(&self) -> Option<(i32, bool)> {
        match self {
            MySQLType::TinyInt(len, signed) => Some((*len, *signed)),
            _ => None,
        }
    }

    pub fn is_time(&self) -> bool {
        self.as_time().is_some()
    }

    pub fn as_time(&self) -> Option<i32> {
        match self {
            MySQLType::Time(len) => Some(*len),
            _ => None,
        }
    }

    pub fn is_blob(&self) -> bool {
        match self {
            MySQLType::Blob => true,
            _ => false,
        }
    }
}