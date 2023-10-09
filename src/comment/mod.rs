use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Comment {
    name: Option<String>,
    desc: Option<String>,
}