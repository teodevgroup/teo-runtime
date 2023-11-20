use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::resolved::Resolve;
use crate::config::connector::{Connector};
use crate::namespace::Namespace;
use teo_result::Result;
use crate::database::database::Database;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_connector(main_namespace: &mut Namespace, schema: &Schema, connector: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("connector", connector.availability()).unwrap();
    let provider_expect = config_decl.get_field("provider").unwrap().type_expr().resolved();
    let url_expect = config_decl.get_field("url").unwrap().type_expr().resolved();
    let provider: Database = fetch_expression_or_null(connector.get_item("provider"), schema, connector, provider_expect, main_namespace)?.try_into()?;
    let url: String = fetch_expression_or_null(connector.get_item("url"), schema, connector, url_expect, main_namespace)?.try_into()?;
    let connector_conf = Connector {
        provider,
        url,
    };
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&connector.namespace_str_path());
    dest_namespace.connector = Some(connector_conf);
    Ok(())
}