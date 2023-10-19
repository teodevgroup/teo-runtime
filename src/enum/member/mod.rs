pub mod decorator;

use serde::Serialize;
use teo_teon::Value;
pub use decorator::Decorator;

#[derive(Debug, Serialize)]
pub struct Member {
    pub name: String,
    pub value: Value,
}

impl Member {

    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}