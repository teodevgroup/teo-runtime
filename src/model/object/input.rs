use std::collections::HashMap;
use self::Input::{SetValue, AtomicUpdater};
use teo_teon::value::Value;


pub enum Input {
    SetValue(Value),
    AtomicUpdater(Value),
}

impl Input {

    pub fn decode_field(updator: &Value) -> Input {
        if let Some(updator_map) = updator.as_dictionary() {
            let key = updator_map.keys().next().unwrap();
            let value = updator_map.values().next().unwrap();
            if key.as_str() == "set" {
                SetValue(value.clone())
            } else {
                AtomicUpdater(updator.clone())
            }
        } else {
            SetValue(updator.clone())
        }
    }

    pub fn key_value(value: &HashMap<String, Value>) -> (&str, &Value) {
        (value.keys().next().unwrap().as_str(), value.values().next().unwrap())
    }

    pub fn has_i_mode(map: &HashMap<String, Value>) -> bool {
        match map.get("mode") {
            Some(val) => {
                if let Some(str) = val.as_str() {
                    return str == "caseInsensitive"
                } else {
                    false
                }
            }
            None => {
                false
            }
        }
    }

    pub fn has_negative_take(json_value: &Value) -> bool {
        if json_value.is_dictionary() {
            let take = json_value.as_dictionary().unwrap().get("take");
            if take.is_some() {
                let take = take.unwrap();
                if take.is_any_int() {
                    let take = take.to_int64().unwrap();
                    return take < 0;
                }
            }
        }
        false
    }
}
