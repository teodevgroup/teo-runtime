use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use crate::app::data::AppData;
use super::creator::Creator;

#[derive(Debug, Clone)]
pub struct Definition {
    inner: Arc<Inner>,
}

#[derive(Educe, Serialize)]
#[educe(Debug)]
struct Inner {
    path: Vec<String>,
    #[educe(Debug(ignore))] #[serde(skip)]
    creator: Arc<dyn Creator>,
    #[serde(skip)]
    app_data: AppData,
}

impl Definition {

    pub fn new(path: Vec<String>, creator: Arc<dyn Creator>, app_data: AppData) -> Self {
        Self {
            inner: Arc::new(Inner {
                path,
                creator,
                app_data,
            }),
        }
    }

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn creator(&self) -> Arc<dyn Creator> {
        self.inner.creator.clone()
    }
}

impl Serialize for Definition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}