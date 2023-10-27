#[derive(Debug)]
pub struct HandlerMatch {
    pub path: Vec<String>,
    pub name: String,
}

impl HandlerMatch {

    pub fn path(&self) -> Vec<&str> {
        self.path.iter().map(AsRef::as_ref).collect()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}