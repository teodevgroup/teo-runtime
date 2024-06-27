use std::sync::Arc;
use serde::Serialize;
use crate::middleware::Use;

#[derive(Debug, Clone)]
pub struct Block {
    inner: Arc<Inner>,
}

#[derive(Debug, Serialize)]
struct Inner {
    uses: Vec<Use>,
}

impl Block {

    pub fn new(uses: Vec<Use>) -> Self {
        Self {
            inner: Arc::new(Inner {
                uses,
            }),
        }
    }

    pub fn uses(&self) -> &Vec<Use> {
        &self.inner.uses
    }
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}
