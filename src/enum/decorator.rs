use crate::arguments::Arguments;
use crate::r#enum::Enum;
use crate::result::Result;

#[derive(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    pub(crate) call: fn(Arguments, &mut Enum) -> Result<()>
}
