pub mod decorator;

use serde::Serialize;
use teo_teon::Value;
pub use decorator::Decorator;
use crate::comment::Comment;
use crate::traits::documentable::Documentable;
use crate::traits::named::Named;

#[derive(Debug, Serialize)]
pub struct Member {
    pub name: String,
    pub comment: Option<Comment>,
    pub value: Value,
}

impl Member {

    pub fn new(name: String, value: Value, comment: Option<Comment>) -> Self {
        Self { name, value, comment }
    }
}

impl Named for Member {

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Documentable for Member {

    fn comment(&self) -> Option<&Comment> {
        self.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "enum member"
    }
}