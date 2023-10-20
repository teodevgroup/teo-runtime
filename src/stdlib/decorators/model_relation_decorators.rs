use teo_result::Result;
use teo_teon::types::enum_variant::EnumVariant;
use teo_teon::Value;
use crate::namespace::Namespace;

pub(in crate::stdlib) fn load_model_relation_decorators(namespace: &mut Namespace) {

    namespace.define_model_relation_decorator("relation", |arguments, relation| {
        let fields: Result<Value> = arguments.get("fields");
        let references: Result<Value> = arguments.get("references");
        let through: Result<Vec<String>> = arguments.get("through");
        let local: Result<EnumVariant> = arguments.get("local");
        let foreign: Result<EnumVariant> = arguments.get("foreign");
        if fields.is_ok() {
            let fields: Vec<EnumVariant> = fields.unwrap().into_vec()?;
            let references: Vec<EnumVariant> = references.unwrap().into_vec()?;
            let fields: Vec<String> = fields.iter().map(|f| f.clone().into_string()).collect::<Result<Vec<_>>>()?;
            let references: Vec<String> = references.iter().map(|f| f.clone().into_string()).collect::<Result<Vec<_>>>()?;
            relation.fields = fields;
            relation.references = references;
        } else if through.is_ok() {
            let through = through.unwrap();
            let local = local.unwrap().into_string()?;
            let foreign = foreign.unwrap().into_string()?;
            relation.through = Some(through);
            relation.local = Some(local);
            relation.foreign = Some(foreign);
        }
        Ok(())
    });
}