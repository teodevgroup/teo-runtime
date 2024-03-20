use itertools::Itertools;
use crate::arguments::Arguments;
use crate::model::Model;
use teo_result::Result;
use crate::{index, model};
use crate::model::field::indexable::Indexable;
use crate::sort::Sort;
use crate::value::interface_enum_variant::InterfaceEnumVariant;

pub fn model_id_decorator(arguments: Arguments, model: &mut Model) -> Result<()> {
    decorator(index::Type::Primary, arguments, model)
}

pub fn model_unique_decorator(arguments: Arguments, model: &mut Model) -> Result<()> {
    decorator(index::Type::Unique, arguments, model)
}

pub fn model_index_decorator(arguments: Arguments, model: &mut Model) -> Result<()> {
    decorator(index::Type::Index, arguments, model)
}

fn decorator(r#type: index::Type, arguments: Arguments, model: &mut Model) -> Result<()> {
    let fields: Vec<InterfaceEnumVariant> = arguments.get("fields")?;
    let map: Option<String> = arguments.get_optional("map")?;
    let name = map.unwrap_or(default_index_name(&fields));
    model.indexes.insert(name.clone(), model::Index::new(r#type, name, fields.iter().map(|f| {
        model::index::Item {
            field: f.value.clone(),
            sort: if let Some(args) = f.args() {
                let sort: Result<Sort> = args.get("sort");
                if let Ok(sort) = sort {
                    sort
                } else {
                    Sort::Asc
                }
            } else {
                Sort::Asc
            },
            len: if let Some(args) = f.args() {
                let length: Result<usize> = args.get("length");
                if let Ok(length) = length {
                    Some(length)
                } else {
                    None
                }
            } else {
                None
            },
        }
    }).collect()));
    Ok(())
}

fn default_index_name(fields: &Vec<InterfaceEnumVariant>) -> String {
    fields.iter().map(|f| f.value.as_str()).sorted().join("_")
}