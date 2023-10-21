use teo_result::Result;
use crate::database::database::Database;
use crate::namespace::Namespace;

pub(super) fn load_database_information(main_namespace: &mut Namespace) {
    load_database_for_child_database(main_namespace, None, None);
}

fn load_database_for_child_database(namespace: &mut Namespace, parent_database: Option<Database>, parent_ref: Option<Vec<String>>) {
    let current_database = namespace.connector.as_ref().map(|c| c.provider);
    let current_ref = if namespace.connector.as_ref().is_some() { Some(namespace.path.clone()) } else { None };
    if current_database.is_some() {
        namespace.database = current_database;
        namespace.connector_reference = current_ref;
    } else {
        namespace.database = parent_database;
        namespace.connector_reference = parent_ref;
    }
    let parent_database = namespace.database.clone();
    let parent_ref = namespace.connector_reference.clone();
    for namespace in namespace.namespaces.values_mut() {
        load_database_for_child_database(namespace, parent_database.clone(), parent_ref.clone());
    }
}