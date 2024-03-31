use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum TypeScriptHTTPProvider {
    Fetch,
    Taro,
    WeChat,
}

impl TypeScriptHTTPProvider {
    pub fn is_fetch(&self) -> bool {
        match self {
            Self::Fetch => true,
            _ => false,
        }
    }

    pub fn is_taro(&self) -> bool {
        match self {
            Self::Taro => true,
            _ => false,
        }
    }

    pub fn is_wechat(&self) -> bool {
        match self {
            Self::WeChat => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ClientLanguage {
    TypeScript(TypeScriptHTTPProvider),
    Swift,
    Kotlin,
    CSharp,
    Dart,
}

impl ClientLanguage {

    pub fn ts_http_provider(&self) -> Option<&TypeScriptHTTPProvider> {
        match self {
            ClientLanguage::TypeScript(v) => Some(v),
            _ => None,
        }
    }
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
pub struct Admin {
    pub dest: String,
    pub host: String,
}


