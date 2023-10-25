#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DatabaseEnum {
    pub(crate) choices: Vec<String>,
}