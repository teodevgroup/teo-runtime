use std::path::PathBuf;
use indexmap::IndexMap;
use teo_teon::Value;

pub enum Response {
    EmptyRes,
    TeonRes(Value),
    ErrorRes { code: u16, kind: String, message: String, fields: Option<IndexMap<String, String>> },
    File(PathBuf),
}