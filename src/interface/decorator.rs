use crate::arguments::Arguments;
use crate::interface::Interface;
use teo_result::Result;

#[derive(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    pub(crate) call: fn(Arguments, &mut Interface) -> Result<()>
}
