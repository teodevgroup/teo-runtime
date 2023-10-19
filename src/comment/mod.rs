use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Comment {
    pub name: Option<String>,
    pub desc: Option<String>,
}