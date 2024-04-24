use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::resolved::Resolve;
use crate::config::debug::Debug;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_debug(main_namespace: &mut Namespace, schema: &Schema, debug: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("debug", debug.availability()).unwrap();
    let expect = config_decl.get_field("logQueries").unwrap().type_expr().resolved();
    let log_queries: bool = fetch_expression_or_null(debug.get_item("logQueries"), schema, debug, expect, main_namespace, diagnostics)?.try_into()?;
    let log_migrations: bool = fetch_expression_or_null(debug.get_item("logMigrations"), schema, debug, expect, main_namespace, diagnostics)?.try_into()?;
    let log_seed_records: bool = fetch_expression_or_null(debug.get_item("logSeedRecords"), schema, debug, expect, main_namespace, diagnostics)?.try_into()?;
    let debug_conf = Debug {
        log_migrations,
        log_queries,
        log_seed_records,
    };
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&debug.namespace_str_path());
    dest_namespace.debug = Some(debug_conf);
    Ok(())
}
