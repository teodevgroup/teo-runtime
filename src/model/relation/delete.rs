use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum Delete {
    Nullify,
    NoAction,
    Cascade,
    Deny,
    Default,
}
