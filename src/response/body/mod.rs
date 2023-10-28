use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct Body {
    pub inner: Arc<BodyInner>
}

impl Body {

    pub fn empty() -> Self {
        Self {
            inner: Arc::new(BodyInner::Empty)
        }
    }

    pub fn string(content: String) -> Self {
        Self {
            inner: Arc::new(BodyInner::String(content))
        }
    }

    pub fn file(content: PathBuf) -> Self {
        Self {
            inner: Arc::new(BodyInner::File(content))
        }
    }
}

pub enum BodyInner {
    Empty,
    String(String),
    File(PathBuf),
}