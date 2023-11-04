use teo_parser::ast::schema::Schema;
use teo_parser::r#type::Type;
use teo_result::Result;
use crate::data_set::{DataSet, Group, Record};
use crate::namespace::Namespace;
use crate::schema::fetch::fetchers::fetch_literals::fetch_dictionary_literal;

pub fn load_data_sets(namespace: &Namespace, names: Option<&Vec<String>>, all: bool, schema: &Schema) -> Result<Vec<DataSet>> {
    let mut result: Vec<DataSet> = vec![];
    for schema_data_set in schema.data_sets() {
        if result.iter().find(|d| d.name == schema_data_set.string_path).is_none() {
            result.push(DataSet {
                notrack: false,
                autoseed: false,
                name: schema_data_set.string_path.clone(),
                groups: vec![]
            });
        }
        let data_set = result.iter_mut().find(|d| d.name == schema_data_set.string_path).unwrap();
        data_set.notrack = schema_data_set.notrack;
        data_set.autoseed = schema_data_set.auto_seed;
        for schema_group in &schema_data_set.groups {
            if data_set.groups.iter().find(|g| g.name == schema_group.resolved().model_string_path).is_none() {
                data_set.groups.push(Group {
                    name: schema_group.resolved().model_string_path.clone(),
                    records: vec![]
                });
            }
            let group = data_set.groups.iter_mut().find(|g| g.name == schema_group.resolved().model_string_path).unwrap();
            for schema_record in &schema_group.records {
                let record = Record {
                    name: schema_record.identifier.name().to_owned(),
                    value: fetch_dictionary_literal(&schema_record.dictionary, schema, schema_record, &Type::Undetermined, namespace)?.as_teon().unwrap().clone(),
                };
                group.records.push(record);
            }
        }

    }
    Ok(result)
}