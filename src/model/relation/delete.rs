use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum Delete {
    Default,
    Nullify,
    Cascade,
    Deny,
}
