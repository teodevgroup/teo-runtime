use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Previous {
    DontKeep,
    Keep,
}

impl Previous {

    pub(crate) fn is_keep(&self) -> bool {
        match self {
            Previous::Keep => true,
            _ => false,
        }
    }
}