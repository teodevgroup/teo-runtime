use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::resolved::Resolve;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::admin::language::Language;
use crate::config::admin::Admin;
use crate::config::client::ClientHost;
use crate::schema::fetch::fetch_expression::{fetch_expression_or_default, fetch_expression_or_null};
use crate::Value;
use crate::value::interface_enum_variant::InterfaceEnumVariant;

pub fn load_admin(main_namespace: &mut Namespace, schema: &Schema, admin: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("admin", admin.availability()).unwrap();
    let dest_expect = config_decl.get_field("dest").unwrap().type_expr().resolved();
    let dest: String = fetch_expression_or_null(admin.get_item("dest"), schema, admin, dest_expect, main_namespace, diagnostics)?.try_into()?;
    let host_expect = config_decl.get_field("host").unwrap().type_expr().resolved();
    let host: ClientHost = fetch_expression_or_null(admin.get_item("host"), schema, admin, host_expect, main_namespace, diagnostics)?.try_into()?;
    let languages_expect = config_decl.get_field("languages").unwrap().type_expr().resolved();
    let languages: Vec<Language> = fetch_expression_or_default(admin.get_item("languages"), schema, admin, Value::Array(vec![Value::InterfaceEnumVariant(InterfaceEnumVariant {
        value: "enUs".to_string(),
        args: None,
    })]), languages_expect, main_namespace, diagnostics)?.try_into()?;
    let admin_config = Admin {
        dest,
        host,
        languages
    };
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&admin.namespace_str_path());
    dest_namespace.admin = Some(admin_config);
    Ok(())
}
