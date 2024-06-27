use serde::Serialize;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize, Clone)]
pub enum Read {
    Read,
    NoRead,
    ReadIf(Pipeline),
}

impl Read {
    pub fn is_no_read(&self) -> bool {
        match self {
            Read::NoRead => true,
            _ => false
        }
    }
}
