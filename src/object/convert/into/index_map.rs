use indexmap::IndexMap;
use teo_teon::Value;
use teo_result::Error;
use crate::object::Object;

impl<'a> TryFrom<&'a Object> for &'a IndexMap<String, Value> {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.as_dictionary() {
            Some(v) => Ok(v),
            None => Err(Error::new(format!("object cannot convert to IndexMap<String, Value>: {:?}", value)))
        }
    }
}