use std::path::PathBuf;
use std::sync::Arc;

use crate::value::Value;

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

    pub fn teon(content: Value) -> Self {
        Self {
            inner: Arc::new(BodyInner::Teon(content))
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.inner.as_ref() {
            BodyInner::Empty => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self.inner.as_ref() {
            BodyInner::File(_) => true,
            _ => false,
        }
    }

    pub fn as_file(&self) -> Option<&PathBuf> {
        match self.inner.as_ref() {
            BodyInner::File(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        match self.inner.as_ref() {
            BodyInner::String(_) => true,
            _ => false,
        }
    }

    pub fn as_text(&self) -> Option<&String> {
        match self.inner.as_ref() {
            BodyInner::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_teon(&self) -> bool {
        match self.inner.as_ref() {
            BodyInner::Teon(_) => true,
            _ => false,
        }
    }

    pub fn as_teon(&self) -> Option<&Value> {
        match self.inner.as_ref() {
            BodyInner::Teon(v) => Some(v),
            _ => None,
        }
    }
}

pub enum BodyInner {
    Empty,
    String(String),
    File(PathBuf),
    Teon(Value),
}