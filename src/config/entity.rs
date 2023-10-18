use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Runtime {
    Rust,
    Node,
    Python
}

#[derive(Debug, Serialize)]
pub struct Entity {
    pub provider: Runtime,
    pub dest: String,
}
