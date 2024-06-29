use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use crate::handler::Method;
use crate::middleware::next::Next;
use crate::traits::named::Named;

#[derive(Educe)]
#[educe(Debug)]
#[derive(Clone)]
pub struct Handler {
    pub(super) inner: Arc<Inner>
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
pub(super) struct Inner {
    pub(super) path: Vec<String>,
    pub(super) namespace_path: Vec<String>,
    pub(super) input_type: Type,
    pub(super) output_type: Type,
    pub(super) nonapi: bool,
    pub(super) format: HandlerInputFormat,
    pub(super) method: Method,
    pub(super) url: Option<String>,
    pub(super) interface: Option<String>,
    pub(super) ignore_prefix: bool,
    #[serde(skip)] #[educe(Debug(ignore))]
    pub(super) call: &'static dyn Next,
}

impl Handler {

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn namespace_path(&self) -> &Vec<String> {
        &self.inner.namespace_path
    }

    pub fn input_type(&self) -> &Type {
        &self.inner.input_type
    }

    pub fn output_type(&self) -> &Type {
        &self.inner.output_type
    }

    pub fn nonapi(&self) -> bool {
        self.inner.nonapi
    }

    pub fn format(&self) -> HandlerInputFormat {
        self.inner.format
    }

    pub fn method(&self) -> Method {
        self.inner.method
    }

    pub fn url(&self) -> Option<&String> {
        self.inner.url.as_ref()
    }

    pub fn interface(&self) -> &Option<String> {
        &self.inner.interface
    }

    pub fn ignore_prefix(&self) -> bool {
        self.inner.ignore_prefix
    }

    pub fn call(&self) -> &'static dyn Next {
        self.inner.call
    }

    pub fn has_custom_url_args(&self) -> bool {
        if self.inner.url.is_some() {
            self.inner.url.as_ref().unwrap().contains("*") || self.inner.url.as_ref().unwrap().contains(":")
        } else {
            false
        }
    }

    pub fn has_body_input(&self) -> bool {
        !(self.inner.method == Method::Get || self.inner.method == Method::Delete)
    }

    pub fn custom_url_args_path(&self) -> Option<Vec<String>> {
        if let Some(interface) = &self.inner.interface {
            let mut result = self.inner.path.clone();
            result.push(interface.clone());
            Some(result)
        } else {
            None
        }
    }
}

impl Named for Handler {

    fn name(&self) -> &str {
        self.inner.path.last().map(|s| s.as_str()).unwrap()
    }
}

impl Serialize for Handler {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}
