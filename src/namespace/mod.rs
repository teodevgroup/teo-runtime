use std::collections::HashMap;
use crate::model::Model;
use crate::r#enum::Enum;

#[derive(Debug)]
pub struct Namespace {
    pub namespaces: HashMap<String, Namespace>,
    pub models: HashMap<String, Model>,
    pub enums: HashMap<String, Enum>,
}

impl Namespace {


}