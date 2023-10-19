use crate::arguments::Arguments;
use crate::model::Model;
use teo_result::Result;

#[derive(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    pub(crate) call: fn(Arguments, &mut Model) -> Result<()>
}
