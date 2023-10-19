use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use crate::model::Model;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::schema::load::load_comment::load_comment;

pub fn load_model(main_namespace: &mut Namespace, schema: &Schema, model_declaration: &teo_parser::ast::model::Model, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut model = Model::new();
    model.path = model_declaration.string_path.clone();
    model.comment = load_comment(model_declaration.comment.as_ref());
    // for enum_member in &enum_declaration.members {
    //     r#enum.members.push(
    //         Member::new(enum_member.identifier.name().to_owned(), enum_member.resolved().value.clone())
    //     );
    // }
    model.finalize();
    let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&model_declaration.namespace_str_path());
    dest_namespace.models.insert(model_declaration.identifier.name().to_owned(), model);
    Ok(())
}
