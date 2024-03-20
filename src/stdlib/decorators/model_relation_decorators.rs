use teo_result::Result;
use crate::value::Value;
use crate::model::relation::delete::Delete;
use crate::model::relation::update::Update;
use crate::namespace::Namespace;

pub(in crate::stdlib) fn load_model_relation_decorators(namespace: &mut Namespace) {

    namespace.define_model_relation_decorator("relation", |arguments, relation| {
        let fields: Result<Value> = arguments.get("fields");
        let references: Result<Value> = arguments.get("references");
        let through: Result<Vec<String>> = arguments.get("through");
        let local: Result<&str> = arguments.get("local");
        let foreign: Result<&str> = arguments.get("foreign");
        let update: Option<Update> = arguments.get_optional("onUpdate")?;
        let delete: Option<Delete> = arguments.get_optional("onDelete")?;
        if fields.is_ok() {
            let fields: Vec<&str> = fields.unwrap().into_vec()?;
            let references: Vec<&str> = references.unwrap().into_vec()?;
            let fields: Vec<String> = fields.iter().map(|f| f.clone().into_string()).collect::<Vec<_>>();
            let references: Vec<String> = references.iter().map(|f| f.clone().into_string()).collect::<Vec<_>>();
            relation.fields = fields;
            relation.references = references;
        } else if through.is_ok() {
            let through = through.unwrap();
            let local = local.unwrap().into_string();
            let foreign = foreign.unwrap().into_string();
            relation.through = Some(through);
            relation.local = Some(local);
            relation.foreign = Some(foreign);
        }
        if let Some(update) = update {
            relation.update = update;
        }
        if let Some(delete) = delete {
            relation.delete = delete;
        }
        Ok(())
    });
}