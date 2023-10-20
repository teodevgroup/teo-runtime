use std::fmt::{Display, Formatter};
use serde::Serialize;
use crate::pipeline::item::BoundedItem;

#[derive(Debug, Serialize, Clone)]
pub struct Pipeline {
    pub items: Vec<BoundedItem>
}

impl Pipeline {

    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

impl Display for Pipeline {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, item) in self.items.iter().enumerate() {
            if index == 0 {
                f.write_str("$")?;
            } else {
                f.write_str(".")?;
            }
            f.write_str(&item.path.join("."))?;
            if !item.arguments.is_empty() {
                f.write_str("(")?;
                //f.write_str(&item.arguments.iter().map(|(k, v)| format!("{k}: {}", v)).join(", "))?;
                f.write_str(")")?;
            }
        }
        Ok(())
    }
}