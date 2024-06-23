use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Comment {
    pub name: Option<String>,
    pub desc: Option<String>,
}