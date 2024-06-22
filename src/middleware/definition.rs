use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use super::creator::Creator;

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct Definition {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub creator: Arc<dyn Creator>,
}
