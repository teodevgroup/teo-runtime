use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::arguments::Arguments;
use crate::middleware::creator::Creator;

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize)]
pub struct Use {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub creator: Arc<dyn Creator>,
    pub arguments: Arguments,
}