use std::ops::Deref;
use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use teo_parser::ast::span::Span;
use teo_parser::diagnostics::diagnostics::{Diagnostics, DiagnosticsError};
use crate::namespace::Namespace;
use crate::result::Result;
use crate::schema::load::load_connector::load_connector;
use crate::schema::load::load_debug::load_debug;
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
    for debug in schema.entities() {
        if debug.is_available() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&debug.namespace_str_path());
            //load_entity(dest_namespace, schema, debug, &mut diagnostics)?;
        }
    }

    // load clients
    for debug in schema.clients() {
        if debug.is_available() {
            let dest_namespace = main_namespace.namespace_mut_or_create_at_path(&debug.namespace_str_path());
            //load_client(dest_namespace, schema, debug, &mut diagnostics)?;
        }
    }

    //
    // pub fn enums(&self) -> Vec<&Enum> {
    //     self.references.enums.iter().map(|path| self.find_top_by_path(path).unwrap().as_enum().unwrap()).collect()
    // }
    //
    // pub fn models(&self) -> Vec<&Model> {
    //     self.references.models.iter().map(|path| self.find_top_by_path(path).unwrap().as_model().unwrap()).collect()
    // }
    //
    // pub fn data_sets(&self) -> Vec<&DataSet> {
    //     self.references.data_sets.iter().map(|path| self.find_top_by_path(path).unwrap().as_data_set().unwrap()).collect()
    // }
    //
    // pub fn interfaces(&self) -> Vec<&InterfaceDeclaration> {
    //     self.references.interfaces.iter().map(|path| self.find_top_by_path(path).unwrap().as_interface_declaration().unwrap()).collect()
    // }
    //
    // pub fn namespaces(&self) -> Vec<&Namespace> {
    //     self.references.namespaces.iter().map(|path| self.find_top_by_path(path).unwrap().as_namespace().unwrap()).collect()
    // }
    //
    // pub fn config_declarations(&self) -> Vec<&ConfigDeclaration> {
    //     self.references.config_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_config_declaration().unwrap()).collect()
    // }
    //
    // pub fn decorator_declarations(&self) -> Vec<&DecoratorDeclaration> {
    //     self.references.decorator_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_decorator_declaration().unwrap()).collect()
    // }
    //
    // pub fn pipeline_item_declarations(&self) -> Vec<&PipelineItemDeclaration> {
    //     self.references.pipeline_item_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_pipeline_item_declaration().unwrap()).collect()
    // }
    //
    // pub fn middleware_declarations(&self) -> Vec<&MiddlewareDeclaration> {
    //     self.references.middlewares.iter().map(|path| self.find_top_by_path(path).unwrap().as_middleware_declaration().unwrap()).collect()
    // }
    //
    // pub fn handler_group_declarations(&self) -> Vec<&HandlerGroupDeclaration> {
    //     self.references.handler_groups.iter().map(|path| self.find_top_by_path(path).unwrap().as_handler_group_declaration().unwrap()).collect()
    // }
    //
    // pub fn struct_declarations(&self) -> Vec<&StructDeclaration> {
    //     self.references.struct_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_struct_declaration().unwrap()).collect()
    // }
    Ok(())
}