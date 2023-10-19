use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::config::connector::{Connector, Database};
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_connector(dest_namespace: &mut Namespace, schema: &Schema, connector: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let provider: Database = fetch_expression_or_null(connector.get_item("provider"), schema, connector).try_into()?;
    let url: String = fetch_expression_or_null(connector.get_item("url"), schema, connector).try_into()?;
    let connector = Connector {
        provider,
        url,
    };
    dest_namespace.connector = Some(connector);
    Ok(())
}