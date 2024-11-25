use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::app::data::AppData;
use crate::pipeline::item::Creator;

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct Item {
    inner: Arc<Inner>
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
struct Inner {
    pub path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) creator: Arc<dyn Creator>,
    #[serde(skip)]
    pub app_data: AppData,
}

impl Item {

    pub fn new(path: Vec<String>, creator: Arc<dyn Creator>, app_data: AppData) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                creator,
                app_data
            })
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn creator(&self) -> Arc<dyn Creator> {
        self.inner.creator.clone()
    }

    pub fn app_data(&self) -> &AppData {
        &self.inner.app_data
    }
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}