use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;
use serde::Serialize;
use crate::r#struct::function::instance_function;
use crate::r#struct::function::instance_function::Function;
use crate::r#struct::function::static_function;
use crate::r#struct::function::static_function::StaticFunction;
use crate::utils::next_path;

#[derive(Debug, Serialize)]
pub struct Struct {
    pub path: Vec<String>,
    pub static_functions: BTreeMap<String, static_function::Definition>,
    pub functions: BTreeMap<String, instance_function::Definition>,
}

impl Struct {

    pub fn define_static_function<F>(&mut self, name: &str, f: F) where F: 'static + StaticFunction {
        self.static_functions.insert(name.to_owned(), static_function::Definition {
            path: next_path(&self.path, name),
            body: Arc::new(f),
        });
    }

    pub fn define_function<F>(&mut self, name: &str, f: F) where F: 'static + Function {
        self.functions.insert(name.to_owned(), instance_function::Definition {
            path: next_path(&self.path, name),
            body: Arc::new(f),
        });
    }

    pub fn static_function(&self, name: &str) -> Option<&static_function::Definition> {
        self.static_functions.get(name)
    }

    pub fn function(&self, name: &str) -> Option<&instance_function::Definition> {
        self.functions.get(name)
    }
}
