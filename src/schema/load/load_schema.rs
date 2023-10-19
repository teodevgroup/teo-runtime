use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::reference::ReferenceType;
use teo_parser::ast::schema::Schema;
use teo_parser::ast::span::Span;
use teo_parser::diagnostics::diagnostics::{Diagnostics, DiagnosticsError};
use crate::namespace::Namespace;
use crate::result::Result;
use crate::schema::load::load_client::load_client;
use crate::schema::load::load_connector::load_connector;
use crate::schema::load::load_debug::load_debug;
use crate::schema::load::load_entity::load_entity;
use crate::schema::load::load_server::load_server;
use crate::schema::load::load_test::load_test;

pub fn load_schema(main_namespace: &mut Namespace, schema: &Schema) -> Result<()> {

    // diagnostics for schema loading
    let mut diagnostics = Diagnostics::new();

    // some of these are just load from schema, while some are validate and load

    // load server
    let mut server_loaded = false;
    if let Some(server) = schema.server() {
        if server.is_available() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&server.namespace_str_path());
            load_server(dest_namespace, schema, server, &mut diagnostics)?;
            server_loaded = true;
        }
    }
    if !server_loaded {
        let source = schema.main_source();
        diagnostics.insert(DiagnosticsError::new(Span::default(), "server config is not found", source.file_path.clone()));
    }

    // load connectors
    for connector in schema.connectors() {
        if connector.is_available() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&connector.namespace_str_path());
            load_connector(dest_namespace, schema, connector, &mut diagnostics)?;
        }
    }

    // load debug
    for debug in schema.debug() {
        if debug.is_available() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&debug.namespace_str_path());
            load_debug(dest_namespace, schema, debug, &mut diagnostics)?;
        }
    }

    // load test
    for test in schema.debug() {
        if test.is_available() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&test.namespace_str_path());
            load_test(dest_namespace, schema, test, &mut diagnostics)?;
        }
    }

    // load entities
    for entity in schema.entities() {
        if entity.is_available() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&entity.namespace_str_path());
            load_entity(dest_namespace, schema, entity, &mut diagnostics)?;
        }
    }

    // load clients
    for debug in schema.clients() {
        if debug.is_available() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&debug.namespace_str_path());
            load_client(dest_namespace, schema, debug, &mut diagnostics)?;
        }
    }

    // validate decorator declarations
    for decorator_declaration in schema.decorator_declarations() {
        let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&decorator_declaration.namespace_str_path());
        match decorator_declaration.decorator_class {
            ReferenceType::EnumDecorator => if dest_namespace.enum_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "enum decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            ReferenceType::EnumMemberDecorator => if dest_namespace.enum_member_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "enum member decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            ReferenceType::ModelDecorator => if dest_namespace.model_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "model decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            ReferenceType::ModelFieldDecorator => if dest_namespace.model_field_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "model field decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            ReferenceType::ModelRelationDecorator => if dest_namespace.model_relation_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "model relation decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            ReferenceType::ModelPropertyDecorator => if dest_namespace.model_property_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "model property decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            ReferenceType::InterfaceDecorator => if dest_namespace.interface_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "interface decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            ReferenceType::InterfaceFieldDecorator => if dest_namespace.interface_field_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "interface field decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            ReferenceType::HandlerDecorator => if dest_namespace.handler_decorators.get(decorator_declaration.identifier.name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier.span, "handler decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
            },
            _ => (),
        }
    }

    // validate pipeline item declarations
    for pipeline_item_declaration in schema.pipeline_item_declarations() {
        let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&pipeline_item_declaration.namespace_str_path());
        if dest_namespace.pipeline_items.get(pipeline_item_declaration.identifier.name()).is_none() {
            diagnostics.insert(DiagnosticsError::new(pipeline_item_declaration.identifier.span, "pipeline item implementation is not found", schema.source(pipeline_item_declaration.source_id()).unwrap().file_path.clone()))
        }
    }

    // validate middleware declarations
    for middleware_declaration in schema.middleware_declarations() {
        let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&middleware_declaration.namespace_str_path());
        if dest_namespace.middlewares.get(middleware_declaration.identifier.name()).is_none() {
            diagnostics.insert(DiagnosticsError::new(middleware_declaration.identifier.span, "middleware implementation is not found", schema.source(middleware_declaration.source_id()).unwrap().file_path.clone()))
        }
    }

    // validate struct declarations
    for struct_declaration in schema.struct_declarations() {
        let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&struct_declaration.namespace_str_path());
        if let Some(struct_implementation) = dest_namespace.structs.get(struct_declaration.identifier.name()) {
            for function_declaration in &struct_declaration.function_declarations {
                if function_declaration.r#static {
                    if struct_implementation.static_functions.get(function_declaration.identifier.name()).is_none() {
                        diagnostics.insert(DiagnosticsError::new(function_declaration.identifier.span, "function implementation is not found", schema.source(struct_declaration.source_id()).unwrap().file_path.clone()));
                    }
                } else {
                    if struct_implementation.functions.get(function_declaration.identifier.name()).is_none() {
                        diagnostics.insert(DiagnosticsError::new(function_declaration.identifier.span, "function implementation is not found", schema.source(struct_declaration.source_id()).unwrap().file_path.clone()));
                    }
                }
            }
        } else {
            diagnostics.insert(DiagnosticsError::new(struct_declaration.identifier.span, "struct implementation is not found", schema.source(struct_declaration.source_id()).unwrap().file_path.clone()))
        }
    }

    for enum_declaration in schema.enums() {
        if enum_declaration.is_available() {

        }
    }

    //
    // pub fn enums(&self) -> Vec<&Enum> {
    //     self.references.enums.iter().map(|path| self.find_top_by_path(path).unwrap().as_enum().unwrap()).collect()
    // }
    //
    // pub fn interfaces(&self) -> Vec<&InterfaceDeclaration> {
    //     self.references.interfaces.iter().map(|path| self.find_top_by_path(path).unwrap().as_interface_declaration().unwrap()).collect()
    // }
    // pub fn models(&self) -> Vec<&Model> {
    //     self.references.models.iter().map(|path| self.find_top_by_path(path).unwrap().as_model().unwrap()).collect()
    // }
    //
    //
    // pub fn handler_group_declarations(&self) -> Vec<&HandlerGroupDeclaration> {
    //     self.references.handler_groups.iter().map(|path| self.find_top_by_path(path).unwrap().as_handler_group_declaration().unwrap()).collect()
    // }
    //
    // pub fn data_sets(&self) -> Vec<&DataSet> {
    //     self.references.data_sets.iter().map(|path| self.find_top_by_path(path).unwrap().as_data_set().unwrap()).collect()
    // }
    Ok(())
}