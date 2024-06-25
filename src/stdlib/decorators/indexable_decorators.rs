use teo_result::Result;
use crate::arguments::Arguments;
use crate::model::field;
use crate::model::field::indexable::{Indexable, SetIndex};
use crate::sort::Sort;
use crate::model;

pub(super) fn id_decorator<I>(arguments: Arguments, indexable: &I) -> Result<()> where I: SetIndex {
    decorator(model::index::Type::Primary, arguments, indexable)
}

pub(super) fn unique_decorator<I>(arguments: Arguments, indexable: &I) -> Result<()> where I: SetIndex {
    decorator(model::index::Type::Unique, arguments, indexable)
}

pub(super) fn index_decorator<I>(arguments: Arguments, indexable: &I) -> Result<()> where I: SetIndex {
    decorator(model::index::Type::Index, arguments, indexable)
}

fn decorator<I>(r#type: model::index::Type, arguments: Arguments, indexable: &I) -> Result<()> where I: SetIndex {
    let sort: Option<Sort> = arguments.get_optional("sort")?;
    let length: Option<usize> = arguments.get_optional("length")?;
    let map: Option<String> = arguments.get_optional("map")?;
    indexable.set_index(field::index::Index::new(r#type, map.unwrap_or(indexable.name().to_owned()), sort.unwrap_or(Sort::Asc), length));
    Ok(())
}