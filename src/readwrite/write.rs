use serde::Serialize;
use crate::pipeline::pipeline::Pipeline;

#[derive(Debug, Serialize)]
pub enum Write {
    Write,
    NoWrite,
    WriteOnce,
    WriteOnCreate,
    WriteNonNull,
    WriteIf(Pipeline),
}

impl Write {

    pub fn is_no_write(&self) -> bool {
        match self {
            Write::NoWrite => true,
            _ => false
        }
    }
}