use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use educe::Educe;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use crate::app::data::AppData;
use crate::handler::{Handler, handler};
use hyper::Method;
use crate::middleware::next::Next;

#[derive(Debug, Clone)]
pub struct Builder {
    inner: Arc<Inner>
}

#[derive(Educe)]
#[educe(Debug)]
struct Inner {
    path: Vec<String>,
    namespace_path: Vec<String>,
    input_type: Type,
    output_type: Type,
    nonapi: bool,
    format: Arc<Mutex<HandlerInputFormat>>,
    method: Arc<Mutex<Method>>,
    url: Arc<Mutex<Option<String>>>,
    interface: Arc<Mutex<Option<String>>>,
    ignore_prefix: AtomicBool,
    #[educe(Debug(ignore))]
    call: Next,
    app_data: AppData,
}

impl Builder {

    pub fn new(path: Vec<String>, namespace_path: Vec<String>, input_type: Type, output_type: Type, nonapi: bool, format: HandlerInputFormat, call: Next, app_data: AppData) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                namespace_path,
                input_type,
                output_type,
                nonapi,
                format: Arc::new(Mutex::new(format)),
                method: Arc::new(Mutex::new(Method::POST)),
                url: Arc::new(Mutex::new(None)),
                interface: Arc::new(Mutex::new(None)),
                ignore_prefix: AtomicBool::new(false),
                call,
                app_data
            })
        }
    }

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
        *self.inner.format.lock().unwrap()
    }

    pub fn set_format(&self, format: HandlerInputFormat) {
        *self.inner.format.lock().unwrap() = format
    }

    pub fn method(&self) -> Method {
        self.inner.method.lock().unwrap().clone()
    }

    pub fn set_method(&self, method: Method) {
        *self.inner.method.lock().unwrap() = method;
    }

    pub fn url(&self) -> Option<String> {
        self.inner.url.lock().unwrap().clone()
    }

    pub fn set_url(&self, url: Option<String>) {
        *self.inner.url.lock().unwrap() = url;
    }

    pub fn interface(&self) -> Option<String> {
        self.inner.interface.lock().unwrap().clone()
    }

    pub fn set_interface(&self, interface: Option<String>) {
        *self.inner.interface.lock().unwrap() = interface;
    }

    pub fn ignore_prefix(&self) -> bool {
        self.inner.ignore_prefix.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_ignore_prefix(&self, ignore_prefix: bool) {
        self.inner.ignore_prefix.store(ignore_prefix, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn call(&self) -> Next {
        self.inner.call.clone()
    }

    pub fn app_data(&self) -> &AppData {
        &self.inner.app_data
    }

    pub(crate) fn build(self) -> Handler {
        Handler {
            inner: Arc::new(handler::Inner {
                path: self.inner.path.clone(),
                namespace_path: self.inner.namespace_path.clone(),
                input_type: self.inner.input_type.clone(),
                output_type: self.inner.output_type.clone(),
                nonapi: self.inner.nonapi,
                format: self.inner.format.lock().unwrap().clone(),
                method: self.inner.method.lock().unwrap().clone(),
                url: self.inner.url.lock().unwrap().clone(),
                interface: self.inner.interface.lock().unwrap().clone(),
                ignore_prefix: self.inner.ignore_prefix.load(std::sync::atomic::Ordering::Relaxed),
                call: self.inner.call.clone(),
            })
        }
    }
}
