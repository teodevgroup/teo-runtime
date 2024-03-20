use teo_result::Error;
use crate::value::interface_enum_variant::InterfaceEnumVariant;
use crate::value::Value;

impl<'a> TryFrom<&'a Value> for &'a InterfaceEnumVariant {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Some(v) = value.as_interface_enum_variant() {
            Ok(v)
        } else {
            Err(Error::new(format!("object is not InterfaceEnumVariant: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Value> for InterfaceEnumVariant {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Some(v) = value.as_interface_enum_variant() {
            Ok(v.clone())
        } else {
            Err(Error::new(format!("object is not InterfaceEnumVariant: {:?}", value)))
        }
    }
}

impl TryFrom<Value> for InterfaceEnumVariant {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(v) = value.as_interface_enum_variant() {
            Ok(v.clone())
        } else {
            Err(Error::new(format!("object is not InterfaceEnumVariant: {:?}", value)))
        }
    }
}

impl<'a> TryFrom<&'a Value> for Vec<InterfaceEnumVariant> {
    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        if let Some(array) = value.as_array() {
            let mut result = vec![];
            for item in array {
                result.push(item.as_interface_enum_variant().unwrap().clone());
            }
            Ok(result)
        } else {
            Err(Error::new(format!("object is not Vec<InterfaceEnumVariant>: {:?}", value)))
        }
    }
}