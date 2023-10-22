use teo_parser::ast::config::Config;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::config::debug::Debug;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_debug(dest_namespace: &mut Namespace, schema: &Schema, debug: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let config_decl = schema.find_config_declaration_by_name("debug", debug.availability()).unwrap();
    let expect = config_decl.get_field("logQueries").unwrap().type_expr.resolved();
    let log_queries: bool = fetch_expression_or_null(debug.get_item("logQueries"), schema, debug, expect)?.try_into()?;
    let log_migrations: bool = fetch_expression_or_null(debug.get_item("logMigrations"), schema, debug, expect)?.try_into()?;
    let log_seed_records: bool = fetch_expression_or_null(debug.get_item("logSeedRecords"), schema, debug, expect)?.try_into()?;
    let debug = Debug {
        log_migrations,
        log_queries,
        log_seed_records,
    };
    dest_namespace.debug = Some(debug);
    Ok(())
}
