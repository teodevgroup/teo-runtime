use teo_parser::r#type::synthesized_shape_reference::SynthesizedShapeReference;
use teo_parser::r#type::Type;
use crate::namespace::Namespace;

pub trait SynthesizedShapeReferenceExtension {
    fn fetch_synthesized_definition_for_namespace<'a>(&'a self, namespace: &'a Namespace) -> Option<&'a Type>;
}

impl SynthesizedShapeReferenceExtension for SynthesizedShapeReference {

    fn fetch_synthesized_definition_for_namespace<'a>(&'a self, namespace: &'a Namespace) -> Option<&'a Type> {
        let model = namespace.model_at_path(&self.owner.as_model_object().unwrap().str_path()).unwrap();
        model.cache().shape.shapes.get(&(self.kind, self.without.clone()))
    }
}

