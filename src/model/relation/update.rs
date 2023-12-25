use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum Update {
    Nullify,
    NoAction,
    Update,
    Delete,
    Deny,
    Default,
}
