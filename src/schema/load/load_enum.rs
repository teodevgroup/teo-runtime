use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use crate::r#enum::member::Member;
use teo_result::Result;
use crate::{namespace, r#enum};
use crate::schema::load::load_comment::load_comment;

pub fn load_enum(main_namespace: &namespace::Builder, schema: &Schema, enum_declaration: &teo_parser::ast::r#enum::Enum, diagnostics: &mut Diagnostics) -> Result<()> {
    let mut members = vec![];
    for enum_member in enum_declaration.members() {
        if enum_member.is_available() {
            members.push(
                Member::new(
                    enum_member.identifier().name().to_owned(),
                    enum_member.resolved().clone().into(),
                    load_comment(enum_member.comment())
                )
            );
        }
    }
    let enum_builder = r#enum::Builder::new(
        enum_declaration.string_path().clone(),
        load_comment(enum_declaration.comment()),
        enum_declaration.option,
        enum_declaration.interface,
        members,
        main_namespace.app_data().clone(),
    );
    let r#enum = enum_builder.build();
    let dest_namespace = main_namespace.namespace_or_create_at_path(&enum_declaration.namespace_string_path());
    dest_namespace.insert_enum(enum_declaration.identifier().name().to_owned(), r#enum);
    Ok(())
}
