use std::sync::{Arc, Mutex};
use crate::cookies::cookie::Cookie;

#[derive(Clone)]
#[repr(transparent)]
pub struct Cookies {
    pub inner: Arc<Mutex<Inner>>
}

#[repr(transparent)]
pub struct Inner {
    pub list: Vec<Cookie>
}

impl Cookies {

    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                list: Vec::new()
            }))
        }
    }

    pub fn entries(&self) -> Vec<Cookie> {
        self.inner.lock().unwrap().list.clone()
    }

    pub fn set_entries(&self, cookies: Vec<Cookie>) {
        self.inner.lock().unwrap().list = cookies;
    }

    pub fn get(&self, name: &str) -> Option<Cookie> {
        self.inner.lock().unwrap().list.iter().find(|cookie| cookie.name() == name).cloned()
    }

    pub fn push(&self, cookie: Cookie) {
        self.inner.lock().unwrap().list.push(cookie);
    }

    pub fn has(&self, name: &str) -> bool {
        self.inner.lock().unwrap().list.iter().any(|cookie| cookie.name() == name)
    }

    pub fn iter(&self) -> impl Iterator<Item = Cookie> {
        self.inner.lock().unwrap().list.clone().into_iter()
    }

    pub fn clear(&self) {
        self.inner.lock().unwrap().list = vec![];
    }
}

impl From<Vec<Cookie>> for Cookies {
    fn from(cookies: Vec<Cookie>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                list: cookies
            }))
        }
    }
}