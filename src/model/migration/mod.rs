use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Migration {
    pub renamed: Option<Vec<String>>,
    pub version: Option<String>,
    pub drop: bool,
}

impl Default for Migration {
    fn default() -> Self {
        Migration {
            renamed: None,
            version: None,
            drop: false,
        }
    }
}