use std::sync::Arc;

#[derive(Debug)]
pub struct Object {
    inner: Arc<ObjectInner>
}

#[derive(Debug)]
struct ObjectInner {

}