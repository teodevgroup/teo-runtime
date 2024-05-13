use std::sync::Arc;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Cookie {
    pub inner: Arc<dyn r#trait::Cookie>
}

impl Cookie {

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn path(&self) -> Option<&str> {
        self.inner.path()
    }

    fn value(&self) -> &str {
        self.inner.value()
    }

    fn expires_datetime(&self) -> Option<&DateTime<Utc>> {
        self.inner.expires_datetime()
    }

    fn expires_session(&self) -> bool {
        self.inner.expires_session()
    }
    fn secure(&self) -> Option<bool> {
        self.inner.secure()
    }

    fn max_age(&self) -> Option<f64> {
        self.inner.max_age()
    }
}

pub mod r#trait {
    use chrono::{DateTime, Utc};

    pub trait Cookie: Send + Sync {
        fn name(&self) -> &str;
        fn path(&self) -> Option<&str>;
        fn value(&self) -> &str;
        fn expires_datetime(&self) -> Option<&DateTime<Utc>>;
        fn expires_session(&self) -> bool;
        fn secure(&self) -> Option<bool>;
        fn max_age(&self) -> Option<f64>;
    }
}
