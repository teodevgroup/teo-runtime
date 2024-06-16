use indexmap::{IndexMap, indexmap};
use regex::Regex;
use hyper::Method;
use crate::handler::r#match::HandlerMatch;

#[derive(Debug)]
pub struct Map {
    records: IndexMap<(Method, String), (Vec<String>, String)>
}

impl Map {

    pub fn new() -> Self {
        Self {
            records: indexmap!{}
        }
    }

    pub fn add_record(&mut self, namespace_path: &Vec<&str>, group_name: Option<&str>, action_name: &str, method: Method, custom_url: Option<&str>, ignore_prefix: bool) {
        let url = if ignore_prefix {
            if custom_url.unwrap().starts_with("/") {
                custom_url.unwrap().to_owned()
            } else {
                "/".to_owned() + custom_url.unwrap()
            }
        } else {
            "/".to_owned() + &namespace_path.join(".") + &if let Some(group_name) = group_name {
                "/".to_owned() + group_name
            } else { "".to_owned() } + &if let Some(custom_url) = custom_url {
                if custom_url.starts_with("/") {
                    custom_url.to_owned()
                } else {
                    "/".to_owned() + custom_url
                }
            } else {
                action_name.to_owned()
            }
        };
        let url = url.replace("//", "/");
        let mut result: Vec<String> = namespace_path.iter().map(|i| i.to_string()).collect();
        if let Some(group_name) = group_name {
            result.push(group_name.to_owned());
        }
        self.records.insert((method, url), (result, action_name.to_owned()));
    }

    pub fn r#match(&self, method: Method, url: &str) -> Option<HandlerMatch> {
        for record in &self.records {
            if let Some(result) = self.try_match(&method, url, record) {
                return Some(result);
            }
        }
        None
    }

    fn try_match(&self, method: &Method, url: &str, record: (&(Method, String), &(Vec<String>, String))) -> Option<HandlerMatch> {
        if method != Method::OPTIONS && record.0.0 != method {
            return None;
        }
        let define = &record.0.1;
        let arg_names = self.fetch_arg_names(define);
        let regex_string = self.replace_arg_names(define);
        let regex_string = format!("^{regex_string}$");
        let regex = Regex::new(&regex_string).unwrap();
        if regex.is_match(url) {
            if let Some(captures) = regex.captures(url) {
                let mut retval = indexmap!{};
                for (index, r#match) in captures.iter().enumerate() {
                    if index == 0 {
                        continue
                    }
                    if let Some(r#match) = r#match {
                        retval.insert(arg_names.get(index - 1).unwrap().to_owned(), r#match.as_str().to_owned());
                    }
                }
                return Some(HandlerMatch {
                    path: record.1.0.clone(),
                    name: record.1.1.clone(),
                    captures: retval
                })
            } else {
                return Some(HandlerMatch {
                    path: record.1.0.clone(),
                    name: record.1.1.clone(),
                    captures: indexmap!{},
                })
            }
        }
        None
    }

    fn fetch_arg_names(&self, define: &String) -> Vec<String> {
        let regex = Regex::new("[:*]([^/]+)").unwrap();
        let captures = regex.captures(define);
        if let Some(captures) = captures {
            captures.iter().map(|m| m.unwrap().as_str()[1..].to_string()).collect()
        } else {
            vec![]
        }
    }

    fn replace_arg_names(&self, define: &String) -> String {
        let normal_regex = Regex::new(":[^/]+").unwrap();
        let replaced = normal_regex.replace(define, "([^/]+)");
        let catch_regex = Regex::new("\\*[^/]+").unwrap();
        let replaced = catch_regex.replace(replaced.as_ref(), "(.+)");
        replaced.as_ref().to_string()
    }

    pub fn default_match(&self, method: Method, url: &str) -> Option<HandlerMatch> {
        if method != Method::OPTIONS && method != Method::POST {
            return None;
        }
        let mut url = url;
        if url.starts_with("/") {
            url = url.trim_start_matches("/");
        }
        if url.ends_with("/") {
            url = url.trim_end_matches("/");
        }
        let parts = url.split("/");
        if parts.clone().count() < 1 {
            return None;
        }
        let mut result: Vec<String> = parts.map(|p| p.to_string()).collect();
        let action = result.pop().unwrap().to_string();
        Some(HandlerMatch {
            path: result,
            name: action,
            captures: indexmap! {}
        })
    }

    pub fn remove_path_prefix<'a>(&self, path: &'a str, prefix: Option<&'a str>) -> &'a str {
        if let Some(prefix) = prefix {
            let mut prefix = prefix;
            prefix = prefix.trim_end_matches("/");
            let result = path.strip_prefix(prefix).unwrap();
            if result == "" {
                "/"
            } else {
                result
            }
        } else {
            path
        }
    }
}