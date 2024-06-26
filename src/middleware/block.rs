use serde::Serialize;
use crate::middleware::Use;

#[derive(Debug, Serialize)]
pub struct Block {
    pub uses: Vec<Use>,
}
