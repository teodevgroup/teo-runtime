use serde::Serialize;
use crate::arguments::Arguments;
use crate::r#enum::member::Member;
use teo_result::Result;

#[derive(Debug, Serialize)]
pub struct Decorator {
    pub path: Vec<String>,
    #[serde(skip)]
    pub(crate) call: fn(Arguments, &mut Member) -> Result<()>
}
