use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ResetDataSets {
    Auto,
    DataSets(Vec<Vec<String>>),
}

#[derive(Debug, Serialize)]
pub struct Test {
    #[serde(rename = "resetAfterQuery")]
    pub reset_after_query: bool,
    #[serde(rename = "resetAfterMutation")]
    pub reset_after_mutation: bool,
    #[serde(rename = "resetDataSets")]
    pub reset_data_sets: ResetDataSets,
}
