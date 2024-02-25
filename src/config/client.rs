use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ClientLanguage {
    JavaScript,
    TypeScript,
    Swift,
    Kotlin,
    CSharp,
    Dart,
}

#[derive(Debug, Serialize)]
pub enum ClientHost {
    String(String),
    Inject(String),
}

impl ClientHost {
    pub fn to_host_string(&self) -> String {
        match self {
            Self::Inject(v) => v.clone(),
            Self::String(s) => {
                let appended = if s.ends_with("/") {
                    s.clone()
                } else {
                    s.to_owned() + "/"
                };
                format!("\"{appended}\"")
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Client {
    pub provider: ClientLanguage,
    pub dest: String,
    pub package: bool,
    pub host: ClientHost,
    #[serde(rename = "objectName")]
    pub object_name: String,
    #[serde(rename = "gitCommit")]
    pub git_commit: bool,
}


