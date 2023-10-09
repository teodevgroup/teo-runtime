use crate::arguments::Arguments;
use crate::pipeline::ctx::Ctx;
use crate::result::Result;

#[derive(Debug, Clone)]
pub struct Item {
    pub path: Vec<String>,
    pub(crate) call: for<'a> fn(&Arguments, Ctx<'a>) -> Result<Ctx<'a>>,
}
