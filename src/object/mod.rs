use crate::value;
use crate::model;
use crate::r#struct;

#[derive(Debug)]
pub enum Object {
    ArcTeon(value::Value),
    ModelObject(model::Object),
    StructObject(r#struct::Object),
}

impl Object {

    pub fn is_arc_teon(&self) -> bool {
        self.as_arc_teon().is_some()
    }

    pub fn as_arc_teon(&self) -> Option<&value::Value> {
        match self {
            Object::ArcTeon(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_model_object(&self) -> bool {
        self.as_model_object().is_some()
    }

    pub fn as_model_object(&self) -> Option<&model::Object> {
        match self {
            Object::ModelObject(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_struct_object(&self) -> bool {
        self.as_struct_object().is_some()
    }

    pub fn as_struct_object(&self) -> Option<&r#struct::Object> {
        match self {
            Object::StructObject(v) => Some(v),
            _ => None,
        }
    }
}