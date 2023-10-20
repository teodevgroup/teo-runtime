use serde::Serialize;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub enum WriteRule {
    Write,
    NoWrite,
    WriteOnce,
    WriteOnCreate,
    WriteNonNull,
    WriteIf(Pipeline),
}

impl WriteRule {

    pub fn is_no_write(&self) -> bool {
        match self {
            WriteRule::NoWrite => true,
            _ => false
        }
    }
}