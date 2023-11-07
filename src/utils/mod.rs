pub mod find_main_schema_file;

pub use find_main_schema_file::find_main_schema_file;

pub(crate) fn next_path(path: &Vec<String>, name: &str) -> Vec<String> {
    let mut new_path = path.clone();
    new_path.push(name.to_string());
    new_path
}

pub trait ContainsStr {

    fn contains_str(&self, str: &str) -> bool;
}

impl ContainsStr for Vec<String> {

    fn contains_str(&self, str: &str) -> bool {
        self.iter().find(|v| v.as_str() == str).is_some()
    }
}