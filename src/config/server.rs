use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Server {
    pub bind: (String, u16),
    #[serde(rename = "pathPrefix")]
    pub path_prefix: Option<String>,
}

impl Server {

    pub fn bind(&self) -> (&str, u16) {
        (&self.bind.0, self.bind.1)
    }

    pub fn path_prefix(&self) -> Option<&str> {
        self.path_prefix.as_deref()
    }
}