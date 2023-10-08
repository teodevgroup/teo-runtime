pub(crate) fn next_path(path: &Vec<String>, name: &str) -> Vec<String> {
    let mut new_path = path.clone();
    new_path.push(name.to_string());
    new_path
}