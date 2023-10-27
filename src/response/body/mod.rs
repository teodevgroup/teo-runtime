use std::path::PathBuf;

pub enum Body {
    Empty,
    String(String),
    File(PathBuf),
}