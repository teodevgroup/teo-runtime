use teo_result::Result;
use crate::value::Value;
use crate::model::relation::delete::Delete;
use crate::model::relation::update::Update;
use crate::namespace;
use crate::namespace::Namespace;

pub(in crate::stdlib) fn load_model_relation_decorators(namespace: &namespace::Builder) {

    namespace.define_model_relation_decorator("relation", |arguments, relation| {
        let fields: Result<Value> = arguments.get("fields");
        let references: Result<Value> = arguments.get("references");
        let through: Result<Vec<String>> = arguments.get("through");
        let local: Result<&str> = arguments.get("local");
        let foreign: Result<&str> = arguments.get("foreign");
        let update: Option<Update> = arguments.get_optional("onUpdate")?;
        let delete: Option<Delete> = arguments.get_optional("onDelete")?;
        if fields.is_ok() {
            let fields: Vec<String> = fields.unwrap().wrap_into_vec()?;
            let references: Vec<String> = references.unwrap().wrap_into_vec()?;
            relation.set_fields(fields);
            relation.set_references(references);
        } else if through.is_ok() {
            let through = through.unwrap();
            let local = local.unwrap().to_string();
            let foreign = foreign.unwrap().to_string();
            relation.set_through(Some(through));
            relation.set_local(Some(local));
            relation.set_foreign(Some(foreign));
        }
        if let Some(update) = update {
            relation.set_update(update);
        }
        if let Some(delete) = delete {
            relation.set_delete(delete);
        }
        Ok(())
    });
}