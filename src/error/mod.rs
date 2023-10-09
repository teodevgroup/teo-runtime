use std::fmt::{Debug, Display, Formatter};

pub struct Error {
    pub message: String,
}

impl Error {

    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    pub fn prefix(&self, prefix: impl AsRef<str>) -> Self {
        Self::new(format!("{}: {}", prefix.as_ref(), self.message))
    }
}

impl Display for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Debug for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}