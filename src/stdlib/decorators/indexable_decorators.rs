use teo_result::Result;
use crate::arguments::Arguments;
use crate::model::field;
use crate::model::field::indexable::Indexable;
use crate::sort::Sort;
use crate::index;

pub(super) fn id_decorator<I>(arguments: Arguments, indexable: &mut I) -> Result<()> where I: Indexable {
    decorator(index::Type::Primary, arguments, indexable)
}

pub(super) fn unique_decorator<I>(arguments: Arguments, indexable: &mut I) -> Result<()> where I: Indexable {
    decorator(index::Type::Unique, arguments, indexable)
}

pub(super) fn index_decorator<I>(arguments: Arguments, indexable: &mut I) -> Result<()> where I: Indexable {
    decorator(index::Type::Index, arguments, indexable)
}

fn decorator<I>(r#type: index::Type, arguments: Arguments, indexable: &mut I) -> Result<()> where I: Indexable {
    let sort: Option<Sort> = arguments.get_optional("sort")?;
    let length: Option<usize> = arguments.get_optional("length")?;
    let map: Option<String> = arguments.get_optional("map")?;
    indexable.set_index(field::index::Index {
        r#type,
        name: map.unwrap_or(indexable.name().to_owned()),
        sort: sort.unwrap_or(Sort::Asc),
        length,
    });
    Ok(())
}