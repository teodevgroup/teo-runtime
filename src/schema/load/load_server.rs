use teo_parser::ast::config::Config;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::r#type::Type;
use crate::config::server::Server;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_server(main_namespace: &mut Namespace, schema: &Schema, server: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("server", server.availability()).unwrap();
    let path_prefix_expect = config_decl.get_field("pathPrefix").unwrap().type_expr.resolved();
    let path_prefix: Option<String> = fetch_expression_or_null(server.get_item("pathPrefix"), schema, server, path_prefix_expect, main_namespace)?.try_into()?;
    let bind_expect = config_decl.get_field("bind").unwrap().type_expr.resolved();
    let bind: (String, i32) = fetch_expression_or_null(server.get_item("bind"), schema, server, bind_expect, main_namespace)?.try_into()?;
    let server_conf = Server {
        bind,
        path_prefix,
    };
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&server.namespace_str_path());
    dest_namespace.server = Some(server_conf);
    Ok(())
}