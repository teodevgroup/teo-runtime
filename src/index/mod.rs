use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum Type {
    Primary,
    Index,
    Unique,
}

impl Type {

    pub(crate) fn is_primary(&self) -> bool {
        match self {
            Type::Primary => true,
            _ => false,
        }
    }

    pub(crate) fn is_unique_or_primary(&self) -> bool {
        match self {
            Type::Unique | Type::Primary => true,
            _ => false,
        }
    }

    pub(crate) fn is_unique(&self) -> bool {
        match self {
            Type::Unique => true,
            _ => false,
        }
    }

    pub(crate) fn is_index(&self) -> bool {
        match self {
            Type::Index => true,
            _ => false,
        }
    }
}