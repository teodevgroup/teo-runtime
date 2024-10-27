use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Server {
    pub bind: (String, u16),
    #[serde(rename = "pathPrefix")]
    pub path_prefix: Option<String>,
}
