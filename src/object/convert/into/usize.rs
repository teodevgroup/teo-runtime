use crate::object::Object;
use teo_result::Error;
use teo_teon::Value;

impl<'a> TryFrom<&'a Object> for usize {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.to_usize() {
            Some(v) => Ok(v),
            None => Err(Error::new(format!("object cannot convert to usize: {:?}", value)))
        }
    }
}