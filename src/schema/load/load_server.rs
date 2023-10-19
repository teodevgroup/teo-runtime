use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::config::server::Server;
use crate::namespace::Namespace;
use crate::result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_server(dest_namespace: &mut Namespace, schema: &Schema, server: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let path_prefix: Option<String> = fetch_expression_or_null(server.get_item("pathPrefix"), schema, server).try_into()?;
    let bind: (String, i32) = fetch_expression_or_null(server.get_item("bind"), schema, server).try_into()?;
    let server = Server {
        bind,
        path_prefix,
    };
    dest_namespace.server = Some(server);
    Ok(())
}