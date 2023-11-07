use serde::Serialize;
use crate::arguments::Arguments;
use crate::model::property::Property;
use teo_result::Result;

#[derive(Debug, Serialize)]
pub struct Decorator {
    pub path: Vec<String>,
    #[serde(skip)]
    pub(crate) call: fn(Arguments, &mut Property) -> Result<()>
}