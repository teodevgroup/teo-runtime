use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct HandlerMatch {
    pub path: Vec<String>,
    pub name: String,
    pub captures: IndexMap<String, String>,
}

impl HandlerMatch {

    pub fn namespace_path(&self) -> Vec<&str> {
        self.path.iter().rev().skip(1).rev().map(AsRef::as_ref).collect()
    }

    pub fn group_name(&self) -> &str {
        self.path().last().unwrap()
    }

    pub fn path(&self) -> Vec<&str> {
        self.path.iter().map(AsRef::as_ref).collect()
    }

    pub fn handler_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn captures(&self) -> &IndexMap<String, String> {
        &self.captures
    }
}