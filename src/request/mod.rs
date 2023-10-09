use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub mod header;

pub struct Request {
    inner: Arc<dyn r#trait::Request>
}

impl Debug for Request {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Request");
        debug_struct.field("method", &self.inner.method());
        debug_struct.field("path", &self.inner.path());
        debug_struct.field("query_string", &self.inner.query_string());
        debug_struct.field("content_type", &self.inner.content_type());
        debug_struct.finish()
    }
}

pub mod r#trait {

    pub trait Request {
        fn method(&self) -> &str;
        fn path(&self) -> &str;
        fn query_string(&self) -> &str;
        fn content_type(&self) -> &str;
        //fn headers(&self) -> ReadOnlyHeaderMap;
    }
}
