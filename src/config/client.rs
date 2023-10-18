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
pub struct Client {
    pub provider: ClientLanguage,
    pub dest: String,
    pub package: bool,
    pub host: String,
    #[serde(rename = "objectName")]
    pub object_name: String,
    #[serde(rename = "gitCommit")]
    pub git_commit: bool,
}


