use serde::Serialize;
use crate::config::client::ClientHost;

#[derive(Debug, Serialize)]
pub struct Admin {
    pub dest: String,
    pub host: ClientHost,
}


