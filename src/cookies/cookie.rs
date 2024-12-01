use std::fmt::{Debug, Display, Formatter};
use std::sync::{Arc, Mutex};
use cookie::{Cookie as Inner, Expiration, SameSite};
use cookie::time::Duration;
use teo_result::Result;

#[repr(transparent)]
#[derive(Clone)]
pub struct Cookie {
    inner: Arc<Mutex<Inner<'static>>>
}

impl Cookie {

    pub fn new<T, U>(name: T, value: U) -> Self where String: From<T>, String: From<U> {
        Self {
            inner: Arc::new(Mutex::new(Inner::new(String::from(name), String::from(value))))
        }
    }

    pub fn parse(string: impl Into<String>) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(Inner::parse(string.into())?))
        })
    }

    pub fn parse_encoded(string: impl Into<String>) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(Inner::parse_encoded(string.into())?))
        })
    }

    pub fn name(&self) -> String {
        self.inner.lock().unwrap().name().to_string()
    }

    pub fn set_name(&self, name: impl Into<String>) {
        self.inner.lock().unwrap().set_name(name.into());
    }

    pub fn value(&self) -> String {
        self.inner.lock().unwrap().value().to_string()
    }

    pub fn value_trimmed(&self) -> String {
        self.inner.lock().unwrap().value_trimmed().to_string()
    }

    pub fn set_value(&self, value: impl Into<String>) {
        self.inner.lock().unwrap().set_value(value.into());
    }

    pub fn http_only(&self) -> Option<bool> {
        self.inner.lock().unwrap().http_only()
    }

    pub fn set_http_only(&self, http_only: Option<bool>) {
        self.inner.lock().unwrap().set_http_only(http_only);
    }

    pub fn secure(&self) -> Option<bool> {
        self.inner.lock().unwrap().secure()
    }

    pub fn set_secure(&self, secure: Option<bool>) {
        self.inner.lock().unwrap().set_secure(secure);
    }

    pub fn same_site(&self) -> Option<SameSite> {
        self.inner.lock().unwrap().same_site()
    }

    pub fn set_same_site(&self, same_site: Option<SameSite>) {
        self.inner.lock().unwrap().set_same_site(same_site);
    }

    pub fn partitioned(&self) -> Option<bool> {
        self.inner.lock().unwrap().partitioned()
    }

    pub fn set_partitioned(&self, partitioned: Option<bool>) {
        self.inner.lock().unwrap().set_partitioned(partitioned);
    }

    pub fn max_age(&self) -> Option<Duration> {
        self.inner.lock().unwrap().max_age()
    }

    pub fn set_max_age(&self, max_age: Option<Duration>) {
        self.inner.lock().unwrap().set_max_age(max_age);
    }

    pub fn path(&self) -> Option<String> {
        self.inner.lock().unwrap().path().map(|s| s.to_string())
    }

    pub fn set_path<T>(&self, path: Option<T>) where String: From<T> {
        let mut guard = self.inner.lock().unwrap();
        let inner = guard.as_mut();
        if let Some(value) = path {
            inner.set_path(String::from(value))
        } else {
            inner.unset_path()
        }
    }

    pub fn domain(&self) -> Option<String> {
        self.inner.lock().unwrap().domain().map(|s| s.to_string())
    }

    pub fn set_domain<T>(&self, domain: Option<T>) where String: From<T> {
        let mut guard = self.inner.lock().unwrap();
        let inner = guard.as_mut();
        if let Some(value) = domain {
            inner.set_domain(String::from(value))
        } else {
            inner.unset_domain()
        }
    }

    pub fn expires(&self) -> Option<Expiration> {
        self.inner.lock().unwrap().expires()
    }

    pub fn set_expires(&self, expires: Option<Expiration>) {
        let mut guard = self.inner.lock().unwrap();
        let inner = guard.as_mut();
        if let Some(value) = expires {
            inner.set_expires(value)
        } else {
            inner.unset_expires()
        }
    }

    pub fn make_permanent(&self) {
        self.inner.lock().unwrap().make_permanent();
    }

    pub fn make_removal(&self) {
        self.inner.lock().unwrap().make_removal();
    }
}

impl Debug for Cookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let guard = self.inner.lock().unwrap();
        let inner = guard.as_ref();
        Debug::fmt(&inner, f)
    }
}

impl Display for Cookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let guard = self.inner.lock().unwrap();
        let inner = guard.as_ref();
        Display::fmt(&inner, f)
    }
}

impl PartialEq for Cookie {
    fn eq(&self, other: &Self) -> bool {
        let guard_self = self.inner.lock().unwrap();
        let inner = guard_self.as_ref();
        let guard_other = other.inner.lock().unwrap();
        let other = guard_other.as_ref();
        inner == other
    }
}
