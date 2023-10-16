use std::sync::Arc;
use educe::Educe;
use serde::Serialize;

#[derive(Educe)]
#[educe(Debug)]
#[derive(Serialize)]
pub struct Handler {
    name: String,
    // #[serde(skip)] #[educe(Debug(ignore))]
    // call: Arc<dyn Call>,
}

