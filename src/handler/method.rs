use serde::Serialize;

#[derive(Debug, Serialize, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Options,
}

impl Method {
    pub fn capitalized_name(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Patch => "PATCH",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Options => "OPTIONS",
        }
    }
}
