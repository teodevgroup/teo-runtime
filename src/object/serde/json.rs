use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use crate::object::{Object, ObjectInner};

impl Serialize for Object {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self.inner.as_ref() {
            ObjectInner::Teon(teon) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$teon", teon)?;
                map.end()
            },
            ObjectInner::ModelObject(model_object) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$modelObject", model_object)?;
                map.end()
            },
            ObjectInner::StructObject(struct_object) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$structObject", struct_object)?;
                map.end()
            },
            ObjectInner::Pipeline(pipeline) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$pipeline", pipeline)?;
                map.end()
            }
            ObjectInner::InterfaceEnumVariant(interface_enum_variant) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("$interfaceEnumVariant", interface_enum_variant)?;
                map.end()
            }
        }
    }
}