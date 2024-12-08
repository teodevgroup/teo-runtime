use std::sync::{Arc, Mutex};
use crate::cookies::cookie::Cookie;
use crate::headers::Headers;
use teo_result::{Result, Error};

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

    pub fn from_response_headers(headers: &Headers) -> Result<Self> {
        let mut cookies: Vec<Cookie> = Vec::new();
        for cookie_header_value in headers.get_all("set-cookie")? {
            let cookie_full_str = cookie_header_value.as_str();
            for cookie_str in cookie_full_str.split(';').map(|s| s.trim()) {
                if !cookie_str.is_empty() {
                    cookies.push(match Cookie::parse_encoded(cookie_str) {
                        Ok(cookie) => cookie,
                        Err(_) => return Err(Error::invalid_request_message(format!("invalid cookie format: `{}`", cookie_str))),
                    });
                }
            }
        }
        Ok(Cookies::from(cookies))

    }

    pub fn from_request_headers(headers: &Headers) -> Result<Self> {
        let mut cookies: Vec<Cookie> = Vec::new();
        for cookie_header_value in headers.get_all("cookie")? {
            let cookie_full_str = cookie_header_value.as_str();
            for cookie_str in cookie_full_str.split(';').map(|s| s.trim()) {
                if !cookie_str.is_empty() {
                    cookies.push(match Cookie::parse_encoded(cookie_str) {
                        Ok(cookie) => cookie,
                        Err(_) => return Err(Error::invalid_request_message(format!("invalid cookie format: `{}`", cookie_str))),
                    });
                }
            }
        }
        Ok(Cookies::from(cookies))
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

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().list.len()
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