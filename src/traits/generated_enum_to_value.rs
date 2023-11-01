use teo_teon::Value;
use teo_result::Result;

pub trait GeneratedEnumToValue {

    fn to_value(&self) -> Value;
}

pub trait ValueToGeneratedEnum<T> {

    fn to_enum_variant(&self) -> Result<T>;
}