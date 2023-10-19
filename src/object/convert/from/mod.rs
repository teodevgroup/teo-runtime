pub mod str;
pub mod string;
pub mod bool;
pub mod i32;
pub mod i64;
pub mod f32;
pub mod f64;
pub mod decimal;
pub mod object_id;
pub mod date;
pub mod datetime;
pub mod usize;
pub mod index_map;
pub mod vec;
pub mod value;

pub mod struct_object;
pub mod model_object;
pub mod pipeline;

use teo_result::Error;
use crate::object::Object;

impl TryFrom<&Object> for Object {

    type Error = Error;

    fn try_from(value: &Object) -> std::result::Result<Self, Self::Error> {
        Ok(value.clone())
    }
}