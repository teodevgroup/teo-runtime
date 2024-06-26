use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use crate::config::entity::{Entity, Runtime};
use teo_result::Result;
use crate::namespace;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_entity(main_namespace: &namespace::Builder, schema: &Schema, entity: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("entity", entity.availability()).unwrap();
    let provider_expect = config_decl.get_field("provider").unwrap().type_expr().resolved();
    let dest_expect = config_decl.get_field("dest").unwrap().type_expr().resolved();
    let provider: Runtime = fetch_expression_or_null(entity.get_item("provider"), schema, entity, provider_expect, main_namespace, diagnostics)?.try_into()?;
    let dest: String = fetch_expression_or_null(entity.get_item("dest"), schema, entity, dest_expect, main_namespace, diagnostics)?.try_into()?;
    let entity_config = Entity {
        provider,
        dest,
    };
    let dest_namespace = main_namespace.namespace_or_create_at_path(&entity.namespace_str_path());
    dest_namespace.insert_entity(entity.name().to_owned(), entity_config);
    Ok(())
}
