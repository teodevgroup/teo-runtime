use teo_parser::ast::config::Config;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::config::test::{ResetDataSets, Test};
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::fetch::fetch_expression::fetch_expression_or_null;

pub fn load_test(dest_namespace: &mut Namespace, schema: &Schema, debug: &Config, diagnostics: &mut Diagnostics) -> Result<()> {
    let reset_after_query: bool = fetch_expression_or_null(debug.get_item("resetAfterQuery"), schema, debug).try_into()?;
    let reset_after_mutation: bool = fetch_expression_or_null(debug.get_item("resetAfterMutation"), schema, debug).try_into()?;
    let reset_data_sets: ResetDataSets = fetch_expression_or_null(debug.get_item("resetDataSets"), schema, debug).try_into()?;
    let test = Test {
        reset_after_query,
        reset_after_mutation,
        reset_data_sets,
    };
    dest_namespace.test = Some(test);
    Ok(())
}
