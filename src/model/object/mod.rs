use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Object {
    inner: Arc<ObjectInner>
}

#[derive(Debug)]
struct ObjectInner {

}