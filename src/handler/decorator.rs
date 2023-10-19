use crate::arguments::Arguments;
use crate::handler::Handler;
use crate::model::Model;
use crate::result::Result;

#[derive(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    pub(crate) call: fn(Arguments, &mut Handler) -> Result<()>
}
