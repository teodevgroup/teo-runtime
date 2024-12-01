use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use crate::app::data::AppData;
use crate::handler::ctx_argument::HandlerCtxArgument;
use crate::handler::{Group, Handler, handler};
use crate::handler::group::group;
use hyper::Method;
use crate::middleware::next::Next;
use crate::request::Request;
use crate::traits::named::Named;
use crate::utils::next_path;

#[derive(Debug, Clone)]
pub struct Builder {
    inner: Arc<Inner>
}

#[derive(Debug)]
struct Inner {
    pub path: Vec<String>,
    pub(crate) handlers: Arc<Mutex<BTreeMap<String, Handler>>>,
    pub app_data: AppData,

}

impl Builder {

    pub fn new(path: Vec<String>, app_data: AppData) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                handlers: Arc::new(Mutex::new(btreemap! {})),
                app_data,
            })
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn handler(&self, name: &str) -> Option<Handler> {
        let handlers = self.inner.handlers.lock().unwrap();
        handlers.get(name).cloned()
    }

    pub fn insert_handler(&self, name: &str, handler: Handler) {
        let mut handlers = self.inner.handlers.lock().unwrap();
        handlers.insert(name.to_owned(), handler);
    }

    pub fn define_handler<T, F>(&self, name: &str, body: F) where T: 'static, F: 'static + HandlerCtxArgument<T> {
        let body = Arc::new(body);
        let handler = Handler {
            inner: Arc::new(handler::Inner {
                namespace_path: {
                    let mut result = self.inner.path.clone();
                    result.pop();
                    result
                },
                input_type: Type::Undetermined,
                output_type: Type::Undetermined,
                nonapi: false,
                format: HandlerInputFormat::Json,
                path: next_path(self.path(), name),
                ignore_prefix: false,
                method: Method::POST,
                interface: None,
                url: None,
                call: Next::new(move |request: Request| {
                    let body = body.clone();
                    async move {
                        body.call(request).await
                    }
                }),
            })
        };
        let mut handlers = self.inner.handlers.lock().unwrap();
        handlers.insert(name.to_owned(), handler);
    }

    pub fn app_data(&self) -> &AppData {
        &self.inner.app_data
    }

    pub fn build(self) -> Group {
        Group {
            inner: Arc::new(group::Inner {
                path: self.path().clone(),
                handlers: self.inner.handlers.lock().unwrap().clone(),
            })
        }
    }
}

impl Named for Builder {
    fn name(&self) -> &str {
        self.inner.path.last().unwrap().as_str()
    }
}
