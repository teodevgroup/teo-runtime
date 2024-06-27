use std::collections::BTreeMap;
use std::sync::Arc;
use indexmap::IndexMap;
use maplit::btreemap;
use serde::Serialize;
use teo_parser::r#type::reference::Reference;
use teo_parser::r#type::synthesized_shape::SynthesizedShape;
use teo_parser::r#type::Type;
use crate::comment::Comment;
use crate::interface::field::Field;
use crate::traits::documentable::Documentable;
use crate::traits::named::Named;
use crate::Value;

#[derive(Debug, Clone)]
pub struct Interface {
    pub(super) inner: Arc<Inner>
}

#[derive(Debug, Serialize)]
pub(super) struct Inner {
    pub(super) path: Vec<String>,
    pub(super) parser_path: Vec<usize>,
    pub(super) comment: Option<Comment>,
    pub(super) fields: IndexMap<String, Field>,
    pub(super) generic_names: Vec<String>,
    pub(super) extends: Vec<Type>,
    pub(super) shape: SynthesizedShape,
    pub(super) generate_client: bool,
    pub(super) generate_entity: bool,
    pub(super) data: BTreeMap<String, Value>,
}

impl Interface {

    pub fn path(&self) -> &Vec<String> {
        &self.inner.path
    }

    pub fn parser_path(&self) -> &Vec<usize> {
        &self.inner.parser_path
    }

    pub fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    pub fn fields(&self) -> &IndexMap<String, Field> {
        &self.inner.fields
    }

    pub fn generic_names(&self) -> &Vec<String> {
        &self.inner.generic_names
    }

    pub fn extends(&self) -> &Vec<Type> {
        &self.inner.extends
    }

    pub fn shape(&self) -> &SynthesizedShape {
        &self.inner.shape
    }

    pub fn generate_client(&self) -> bool {
        self.inner.generate_client
    }

    pub fn generate_entity(&self) -> bool {
        self.inner.generate_entity
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.inner.data
    }

    pub fn shape_from_generics(&self, generics: &Vec<Type>) -> SynthesizedShape {
        let map = self.calculate_generics_map(generics);
        self.inner.shape.replace_generics(&map)
    }

    pub fn calculate_generics_map(&self, types: &Vec<Type>) -> BTreeMap<String, Type> {
        if self.generic_names().len() == types.len() {
            return self.generic_names().iter().enumerate().map(|(index, name)| (name.to_string(), types.get(index).unwrap().clone())).collect();
        }
        btreemap!{}
    }

    pub fn as_type_reference(&self) -> Reference {
        Reference::new(self.inner.parser_path.clone(), self.inner.path.clone())
    }
}

impl Named for Interface {

    fn name(&self) -> &str {
        self.inner.path.last().map(|s| s.as_str()).unwrap()
    }
}

impl Documentable for Interface {

    fn comment(&self) -> Option<&Comment> {
        self.inner.comment.as_ref()
    }

    fn kind(&self) -> &'static str {
        "interface"
    }
}

impl Serialize for Interface {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}