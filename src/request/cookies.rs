use std::sync::Arc;
use cookie::Cookie;

#[derive(Clone)]
pub struct Cookies {
    inner: Arc<Vec<Cookie<'static>>>
}

impl Cookies {

    pub fn entries(&self) -> &Vec<Cookie<'static>> {
        &self.inner
    }

    pub fn get(&self, name: &str) -> Option<&Cookie<'static>> {
        self.inner.iter().find(|cookie| cookie.name() == name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.inner.iter().any(|cookie| cookie.name() == name)
    }
}

impl From<Vec<Cookie<'static>>> for Cookies {
    fn from(cookies: Vec<Cookie<'static>>) -> Self {
        Self {
            inner: Arc::new(cookies)
        }
    }
}