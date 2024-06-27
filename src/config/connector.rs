use std::sync::Arc;
use serde::Serialize;
use crate::database::database::Database;

#[derive(Debug, Clone)]
pub struct Connector {
    pub(crate) inner: Arc<Inner>,
}

#[derive(Debug, Serialize)]
struct Inner {
    provider: Database,
    url: String,
}

impl Connector {
    pub fn new(provider: Database, url: String) -> Self {
        Self {
            inner: Arc::new(Inner {
                provider,
                url,
            })
        }
    }

    pub fn provider(&self) -> Database {
        self.inner.provider
    }

    pub fn url(&self) -> &str {
        &self.inner.url
    }
}

impl Serialize for Connector {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}