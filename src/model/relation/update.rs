use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum Update {
    Default,
    Update,
    Delete,
}
