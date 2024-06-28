use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Server {
    pub bind: (String, i32),
    #[serde(rename = "pathPrefix")]
    pub path_prefix: Option<String>,
}
