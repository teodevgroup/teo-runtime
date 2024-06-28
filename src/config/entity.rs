use serde::Serialize;

#[derive(Debug, Serialize, Copy, Clone)]
pub enum Runtime {
    Rust,
    Node,
    Python
}

#[derive(Debug, Serialize, Clone)]
pub struct Entity {
    pub provider: Runtime,
    pub dest: String,
}
