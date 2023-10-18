use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Interface {
    path: Vec<String>,
}
