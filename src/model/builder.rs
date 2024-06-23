use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use indexmap::IndexMap;
use crate::action::Action;
use crate::comment::Comment;
use crate::model::{Field, Index, Migration, Property, Relation};
use crate::model::model::Cache;
use crate::pipeline::Pipeline;
use crate::Value;

pub struct Builder {
    pub inner: Arc<BuilderInner>,
}

pub struct BuilderInner {
    pub path: Vec<String>,
    pub parser_path: Vec<usize>,
    pub comment: Option<Comment>,
    #[serde(rename = "tableName")]
    pub table_name: Arc<Mutex<String>>,
    pub actions: Arc<Mutex<Vec<Action>>>,
    #[serde(rename = "generateClient")]
    pub generate_client: bool,
    #[serde(rename = "generateEntity")]
    pub generate_entity: bool,
    #[serde(rename = "showInStudio")]
    pub show_in_studio: bool,
    #[serde(rename = "synthesizeShapes")]
    pub synthesize_shapes: bool,
    pub fields: IndexMap<String, Field>,
    pub relations: IndexMap<String, Relation>,
    pub properties: IndexMap<String, Property>,
    pub indexes: IndexMap<String, Index>,
    #[serde(rename = "primaryIndex")]
    pub primary_index: String,
    #[serde(rename = "beforeSave")]
    pub before_save: Pipeline,
    #[serde(rename = "afterSave")]
    pub after_save: Pipeline,
    #[serde(rename = "beforeDelete")]
    pub before_delete: Pipeline,
    #[serde(rename = "afterDelete")]
    pub after_delete: Pipeline,
    #[serde(rename = "canRead")]
    pub can_read: Pipeline,
    #[serde(rename = "canMutate")]
    pub can_mutate: Pipeline,
    pub migration: Migration,
    pub data: BTreeMap<String, Value>,
    pub cache: Cache,
}

impl Builder {
    pub fn new(path: Vec<String>, parser_path: Vec<usize>, comment: Option<Comment>) -> Self {
        let table_name = path.last().unwrap().to_string();
        Self {
            inner: Arc::new(BuilderInner {
                path,
                parser_path,
                comment,
                table_name: Arc::new(Mutex::new(table_name)),
                actions: vec![],

            })
        }
    }
}