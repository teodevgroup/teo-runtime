use serde::Serialize;
use crate::handler::Handler;
use crate::handler::handler::{Call, Method};
use crate::utils::next_path;

#[derive(Serialize, Debug)]
pub struct Group {
    pub path: Vec<String>,
    pub handlers: Vec<Handler>,
}

impl Group {

    pub fn define_handler<F>(&mut self, name: &str, call: F) -> Handler where F: 'static + Call {
        Handler {
            path: next_path(&self.path, name),
            ignore_namespace: false,
            method: Method::Post,
            url: None,
            call: Box::leak(Box::new(call)),
        }
    }
}

