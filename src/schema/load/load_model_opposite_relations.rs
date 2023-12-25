use std::collections::BTreeMap;
use maplit::btreemap;
use crate::namespace::Namespace;

pub(super) fn load_model_opposite_relations(main_namespace: &mut Namespace) {
    let relations = fetch_model_opposite_relations(main_namespace);
    main_namespace.model_opposite_relations_map = relations;
}

fn fetch_model_opposite_relations(main_namespace: &Namespace) -> BTreeMap<Vec<String>, Vec<(Vec<String>, String)>> {
    let mut result = btreemap! {};
    add_model_opposite_relations_from_namespace(main_namespace, &mut result);
    result
}

fn add_model_opposite_relations_from_namespace(namespace: &Namespace, result: &mut BTreeMap<Vec<String>, Vec<(Vec<String>, String)>>) {
    for model in namespace.models.values() {
        ensure_entry_for_model(&model.path, result);
        for relation in model.relations.values() {
            install_entry_for_model(&relation.model, result, &model.path, &relation.name);
        }
    }
    for namespace in namespace.namespaces.values() {
        add_model_opposite_relations_from_namespace(namespace, result);
    }
}

fn ensure_entry_for_model(model_path: &Vec<String>, result: &mut BTreeMap<Vec<String>, Vec<(Vec<String>, String)>>) {
    if result.get(model_path).is_none() {
        result.insert(model_path.clone(), vec![]);
    }
}

fn install_entry_for_model(
    model_path: &Vec<String>,
    result: &mut BTreeMap<Vec<String>, Vec<(Vec<String>, String)>>,
    model: &Vec<String>,
    relation: &String,
) {
    ensure_entry_for_model(model_path, result);
    result.get_mut(model_path).unwrap().push((model.clone(), relation.clone()));
}