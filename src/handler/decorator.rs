use serde::Serialize;
use crate::arguments::Arguments;
use crate::handler::Handler;
use crate::model::Model;
use teo_result::Result;

#[derive(Debug, Serialize)]
pub struct Decorator {
    pub path: Vec<String>,
    #[serde(skip)]
    pub(crate) call: fn(Arguments, &mut Handler) -> Result<()>
}
