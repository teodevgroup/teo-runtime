use std::sync::Arc;
use educe::Educe;
use super::creator::Creator;

#[derive(Educe)]
#[educe(Debug)]
pub struct Definition {
    pub path: Vec<String>,
    #[educe(Debug(ignore))]
    pub creator: Arc<dyn Creator>,
}
