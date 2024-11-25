use std::sync::Arc;
use educe::Educe;
use serde::Serialize;
use teo_parser::r#type::Type;
use crate::arguments::Arguments;
use crate::pipeline::Ctx;
use crate::pipeline::item::templates::call::Call;
use crate::Value;

#[derive(Educe, Serialize, Clone)]
#[educe(Debug)]
pub struct BoundedItem {
    pub path: Vec<String>,
    pub arguments: Arguments,
    #[educe(Debug(ignore))] #[serde(skip)]
    pub(crate) call: Arc<dyn Call>,
    pub(crate) cast_output_type: Option<Type>,
}

impl BoundedItem {

    pub(crate) async fn call(&self, ctx: Ctx) -> teo_result::Result<Value> {
        self.call.call(ctx).await
    }
}