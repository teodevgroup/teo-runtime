use std::collections::BTreeMap;
use serde::Serialize;
use teo_parser::ast::handler::HandlerInputFormat;
use crate::handler::Handler;
use crate::handler::handler::{Method};
use crate::middleware::next::Next;
use crate::utils::next_path;

#[derive(Serialize, Debug)]
pub struct Group {
    pub path: Vec<String>,
    pub handlers: BTreeMap<String, Handler>,
}

impl Group {

    pub fn define_handler<F>(&mut self, name: &str, call: F) where F: 'static + Next {
        let handler = Handler {
            format: HandlerInputFormat::Json,
            path: next_path(&self.path, name),
            ignore_prefix: false,
            method: Method::Post,
            url: None,
            call: Box::leak(Box::new(call)),
        };
        self.handlers.insert(name.to_owned(), handler);
    }
}

