use serde::Serialize;
use crate::admin::language::Language;
use crate::config::client::ClientHost;

#[derive(Debug, Serialize, Clone)]
pub struct Admin {
    pub dest: String,
    pub host: ClientHost,
    pub languages: Vec<Language>,
}


