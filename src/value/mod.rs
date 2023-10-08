use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug)]
pub struct Value {
    inner: Arc<teo_teon::Value>
}

impl Value {

}

impl Display for Value {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.inner.as_ref(), f)
    }
}

impl From<teo_teon::Value> for Value {

    fn from(value: teo_teon::Value) -> Self {
        Self { inner: Arc::new(value) }
    }
}
