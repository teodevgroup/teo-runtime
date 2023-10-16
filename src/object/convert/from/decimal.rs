use std::sync::Arc;
use bigdecimal::BigDecimal;
use teo_teon::Value;
use crate::object::{Object, ObjectInner};

impl From<BigDecimal> for Object {

    fn from(value: BigDecimal) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Decimal(value)))
        }
    }
}

impl From<&BigDecimal> for Object {

    fn from(value: &BigDecimal) -> Self {
        Object {
            inner: Arc::new(ObjectInner::Teon(Value::Decimal(value.clone())))
        }
    }
}