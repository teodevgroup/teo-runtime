use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum MySQLType {
    VarChar(i32),
    Text,
    Char(i32),
    TinyText,
    MediumText,
    LongText,
    Bit(Option<i32>),
    TinyInt(Option<i32>, bool),
    Int(Option<i32>, bool),
    SmallInt(Option<i32>, bool),
    MediumInt(Option<i32>, bool),
    BigInt(Option<i32>, bool),
    Year,
    Float,
    Double,
    Decimal(i32, i32),
    DateTime(i32),
    Date,
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


    pub fn is_var_char(&self) -> bool {
        self.as_var_char().is_some()
    }

    pub fn as_var_char(&self) -> Option<i32> {
        match self {
            MySQLType::VarChar(len) => Some(*len),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            MySQLType::Text => true,
            _ => false,
        }
    }

    pub fn is_char(&self) -> bool {
        self.as_char().is_some()
    }

    pub fn as_char(&self) -> Option<i32> {
        match self {
            MySQLType::Char(len) => Some(*len),
            _ => None,
        }
    }

    pub fn is_tiny_text(&self) -> bool {
        match self {
            MySQLType::TinyText => true,
            _ => false,
        }
    }

    pub fn is_medium_text(&self) -> bool {
        match self {
            MySQLType::MediumText => true,
            _ => false,
        }
    }

    pub fn is_long_text(&self) -> bool {
        match self {
            MySQLType::LongText => true,
            _ => false,
        }
    }

    pub fn is_bit(&self) -> bool {
        self.as_bit().is_some()
    }

    pub fn as_bit(&self) -> Option<Option<i32>> {
        match self {
            MySQLType::Bit(len) => Some(*len),
            _ => None,
        }
    }

    pub fn is_tiny_int(&self) -> bool {
        self.as_tiny_int().is_some()
    }

    pub fn as_tiny_int(&self) -> Option<(Option<i32>, bool)> {
        match self {
            MySQLType::TinyInt(len, signed) => Some((*len, *signed)),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        self.as_int().is_some()
    }

    pub fn as_int(&self) -> Option<(Option<i32>, bool)> {
        match self {
            MySQLType::Int(len, signed) => Some((*len, *signed)),
            _ => None,
        }
    }

    pub fn is_small_int(&self) -> bool {
        self.as_small_int().is_some()
    }

    pub fn as_small_int(&self) -> Option<(Option<i32>, bool)> {
        match self {
            MySQLType::SmallInt(len, signed) => Some((*len, *signed)),
            _ => None,
        }
    }

    pub fn is_medium_int(&self) -> bool {
        self.as_medium_int().is_some()
    }

    pub fn as_medium_int(&self) -> Option<(Option<i32>, bool)> {
        match self {
            MySQLType::MediumInt(len, signed) => Some((*len, *signed)),
            _ => None,
        }
    }

    pub fn is_big_int(&self) -> bool {
        self.as_big_int().is_some()
    }

    pub fn as_big_int(&self) -> Option<(Option<i32>, bool)> {
        match self {
            MySQLType::BigInt(len, signed) => Some((*len, *signed)),
            _ => None,
        }
    }

    pub fn is_year(&self) -> bool {
        match self {
            MySQLType::Year => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            MySQLType::Float => true,
            _ => false,
        }
    }

    pub fn is_double(&self) -> bool {
        match self {
            MySQLType::Double => true,
            _ => false,
        }
    }

    pub fn is_decimal(&self) -> bool {
        self.as_decimal().is_some()
    }

    pub fn as_decimal(&self) -> Option<(i32, i32)> {
        match self {
            MySQLType::Decimal(p, s) => Some((*p, *s)),
            _ => None,
        }
    }

    pub fn is_datetime(&self) -> bool {
        self.as_datetime().is_some()
    }

    pub fn as_datetime(&self) -> Option<i32> {
        match self {
            MySQLType::DateTime(len) => Some(*len),
            _ => None,
        }
    }

    pub fn is_date(&self) -> bool {
        match self {
            MySQLType::Date => true,
            _ => false,
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

    pub fn is_timestamp(&self) -> bool {
        self.as_timestamp().is_some()
    }

    pub fn as_timestamp(&self) -> Option<i32> {
        match self {
            MySQLType::Timestamp(len) => Some(*len),
            _ => None,
        }
    }

    pub fn is_json(&self) -> bool {
        match self {
            MySQLType::Json => true,
            _ => false,
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            MySQLType::Binary => true,
            _ => false,
        }
    }

    pub fn is_var_binary(&self) -> bool {
        match self {
            MySQLType::VarBinary => true,
            _ => false,
        }
    }

    pub fn is_tiny_blob(&self) -> bool {
        match self {
            MySQLType::TinyBlob => true,
            _ => false,
        }
    }

    pub fn is_blob(&self) -> bool {
        match self {
            MySQLType::Blob => true,
            _ => false,
        }
    }

    pub fn is_medium_blob(&self) -> bool {
        match self {
            MySQLType::MediumBlob => true,
            _ => false,
        }
    }

    pub fn is_long_blob(&self) -> bool {
        match self {
            MySQLType::LongBlob => true,
            _ => false,
        }
    }
}