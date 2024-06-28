use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use crate::handler::ctx_argument::HandlerCtxArgument;
use crate::handler::{Group, Handler, handler};
use crate::handler::group::group;
use crate::handler::Method;
use crate::request;
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

}

impl Builder {

    pub fn new(path: Vec<String>) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                handlers: Arc::new(Mutex::new(btreemap! {}))
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

    pub fn define_handler<T, F>(&self, name: &str, call: F) where T: 'static, F: 'static + HandlerCtxArgument<T> {
        let wrapped_call = Box::leak(Box::new(call));
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
                method: Method::Post,
                interface: None,
                url: None,
                call: Box::leak(Box::new(|ctx: request::Ctx| async {
                    wrapped_call.call(ctx).await
                })),
            })
        };
        let mut handlers = self.inner.handlers.lock().unwrap();
        handlers.insert(name.to_owned(), handler);
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
