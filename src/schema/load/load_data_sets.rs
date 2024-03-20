use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_parser::traits::named_identifiable::NamedIdentifiable;
use teo_parser::traits::resolved::Resolve;
use teo_result::Result;
use crate::value::Value;
use crate::teon;
use crate::data_set::{DataSet, Group, Record};
use crate::namespace::Namespace;
use crate::schema::fetch::fetchers::fetch_literals::fetch_dictionary_literal;
use crate::traits::named::Named;

pub fn load_data_sets(namespace: &Namespace, names: Option<&Vec<String>>, all: bool, schema: &Schema) -> Result<Vec<DataSet>> {
    let mut result: Vec<DataSet> = vec![];
    for schema_data_set in schema.data_sets() {
        if all || (names.is_some() && names.unwrap().contains(&schema_data_set.string_path().join("."))) || (names.is_none() && schema_data_set.auto_seed) {
            if result.iter().find(|d| &d.name == schema_data_set.string_path()).is_none() {
                result.push(DataSet {
                    notrack: false,
                    autoseed: false,
                    name: schema_data_set.string_path().clone(),
                    groups: vec![]
                });
            }
            let data_set = result.iter_mut().find(|d| &d.name == schema_data_set.string_path()).unwrap();
            data_set.notrack = schema_data_set.notrack;
            data_set.autoseed = schema_data_set.auto_seed;
            for schema_group in schema_data_set.groups() {
                if data_set.groups.iter().find(|g| &g.name == schema_group.resolved().string_path()).is_none() {
                    data_set.groups.push(Group {
                        name: schema_group.resolved().string_path().clone(),
                        records: vec![]
                    });
                }
                let group = data_set.groups.iter_mut().find(|g| &g.name == schema_group.resolved().string_path()).unwrap();
                for schema_record in schema_group.records() {
                    let record = Record {
                        name: schema_record.identifier().name().to_owned(),
                        value: fetch_dictionary_literal(schema_record.dictionary(), schema, schema_record, &Type::Undetermined, namespace)?.as_teon().unwrap().clone(),
                    };
                    group.records.push(record);
                }
            }
            normalize_dataset_relations(data_set, namespace);
        }
    }
    Ok(result)
}

pub(crate) fn normalize_dataset_relations<'a>(dataset: &mut DataSet, namespace: &Namespace) {
    let mut assign_relation_other_sides = vec![];
    for group in &dataset.groups {
        let model = namespace.model_at_path(&group.model_path()).unwrap();
        for record in &group.records {
            for (k, v) in record.value.as_dictionary().unwrap() {
                if let Some(relation) = model.relation(k) {
                    let (opposite_model, opposite_rel) = namespace.opposite_relation(relation);
                    // If there isn't a relation defined on the opposite side, just leave it here
                    if opposite_rel.is_none() {
                        continue
                    }
                    let opposite_rel = opposite_rel.unwrap();
                    if relation.is_vec {
                        for v in v.as_array().unwrap() {
                            assign_relation_other_sides.push((dataset.name.clone(), opposite_model.path.clone(), v.as_str().unwrap().to_owned(), opposite_rel.name.clone(), record.name.clone()));
                        }
                    } else {
                        assign_relation_other_sides.push((dataset.name.clone(), opposite_model.path.clone(), v.as_str().unwrap().to_owned(), opposite_rel.name.clone(), record.name.clone()));
                    }
                }
            }
        }
    }
    for (data_set_name, model_name, record_name, field_name, value_name) in &assign_relation_other_sides {
        assign_relation_other_side(dataset, data_set_name, model_name, record_name, field_name, value_name, namespace);
    }
}

fn assign_relation_other_side(dataset: &mut DataSet, data_set_name: &Vec<String>, model_name: &Vec<String>, record_name: &String, field_name: &String, value_name: &String, namespace: &Namespace) {
    let that_group = dataset.groups.iter_mut().find(|g| &g.name == model_name).unwrap();
    let that_record = that_group.records.iter_mut().find(|r| &r.name == record_name).unwrap();
    let model = namespace.model_at_path(&model_name.iter().map(AsRef::as_ref).collect()).unwrap();
    let relation = model.relation(field_name).unwrap();
    if relation.is_vec {
        if that_record.value.as_dictionary_mut().unwrap().contains_key(relation.name()) {
            let array = that_record.value.as_dictionary_mut().unwrap().get_mut(relation.name()).unwrap().as_array_mut().unwrap();
            let to_insert = Value::String(value_name.clone());
            if !array.contains(&to_insert) {
                array.push(to_insert);
            }
        } else {
            that_record.value.as_dictionary_mut().unwrap().insert(relation.name().to_owned(), teon!([Value::String(value_name.clone())]));
        }
    } else {
        that_record.value.as_dictionary_mut().unwrap().insert(relation.name().to_owned(), Value::String(value_name.clone()));
    }
}