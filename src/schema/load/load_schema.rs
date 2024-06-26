use std::process::exit;
use teo_parser::ast::reference_space::ReferenceSpace;
use teo_parser::ast::schema::Schema;
use teo_parser::ast::span::Span;
use teo_parser::diagnostics::diagnostics::{Diagnostics, DiagnosticsError};
use teo_parser::diagnostics::printer::print_diagnostics;
use teo_parser::traits::has_availability::HasAvailability;
use teo_parser::traits::identifiable::Identifiable;
use teo_parser::traits::info_provider::InfoProvider;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::node_trait::NodeTrait;
use crate::namespace::Namespace;
use teo_result::Result;
use crate::namespace;
use crate::schema::load::load_admin::load_admin;
use crate::schema::load::load_client::load_client;
use crate::schema::load::load_connector::load_connector;
use crate::schema::load::load_database_information::load_database_information;
use crate::schema::load::load_debug::load_debug;
use crate::schema::load::load_entity::load_entity;
use crate::schema::load::load_enum::load_enum;
use crate::schema::load::load_handler::load_handler;
use crate::schema::load::load_handler_group::load_handler_group;
use crate::schema::load::load_handler_template::load_handler_template;
use crate::schema::load::load_interface::load_interface;
use crate::schema::load::load_model::load_model;
use crate::schema::load::load_model_opposite_relations::load_model_opposite_relations;
use crate::schema::load::load_server::load_server;
use crate::schema::load::load_use_middlewares::load_use_middlewares;

pub async fn load_schema(main_namespace_builder: &namespace::Builder, schema: &Schema, ignores_loading: bool) -> Result<()> {

    // diagnostics for schema loading
    let mut diagnostics = Diagnostics::new();

    // some of these are just load from schema, while some are validate and load

    // setup namespaces, this is used for recursively setting database information
    for namespace in schema.namespaces() {
        let _ = main_namespace_builder.namespace_or_create_at_path(&namespace.str_path());
    }

    // load server
    let mut server_loaded = false;
    if let Some(server) = schema.server() {
        if server.is_available() {
            load_server(main_namespace_builder, schema, server, &mut diagnostics)?;
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
            load_connector(main_namespace_builder, schema, connector, &mut diagnostics)?;
        }
    }

    // setting up database information
    load_database_information(main_namespace_builder);

    // load debug
    if let Some(debug) = schema.debug() {
        if debug.is_available() {
            load_debug(main_namespace, schema, debug, &mut diagnostics)?;
        }
    }

    // load entities
    for entity in schema.entities() {
        if entity.is_available() {
            load_entity(main_namespace, schema, entity, &mut diagnostics)?;
        }
    }

    // load clients
    for debug in schema.clients() {
        if debug.is_available() {
            load_client(main_namespace, schema, debug, &mut diagnostics)?;
        }
    }

    // load admin dashboard
    if let Some(admin) = schema.admin() {
        if admin.is_available() {
            load_admin(main_namespace, schema, admin, &mut diagnostics)?;
        }
    }

    if !ignores_loading {

        // validate decorator declarations
        for decorator_declaration in schema.decorator_declarations() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&decorator_declaration.namespace_str_path());
            match decorator_declaration.decorator_class {
                ReferenceSpace::EnumDecorator => if dest_namespace.enum_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "enum decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                ReferenceSpace::EnumMemberDecorator => if dest_namespace.enum_member_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "enum member decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                ReferenceSpace::ModelDecorator => if dest_namespace.model_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "model decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                ReferenceSpace::ModelFieldDecorator => if dest_namespace.model_field_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "model field decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                ReferenceSpace::ModelRelationDecorator => if dest_namespace.model_relation_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "model relation decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                ReferenceSpace::ModelPropertyDecorator => if dest_namespace.model_property_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "model property decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                ReferenceSpace::InterfaceDecorator => if dest_namespace.interface_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "interface decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                ReferenceSpace::InterfaceFieldDecorator => if dest_namespace.interface_field_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "interface field decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                ReferenceSpace::HandlerDecorator => if dest_namespace.handler_decorators.get(decorator_declaration.identifier().name()).is_none() {
                    diagnostics.insert(DiagnosticsError::new(decorator_declaration.identifier().span(), "handler decorator implementation is not found", schema.source(decorator_declaration.source_id()).unwrap().file_path.clone()))
                },
                _ => (),
            }
        }

        // validate pipeline item declarations
        for pipeline_item_declaration in schema.pipeline_item_declarations() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&pipeline_item_declaration.namespace_str_path());
            if dest_namespace.pipeline_items.get(pipeline_item_declaration.identifier().name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(pipeline_item_declaration.identifier().span(), "pipeline item implementation is not found", schema.source(pipeline_item_declaration.source_id()).unwrap().file_path.clone()))
            }
        }

        // validate struct declarations
        for struct_declaration in schema.struct_declarations() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&struct_declaration.namespace_str_path());
            if let Some(struct_implementation) = dest_namespace.structs.get(struct_declaration.identifier().name()) {
                for function_declaration in struct_declaration.function_declarations() {
                    if function_declaration.r#static {
                        if struct_implementation.static_functions.get(function_declaration.identifier().name()).is_none() {
                            diagnostics.insert(DiagnosticsError::new(function_declaration.identifier().span(), "function implementation is not found", schema.source(struct_declaration.source_id()).unwrap().file_path.clone()));
                        }
                    } else {
                        if struct_implementation.functions.get(function_declaration.identifier().name()).is_none() {
                            diagnostics.insert(DiagnosticsError::new(function_declaration.identifier().span(), "function implementation is not found", schema.source(struct_declaration.source_id()).unwrap().file_path.clone()));
                        }
                    }
                }
            } else {
                diagnostics.insert(DiagnosticsError::new(struct_declaration.identifier().span(), "struct implementation is not found", schema.source(struct_declaration.source_id()).unwrap().file_path.clone()))
            }
        }

        // validate handlers
        for handler_declaration in schema.handler_declarations() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&handler_declaration.namespace_str_path());
            if dest_namespace.handlers.get(handler_declaration.identifier().name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(handler_declaration.identifier().span(), "handler implementation is not found", schema.source(handler_declaration.source_id()).unwrap().file_path.clone()));
            }
        }

        // validate handler templates
        for handler_template_declaration in schema.handler_template_declarations() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&handler_template_declaration.namespace_str_path());
            if dest_namespace.handler_templates.get(handler_template_declaration.identifier().name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(handler_template_declaration.identifier().span(), "handler template implementation is not found", schema.source(handler_template_declaration.source_id()).unwrap().file_path.clone()));
            }
        }

        // validate handler groups
        for handler_group_declaration in schema.handler_group_declarations() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&handler_group_declaration.namespace_str_path());
            if dest_namespace.handler_groups.get(handler_group_declaration.identifier().name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(handler_group_declaration.identifier().span(), "handler group implementation is not found", schema.source(handler_group_declaration.source_id()).unwrap().file_path.clone()));
            }
            if let Some(group) = dest_namespace.handler_groups.get_mut(handler_group_declaration.identifier().name()) {
                for handler_declaration in handler_group_declaration.handler_declarations() {
                    if group.handlers.get(handler_declaration.name()).is_none() {
                        diagnostics.insert(DiagnosticsError::new(handler_declaration.identifier().span(), "handler implementation is not found", schema.source(handler_group_declaration.source_id()).unwrap().file_path.clone()));
                    }
                }
            }
        }

        // validate middleware declarations
        for middleware_declaration in schema.middleware_declarations() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&middleware_declaration.namespace_str_path());
            if dest_namespace.middlewares.get(middleware_declaration.identifier().name()).is_none() {
                diagnostics.insert(DiagnosticsError::new(middleware_declaration.identifier().span(), "middleware implementation is not found", schema.source(middleware_declaration.source_id()).unwrap().file_path.clone()))
            }
        }

        // load middlewares
        load_use_middlewares(main_namespace, schema, &mut diagnostics).await?;
    }

    // load enums
    for enum_declaration in schema.enums() {
        if enum_declaration.is_available() {
            load_enum(main_namespace, schema, enum_declaration, &mut diagnostics)?;
        }
    }

    // load interfaces
    for interface_declaration in schema.interfaces() {
        if interface_declaration.is_available() {
            load_interface(main_namespace, schema, interface_declaration, &mut diagnostics)?;
        }
    }

    // load handler templates
    for handler_template_declaration in schema.handler_template_declarations() {
        load_handler_template(main_namespace, schema, handler_template_declaration, &mut diagnostics)?;
    }

    // load handlers
    for handler_declaration in schema.handler_declarations() {
        load_handler(main_namespace, schema, handler_declaration, &mut diagnostics)?;
    }

    // load handler groups
    for handler_group_declaration in schema.handler_group_declarations() {
        load_handler_group(main_namespace, schema, handler_group_declaration, &mut diagnostics)?;
    }

    // load models
    for model_declaration in schema.models() {
        let database = main_namespace.namespace_mut_or_create_at_path(&model_declaration.namespace_str_path()).database;
        if database.is_some() && model_declaration.is_available() {
            load_model(main_namespace, schema, model_declaration, &mut diagnostics)?;
        }
    }

    // load model opposite relations
    load_model_opposite_relations(main_namespace);

    // diagnostics
    if !ignores_loading {
        print_diagnostics(&diagnostics, true);
        if diagnostics.has_errors() {
            exit(1);
        }
    }

    Ok(())
}