use teo_parser::ast::config::Config;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::config::debug::Debug;
use crate::config::entity::{Entity, Runtime};
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_entity(dest_namespace: &mut Namespace, schema: &Schema, entity: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("entity", entity.availability()).unwrap();
    let provider_expect = config_decl.get_field("provider").unwrap().type_expr.resolved();
    let dest_expect = config_decl.get_field("dest").unwrap().type_expr.resolved();
    let provider: Runtime = fetch_expression_or_null(entity.get_item("provider"), schema, entity, provider_expect)?.try_into()?;
    let dest: String = fetch_expression_or_null(entity.get_item("dest"), schema, entity, dest_expect)?.try_into()?;
    let entity_config = Entity {
        provider,
        dest,
    };
    dest_namespace.entities.insert(entity.name().to_owned(), entity_config);
    Ok(())
}
