use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct HandlerMatch {
    path: Vec<String>,
    name: String,
    captures: IndexMap<String, String>,
    path_without_last: Vec<String>,
}

impl HandlerMatch {

    pub fn new(path: Vec<String>, name: String, captures: IndexMap<String, String>) -> Self {
        let mut path_without_last = path.clone();
        path_without_last.pop();
        Self {
            path,
            name,
            captures,
            path_without_last,
        }
    }

    pub fn path_without_last(&self) -> &Vec<String> {
        &self.path_without_last
    }

    pub fn group_name(&self) -> &str {
        self.path().last().unwrap()
    }

    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    pub fn handler_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn captures(&self) -> &IndexMap<String, String> {
        &self.captures
    }
}