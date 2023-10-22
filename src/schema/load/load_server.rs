use teo_parser::ast::config::Config;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use crate::config::server::Server;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_server(dest_namespace: &mut Namespace, schema: &Schema, server: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("server", server.availability()).unwrap();
    let path_prefix_expect = config_decl.get_field("pathPrefix").unwrap().type_expr.resolved();
    let path_prefix: Option<String> = fetch_expression_or_null(server.get_item("pathPrefix"), schema, server, path_prefix_expect)?.try_into()?;
    let bind_expect = config_decl.get_field("bind").unwrap().type_expr.resolved();
    let bind: (String, i32) = fetch_expression_or_null(server.get_item("bind"), schema, server, bind_expect)?.try_into()?;
    let server = Server {
        bind,
        path_prefix,
    };
    dest_namespace.server = Some(server);
    Ok(())
}