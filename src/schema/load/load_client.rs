use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::config::client::{Client, ClientLanguage};
use crate::namespace::Namespace;
use crate::result::Result;
use crate::schema::fetch::fetch_expression::{fetch_expression_or_default, fetch_expression_or_null};

pub fn load_client(dest_namespace: &mut Namespace, schema: &Schema, client: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let provider: ClientLanguage = fetch_expression_or_null(client.get_item("provider"), schema, client).try_into()?;
    let dest: String = fetch_expression_or_null(client.get_item("dest"), schema, client).try_into()?;
    let package: bool = fetch_expression_or_default(client.get_item("package"), schema, client, true).try_into()?;
    let host: String = fetch_expression_or_null(client.get_item("host"), schema, client).try_into()?;
    let object_name: String = fetch_expression_or_default(client.get_item("objectName"), schema, client, "teo").try_into()?;
    let git_commit: bool = fetch_expression_or_default(client.get_item("gitCommit"), schema, client, false).try_into()?;
    let client_config = Client {
        provider,
        dest,
        package,
        host,
        object_name,
        git_commit,
    };
    dest_namespace.clients.insert(client.name().to_owned(), client_config);
    Ok(())
}
