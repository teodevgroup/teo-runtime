use serde::Serialize;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub enum Optionality {
    Required,
    Optional,
    PresentWith(Vec<String>),
    PresentWithout(Vec<String>),
    PresentIf(Pipeline),
}

impl Optionality {

    pub fn is_any_optional(&self) -> bool {
        match self {
            Optionality::Required => false,
            _ => true,
        }
    }

    pub fn is_required(&self) -> bool {
        match self {
            Optionality::Required => true,
            _ => false
        }
    }
}
