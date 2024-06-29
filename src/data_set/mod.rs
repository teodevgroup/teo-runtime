use crate::value::Value;

#[derive(Debug, Clone)]
pub struct DataSet {
    pub notrack: bool,
    pub autoseed: bool,
    pub name: Vec<String>,
    pub groups: Vec<Group>
}

#[derive(Debug, Clone)]
pub struct Group {
    pub name: Vec<String>,
    pub records: Vec<Record>,
}

impl Group {
    pub fn model_path(&self) -> &Vec<String> {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub name: String,
    pub value: Value,
}