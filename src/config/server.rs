use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Server {
    pub bind: (String, i32),
    #[serde(rename = "pathPrefix")]
    pub path_prefix: Option<String>,
}
