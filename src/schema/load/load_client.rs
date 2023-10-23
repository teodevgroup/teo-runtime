use teo_parser::ast::config::Config;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::config::client::{Client, ClientLanguage};
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::{fetch_expression_or_default, fetch_expression_or_null};

pub fn load_client(main_namespace: &mut Namespace, schema: &Schema, client: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("entity", client.availability()).unwrap();
    let provider_expect = config_decl.get_field("provider").unwrap().type_expr.resolved();
    let dest_expect = config_decl.get_field("dest").unwrap().type_expr.resolved();
    let package_expect = config_decl.get_field("package").unwrap().type_expr.resolved();
    let host_expect = config_decl.get_field("host").unwrap().type_expr.resolved();
    let object_name_expect = config_decl.get_field("objectName").unwrap().type_expr.resolved();
    let git_commit_expect = config_decl.get_field("gitCommit").unwrap().type_expr.resolved();

    let provider: ClientLanguage = fetch_expression_or_null(client.get_item("provider"), schema, client, provider_expect, main_namespace)?.try_into()?;
    let dest: String = fetch_expression_or_null(client.get_item("dest"), schema, client, dest_expect, main_namespace)?.try_into()?;
    let package: bool = fetch_expression_or_default(client.get_item("package"), schema, client, true, package_expect, main_namespace)?.try_into()?;
    let host: String = fetch_expression_or_null(client.get_item("host"), schema, client, host_expect, main_namespace)?.try_into()?;
    let object_name: String = fetch_expression_or_default(client.get_item("objectName"), schema, client, "teo", object_name_expect, main_namespace)?.try_into()?;
    let git_commit: bool = fetch_expression_or_default(client.get_item("gitCommit"), schema, client, false, git_commit_expect, main_namespace)?.try_into()?;
    let client_config = Client {
        provider,
        dest,
        package,
        host,
        object_name,
        git_commit,
    };
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&client.namespace_str_path());
    dest_namespace.clients.insert(client.name().to_owned(), client_config);
    Ok(())
}
