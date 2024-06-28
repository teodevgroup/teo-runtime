use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use teo_result::Result;
use crate::r#enum;

pub trait Call: Send + Sync {
    fn call(&self, args: Arguments, member: &r#enum::member::Builder) -> Result<()>;
}

impl<F> Call for F where
    F: Fn(Arguments, &r#enum::member::Builder) -> Result<()> + Send + Sync {
    fn call(&self, args: Arguments, member: &r#enum::member::Builder) -> Result<()> {
        self(args, member)
    }
}

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct Decorator {
    inner: Arc<Inner>,
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
struct Inner {
    path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    call: Arc<dyn Call>,
}

impl Decorator {

    pub fn new<T>(path: Vec<String>, call: T) -> Self where T: Call + 'static {
        Self {
            inner: Arc::new(Inner {
                path,
                call: Arc::new(call),
            }),
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn call(&self) -> &dyn Call {
        self.inner.call.as_ref()
    }
}

impl Serialize for Decorator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where
        S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}