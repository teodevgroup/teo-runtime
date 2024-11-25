use std::sync::Arc;
use crate::pipeline::item::Call;

pub struct ItemImpl {
    pub item_call: Arc<dyn Call>,
}

impl ItemImpl {

    pub fn new<F>(f: F) -> Self where F: Call + 'static {
        Self {
            item_call: Arc::new(f),
        }
    }
}