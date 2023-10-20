use serde::Serialize;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub enum Optionality {
    Optional,
    Required,
    PresentWith(Vec<String>),
    PresentWithout(Vec<String>),
    PresentIf(Pipeline),
}

impl Optionality {

    pub(crate) fn is_any_optional(&self) -> bool {
        match self {
            Optionality::Required => false,
            _ => true,
        }
    }

    pub(crate) fn is_required(&self) -> bool {
        match self {
            Optionality::Required => true,
            _ => false
        }
    }
}
