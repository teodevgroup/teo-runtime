use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use hyper::header::HeaderValue;
use hyper::HeaderMap;
use hyper::http::HeaderName;
use teo_result::Result;

#[repr(transparent)]
#[derive(Clone)]
pub struct Headers {
    inner: Arc<Mutex<Inner>>
}

#[repr(transparent)]
struct Inner {
    map: HeaderMap<HeaderValue>,
}

impl Headers {

    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                map: HeaderMap::new()
            }))
        }
    }

    pub fn keys(&self) -> Vec<String> {
        let guard = self.inner.lock().unwrap();
        guard.map.keys().map(|k| k.to_string()).collect()
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().map.len()
    }

    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        let guard = self.inner.lock().unwrap();
        guard.map.contains_key(key.as_ref())
    }

    pub fn insert(&self, key: impl Into<String>, value: impl Into<String>) -> Result<()> {
        let mut guard = self.inner.lock()?;
        let value_string = value.into();
        let header_name = HeaderName::from_str(key.into().as_str())?;
        guard.map.insert(header_name.to_owned(), HeaderValue::from_str(value_string.as_str())?);
        Ok(())
    }

    pub fn append(&self, key: impl Into<String>, value: impl Into<String>) -> Result<()> {
        let mut guard = self.inner.lock()?;
        let value_string = value.into();
        let header_name = HeaderName::from_str(key.into().as_str())?;
        guard.map.append(header_name.to_owned(), HeaderValue::from_str(value_string.as_str())?);
        Ok(())
    }

    pub fn get(&self, key: impl AsRef<str>) -> Result<Option<String>> {
        let guard = self.inner.lock()?;
        guard.map.get(key.as_ref()).map_or(Ok(None), |s| Ok(Some(s.to_str()?.to_string())))
    }

    pub fn get_all(&self, key: impl AsRef<str>) -> Result<Vec<String>> {
        let guard = self.inner.lock()?;
        Ok(guard.map.get_all(key.as_ref()).iter().map(|s| Ok(s.to_str()?.to_string())).collect::<Result<Vec<String>>>()?)
    }

    pub fn remove(&self, key: impl AsRef<str>) {
        let mut guard = self.inner.lock().unwrap();
        guard.map.remove(key.as_ref());
    }

    pub fn clear(&self) {
        let mut guard = self.inner.lock().unwrap();
        guard.map.clear();
    }

    pub fn extend_to(&self, map: &mut HeaderMap<HeaderValue>) {
        let guard = self.inner.lock().unwrap();
        map.extend(guard.map.clone())
    }
}

pub struct HeadersIter {
    inner: HeaderMap<HeaderValue>,
    index: usize,
}

impl Iterator for HeadersIter {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let keys = self.inner.keys().collect::<Vec<&HeaderName>>();
        if self.index < keys.len() {
            let key = keys[self.index];
            let value = self.inner.get(key).unwrap();
            self.index += 1;
            Some((key.to_string(), value.to_str().unwrap().to_string()))
        } else {
            None
        }
    }
}

impl IntoIterator for Headers {
    type Item = (String, String);
    type IntoIter = HeadersIter;

    fn into_iter(self) -> Self::IntoIter {
        HeadersIter {
            inner: self.inner.lock().unwrap().map.clone(),
            index: 0,
        }
    }
}

impl IntoIterator for &Headers {
    type Item = (String, String);
    type IntoIter = HeadersIter;

    fn into_iter(self) -> Self::IntoIter {
        HeadersIter {
            inner: self.inner.lock().unwrap().map.clone(),
            index: 0,
        }
    }
}

impl From<HeaderMap<HeaderValue>> for Headers {
    fn from(value: HeaderMap<HeaderValue>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                map: value
            }))
        }
    }
}

impl Debug for Headers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let guard = self.inner.lock().unwrap();
        let header_map = &guard.map;
        Debug::fmt(header_map, f)
    }
}

impl PartialEq for Headers {
    fn eq(&self, other: &Self) -> bool {
        let guard_self = self.inner.lock().unwrap();
        let guard_other = other.inner.lock().unwrap();
        let self_map = &guard_self.map;
        let other_map = &guard_other.map;
        self_map == other_map
    }
}
