use crate::database::database::Database;
use crate::namespace;

pub(super) fn load_database_information(main_namespace: &namespace::Builder) {
    load_database_for_child_database(main_namespace, None, None);
}

fn load_database_for_child_database(namespace: &namespace::Builder, parent_database: Option<Database>, parent_ref: Option<Vec<String>>) {
    let current_database = namespace.connector().map(|c| c.provider());
    let current_ref = if namespace.connector().is_some() { Some(namespace.path().clone()) } else { None };
    if current_database.is_some() {
        namespace.set_database(current_database);
        namespace.set_database(current_database);
        namespace.set_connector_reference(current_ref);
    } else {
        namespace.set_database(parent_database);
        namespace.set_connector_reference(parent_ref);
    }
    let parent_database = namespace.database();
    let parent_ref = namespace.connector_reference();
    for namespace in namespace.namespaces().values() {
        load_database_for_child_database(namespace, parent_database, parent_ref.clone());
    }
}