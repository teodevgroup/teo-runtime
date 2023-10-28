use std::sync::Arc;

#[derive(Clone)]
pub struct HeaderMap {
    pub inner: Arc<dyn r#trait::HeaderMap>
}

impl HeaderMap {

    pub fn keys(&self) -> Vec<&str> {
        self.inner.keys()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.inner.contains_key(key.as_ref())
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
        self.inner.get(key.as_ref())
    }
}

pub mod r#trait {

    pub trait HeaderMap {

        fn keys(&self) -> Vec<&str>;

        fn len(&self) -> usize;

        fn contains_key(&self, key: &str) -> bool;

        fn get(&self, key: &str) -> Option<&str>;
    }
}
