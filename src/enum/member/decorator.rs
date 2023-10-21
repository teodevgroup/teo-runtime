use crate::arguments::Arguments;
use crate::r#enum::member::Member;
use teo_result::Result;

#[derive(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    pub(crate) call: fn(Arguments, &mut Member) -> Result<()>
}
