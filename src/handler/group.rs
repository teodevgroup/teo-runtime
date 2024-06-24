use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use serde::Serialize;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_parser::r#type::Type;
use crate::handler::ctx_argument::HandlerCtxArgument;
use crate::handler::Handler;
use crate::handler::handler::Method;
use crate::request;
use crate::traits::named::Named;
use crate::utils::next_path;

#[derive(Serialize, Debug, Clone)]
pub struct Group {
    inner: Arc<GroupInner>
}

#[derive(Serialize, Debug)]
struct GroupInner {
    pub path: Vec<String>,
    pub(crate) handlers: Arc<Mutex<BTreeMap<String, Handler>>>,

}

impl Group {

    pub fn new(path: Vec<String>) -> Self {
        Self {
            inner: Arc::new(GroupInner {
                path,
                handlers: Arc::new(Mutex::new(btreemap! {}))
            })
        }
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
        };
        let mut handlers = self.inner.handlers.lock().unwrap();
        handlers.insert(name.to_owned(), handler);
    }
}

impl Named for Group {
    fn name(&self) -> &str {
        self.inner.path.last().unwrap().as_str()
    }
}
