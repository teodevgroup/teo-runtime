use crate::arguments::Arguments;
use crate::model::field::Field;
use crate::result::Result;

#[derive(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    pub(crate) call: fn(&Arguments, &mut Field) -> Result<()>
}