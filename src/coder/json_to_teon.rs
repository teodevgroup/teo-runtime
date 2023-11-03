use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use bigdecimal::{BigDecimal, FromPrimitive};
use bson::oid::ObjectId;
use chrono::{DateTime, NaiveDate, Utc};
use indexmap::{indexmap, IndexMap};
use itertools::Itertools;
use key_path::KeyPath;
use maplit::btreemap;
use teo_parser::r#type::shape_reference::ShapeReference;
use teo_parser::r#type::Type;
use teo_parser::shape::input::Input;
use teo_parser::shape::r#static::{STATIC_TYPES, STATIC_WHERE_INPUT_FOR_TYPE};
use teo_parser::shape::shape::Shape;
use teo_teon::Value;
use teo_teon::types::file::File;
use crate::interface::Interface;
use crate::namespace::Namespace;
use crate::path::Error;
use crate::utils::ContainsStr;

pub fn fetch_input<'a>(reference: &ShapeReference, main_namespace: &'a Namespace) -> crate::path::Result<Cow<'a, Input>> {
    match reference {
        ShapeReference::BoolFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("BoolFilter").unwrap())),
        ShapeReference::BoolNullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("BoolNullableFilter").unwrap())),
        ShapeReference::IntFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("IntFilter").unwrap())),
        ShapeReference::IntNullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("IntNullableFilter").unwrap())),
        ShapeReference::Int64Filter => Ok(Cow::Borrowed(STATIC_TYPES.get("Int64Filter").unwrap())),
        ShapeReference::Int64NullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("Int64NullableFilter").unwrap())),
        ShapeReference::Float32Filter => Ok(Cow::Borrowed(STATIC_TYPES.get("Float32Filter").unwrap())),
        ShapeReference::Float32NullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("Float32NullableFilter").unwrap())),
        ShapeReference::FloatFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("FloatFilter").unwrap())),
        ShapeReference::FloatNullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("FloatNullableFilter").unwrap())),
        ShapeReference::DecimalFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DecimalFilter").unwrap())),
        ShapeReference::DecimalNullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DecimalNullableFilter").unwrap())),
        ShapeReference::DateFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DateFilter").unwrap())),
        ShapeReference::DateNullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DateNullableFilter").unwrap())),
        ShapeReference::DateTimeFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DateTimeFilter").unwrap())),
        ShapeReference::DateTimeNullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DateTimeNullableFilter").unwrap())),
        ShapeReference::ObjectIdFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("ObjectIdFilter").unwrap())),
        ShapeReference::ObjectIdNullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("ObjectIdNullableFilter").unwrap())),
        ShapeReference::StringFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("StringFilter").unwrap())),
        ShapeReference::StringNullableFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("StringNullableFilter").unwrap())),
        ShapeReference::EnumFilter(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("EnumFilter").unwrap().as_shape().unwrap().replace_generics(&map))))
        },
        ShapeReference::EnumNullableFilter(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("EnumNullableFilter").unwrap().as_shape().unwrap().replace_generics(&map))))
        }
        ShapeReference::ArrayFilter(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("ArrayFilter").unwrap().as_shape().unwrap().replace_generics(&map))))
        }
        ShapeReference::ArrayNullableFilter(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("ArrayNullableFilter").unwrap().as_shape().unwrap().replace_generics(&map))))
        }
        ShapeReference::BoolWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("BoolWithAggregatesFilter").unwrap())),
        ShapeReference::BoolNullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("BoolNullableWithAggregatesFilter").unwrap())),
        ShapeReference::IntWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("IntWithAggregatesFilter").unwrap())),
        ShapeReference::IntNullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("IntNullableWithAggregatesFilter").unwrap())),
        ShapeReference::Int64WithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("Int64WithAggregatesFilter").unwrap())),
        ShapeReference::Int64NullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("Int64NullableWithAggregatesFilter").unwrap())),
        ShapeReference::Float32WithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("Float32WithAggregatesFilter").unwrap())),
        ShapeReference::Float32NullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("Float32NullableWithAggregatesFilter").unwrap())),
        ShapeReference::FloatWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("FloatWithAggregatesFilter").unwrap())),
        ShapeReference::FloatNullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("FloatNullableWithAggregatesFilter").unwrap())),
        ShapeReference::DecimalWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DecimalWithAggregatesFilter").unwrap())),
        ShapeReference::DecimalNullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DecimalNullableWithAggregatesFilter").unwrap())),
        ShapeReference::DateWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DateWithAggregatesFilter").unwrap())),
        ShapeReference::DateNullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DateNullableWithAggregatesFilter").unwrap())),
        ShapeReference::DateTimeWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DateTimeWithAggregatesFilter").unwrap())),
        ShapeReference::DateTimeNullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("DateTimeNullableWithAggregatesFilter").unwrap())),
        ShapeReference::ObjectIdWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("ObjectIdWithAggregatesFilter").unwrap())),
        ShapeReference::ObjectIdNullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("ObjectIdNullableWithAggregatesFilter").unwrap())),
        ShapeReference::StringWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("StringWithAggregatesFilter").unwrap())),
        ShapeReference::StringNullableWithAggregatesFilter => Ok(Cow::Borrowed(STATIC_TYPES.get("StringNullableWithAggregatesFilter").unwrap())),
        ShapeReference::EnumWithAggregatesFilter(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("EnumWithAggregatesFilter").unwrap().as_shape().unwrap().replace_generics(&map))))
        }
        ShapeReference::EnumNullableWithAggregatesFilter(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("EnumNullableWithAggregatesFilter").unwrap().as_shape().unwrap().replace_generics(&map))))
        }
        ShapeReference::ArrayWithAggregatesFilter(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("ArrayWithAggregatesFilter").unwrap().as_shape().unwrap().replace_generics(&map))))
        }
        ShapeReference::ArrayNullableWithAggregatesFilter(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("ArrayNullableWithAggregatesFilter").unwrap().as_shape().unwrap().replace_generics(&map))))
        }
        ShapeReference::IntAtomicUpdateOperationInput => Ok(Cow::Borrowed(STATIC_TYPES.get("IntAtomicUpdateOperationInput").unwrap())),
        ShapeReference::Int64AtomicUpdateOperationInput => Ok(Cow::Borrowed(STATIC_TYPES.get("Int64AtomicUpdateOperationInput").unwrap())),
        ShapeReference::Float32AtomicUpdateOperationInput => Ok(Cow::Borrowed(STATIC_TYPES.get("Float32AtomicUpdateOperationInput").unwrap())),
        ShapeReference::FloatAtomicUpdateOperationInput => Ok(Cow::Borrowed(STATIC_TYPES.get("FloatAtomicUpdateOperationInput").unwrap())),
        ShapeReference::DecimalAtomicUpdateOperationInput => Ok(Cow::Borrowed(STATIC_TYPES.get("DecimalAtomicUpdateOperationInput").unwrap())),
        ShapeReference::ArrayAtomicUpdateOperationInput(i) => {
            let map = btreemap! {"T".to_owned() => i.as_ref().clone()};
            Ok(Cow::Owned(Input::Shape(STATIC_TYPES.get("ArrayAtomicUpdateOperationInput").unwrap().as_shape().unwrap().replace_generics(&map))))
        }
        ShapeReference::Args(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("Args").unwrap()))
        }
        ShapeReference::FindManyArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("FindManyArgs").unwrap()))
        }
        ShapeReference::FindFirstArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("FindFirstArgs").unwrap()))
        }
        ShapeReference::FindUniqueArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("FindUniqueArgs").unwrap()))
        }
        ShapeReference::CreateArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CreateArgs").unwrap()))
        }
        ShapeReference::UpdateArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpdateArgs").unwrap()))
        }
        ShapeReference::UpsertArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpsertArgs").unwrap()))
        }
        ShapeReference::CopyArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CopyArgs").unwrap()))
        }
        ShapeReference::DeleteArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("DeleteArgs").unwrap()))
        }
        ShapeReference::CreateManyArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CreateManyArgs").unwrap()))
        }
        ShapeReference::UpdateManyArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpdateManyArgs").unwrap()))
        }
        ShapeReference::CopyManyArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CopyManyArgs").unwrap()))
        }
        ShapeReference::DeleteManyArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("DeleteManyArgs").unwrap()))
        }
        ShapeReference::CountArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CountArgs").unwrap()))
        }
        ShapeReference::AggregateArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("AggregateArgs").unwrap()))
        }
        ShapeReference::GroupByArgs(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("GroupByArgs").unwrap()))
        }
        ShapeReference::RelationFilter(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("RelationFilter").unwrap()))
        }
        ShapeReference::ListRelationFilter(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("ListRelationFilter").unwrap()))
        }
        ShapeReference::WhereInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("WhereInput").unwrap()))
        }
        ShapeReference::WhereUniqueInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("WhereUniqueInput").unwrap()))
        }
        ShapeReference::ScalarFieldEnum(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("ScalarFieldEnum").unwrap()))
        }
        ShapeReference::ScalarWhereWithAggregatesInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("ScalarWhereWithAggregatesInput").unwrap()))
        }
        ShapeReference::CountAggregateInputType(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CountAggregateInputType").unwrap()))
        }
        ShapeReference::SumAggregateInputType(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("SumAggregateInputType").unwrap()))
        }
        ShapeReference::AvgAggregateInputType(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("AvgAggregateInputType").unwrap()))
        }
        ShapeReference::MaxAggregateInputType(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("MaxAggregateInputType").unwrap()))
        }
        ShapeReference::MinAggregateInputType(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("MinAggregateInputType").unwrap()))
        }
        ShapeReference::CreateInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CreateInput").unwrap()))
        }
        ShapeReference::CreateInputWithout(_, k, _) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CreateInputWithout").unwrap()))
        }
        ShapeReference::CreateNestedOneInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CreateNestedOneInput").unwrap()))
        }
        ShapeReference::CreateNestedOneInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("CreateNestedOneInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::CreateNestedManyInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CreateNestedManyInput").unwrap()))
        }
        ShapeReference::CreateNestedManyInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("CreateNestedManyInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::UpdateInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpdateInput").unwrap()))
        }
        ShapeReference::UpdateInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("UpdateInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::UpdateNestedOneInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpdateNestedOneInput").unwrap()))
        }
        ShapeReference::UpdateNestedOneInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("UpdateNestedOneInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::UpdateNestedManyInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpdateNestedManyInput").unwrap()))
        }
        ShapeReference::UpdateNestedManyInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("UpdateNestedManyInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::ConnectOrCreateInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("ConnectOrCreateInput").unwrap()))
        }
        ShapeReference::ConnectOrCreateInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("ConnectOrCreateInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::UpdateWithWhereUniqueInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpdateWithWhereUniqueInput").unwrap()))
        }
        ShapeReference::UpdateWithWhereUniqueInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("UpdateWithWhereUniqueInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::UpsertWithWhereUniqueInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpsertWithWhereUniqueInput").unwrap()))
        }
        ShapeReference::UpsertWithWhereUniqueInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("UpsertWithWhereUniqueInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::UpdateManyWithWhereInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("UpdateManyWithWhereInput").unwrap()))
        }
        ShapeReference::UpdateManyWithWhereInputWithout(_, k, without) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.map.get(&("UpdateManyWithWhereInputWithout".to_owned(), Some(without.clone()))).unwrap()))
        }
        ShapeReference::Select(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("Select").unwrap()))
        }
        ShapeReference::Include(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("Include").unwrap()))
        }
        ShapeReference::OrderByInput(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("OrderByInput").unwrap()))
        }
        ShapeReference::Result(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("Result").unwrap()))
        }
        ShapeReference::CountAggregateResult(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("CountAggregateResult").unwrap()))
        }
        ShapeReference::SumAggregateResult(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("SumAggregateResult").unwrap()))
        }
        ShapeReference::AvgAggregateResult(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("AvgAggregateResult").unwrap()))
        }
        ShapeReference::MinAggregateResult(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("MinAggregateResult").unwrap()))
        }
        ShapeReference::MaxAggregateResult(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("MaxAggregateResult").unwrap()))
        }
        ShapeReference::AggregateResult(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("AggregateResult").unwrap()))
        }
        ShapeReference::GroupByResult(_, k) => {
            let model = main_namespace.model_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            Ok(Cow::Borrowed(model.cache.shape.get("GroupByResult").unwrap()))
        }
    }
}

pub fn json_to_teon_with_type(json: &serde_json::Value, path: &KeyPath, t: &Type, main_namespace: &Namespace) -> crate::path::Result<Value> {
    match t {
        Type::Undetermined => Ok(Value::from(json)),
        Type::Ignored => Ok(Value::from(json)),
        Type::Any => Ok(Value::from(json)),
        Type::Null => if json.is_null() { Ok(Value::Null) } else { Err(Error::value_error(path.clone(), "expect null")) },
        Type::Bool => if json.is_boolean() { Ok(Value::from(json)) } else { Err(Error::value_error(path.clone(), "expect bool")) },
        Type::Int => if json.is_i64() { Ok(Value::Int(json.as_i64().unwrap() as i32)) } else { Err(Error::value_error(path.clone(), "expect int")) },
        Type::Int64 => if json.is_i64() { Ok(Value::Int64(json.as_i64().unwrap())) } else { Err(Error::value_error(path.clone(), "expect int 64")) },
        Type::Float32 => if json.is_f64() { Ok(Value::Float32(json.as_f64().unwrap() as f32)) } else { Err(Error::value_error(path.clone(), "expect float 32")) },
        Type::Float => if json.is_f64() { Ok(Value::Float(json.as_f64().unwrap())) } else { Err(Error::value_error(path.clone(), "expect float")) },
        Type::Decimal => if json.is_string() {
            Ok(Value::Decimal(match BigDecimal::from_str(json.as_str().unwrap()) {
                Ok(s) => s,
                Err(_) => Err(Error::value_error(path.clone(), "string is not valid decimal"))?,
            }))
        } else if json.is_number() {
            if let Some(f) = json.as_f64() {
                Ok(Value::Decimal(match BigDecimal::from_f64(f) {
                    Some(s) => s,
                    None => Err(Error::value_error(path.clone(), "number is not valid decimal"))?,
                }))
            } else if let Some(i) = json.as_i64() {
                Ok(Value::Decimal(match BigDecimal::from_i64(i) {
                    Some(s) => s,
                    None => Err(Error::value_error(path.clone(), "number is not valid decimal"))?,
                }))
            } else {
                unreachable!()
            }
        } else {
            Err(Error::value_error(path.clone(), "expect string or number which represents decimal"))
        }
        Type::String => if json.is_string() { Ok(Value::String(json.as_str().unwrap().to_owned())) } else { Err(Error::value_error(path.clone(), "expect string")) },
        Type::ObjectId => if json.is_string() {
            Ok(Value::ObjectId(match ObjectId::parse_str(json.as_str().unwrap()) {
                Ok(s) => s,
                Err(_) => Err(Error::value_error(path.clone(), "string is not valid object id"))?,
            }))
        } else {
            Err(Error::value_error(path.clone(), "expect string represents object id"))
        }
        Type::Date => if json.is_string() {
            Ok(Value::Date(match NaiveDate::parse_from_str(json.as_str().unwrap(), "%Y-%m-%d") {
                Ok(s) => s,
                Err(_) => Err(Error::value_error(path.clone(), "string is not valid date"))?,
            }))
        } else {
            Err(Error::value_error(path.clone(), "expect string represents date"))
        }
        Type::DateTime => if json.is_string() {
            Ok(Value::DateTime(match DateTime::parse_from_rfc3339(json.as_str().unwrap()) {
                Ok(d) => d.with_timezone(&Utc),
                Err(_) => Err(Error::value_error(path.clone(), "string is not valid datetime"))?,
            }))
        } else {
            Err(Error::value_error(path.clone(), "expect string represents datetime"))
        }
        Type::File => Ok(Value::File(match File::try_from(json) {
            Ok(f) => f,
            Err(_) => Err(Error::value_error(path.clone(), "invalid file input"))?,
        })),
        Type::Regex => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::Model => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::DataSet => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::Enumerable(inner) => {
            if let Some(json_array) = json.as_array() {
                let values: Vec<Value> = json_array.iter().enumerate().map(|(i, j)| Ok(json_to_teon_with_type(j, &(path + i), inner.as_ref(), main_namespace)?)).collect::<crate::path::Result<Vec<Value>>>()?;
                Ok(Value::Array(values))
            } else {
                Ok(Value::Array(vec![json_to_teon_with_type(json, path, inner.as_ref(), main_namespace)?]))
            }
        }
        Type::Array(inner) => {
            if let Some(json_array) = json.as_array() {
                let values: Vec<Value> = json_array.iter().enumerate().map(|(i, j)| json_to_teon_with_type(j, &(path + i), inner.as_ref(), main_namespace)).collect::<crate::path::Result<Vec<Value>>>()?;
                Ok(Value::Array(values))
            } else {
                Err(Error::value_error(path.clone(), "expect array"))
            }
        }
        Type::Dictionary(inner) => {
            if let Some(json_object) = json.as_object() {
                let values: IndexMap<String, Value> = json_object.iter().map(|(k, j)| Ok((k.clone(), json_to_teon_with_type(j, &(path + k), inner.as_ref(), main_namespace)?))).collect::<crate::path::Result<IndexMap<String, Value>>>()?;
                Ok(Value::Dictionary(values))
            } else {
                Err(Error::value_error(path.clone(), "expect dictionary"))
            }
        }
        Type::Tuple(_) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::Range(_) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::Union(inners) => {
            for inner in inners {
                if let Ok(result) = json_to_teon_with_type(json, path, inner, main_namespace) {
                    return Ok(result);
                }
            }
            Err(Error::value_error(path.clone(), "unexpected value"))
        }
        Type::EnumVariant(_, k) => if json.is_string() {
            let e = main_namespace.enum_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            if e.cache.member_names.contains_str(json.as_str().unwrap()) {
                Ok(Value::String(json.as_str().unwrap().to_owned()))
            } else {
                Err(Error::value_error(path.clone(), "expect enum member"))
            }
        } else {
            Err(Error::value_error(path.clone(), "expect string represents enum member"))
        }
        Type::InterfaceObject(_, gens, k) => {
            let i = main_namespace.interface_at_path(&k.iter().map(AsRef::as_ref).collect()).unwrap();
            let shapes = collect_interface_shapes(i, gens, main_namespace);
            json_to_teon_with_shapes(json, path, shapes, main_namespace)
        }
        Type::ModelObject(_, _) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::StructObject(_, _) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::ModelScalarFields(_, _) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::ModelScalarFieldsWithoutVirtuals(_, _) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::ModelScalarFieldsAndCachedPropertiesWithoutVirtuals(_, _) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::ModelRelations(_, _) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::ModelDirectRelations(_, _) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::FieldType(_, _) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::FieldReference(_) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::GenericItem(_) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::Keyword(_) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::Optional(inner) => {
            if json.is_null() {
                Ok(Value::Null)
            } else {
                json_to_teon_with_type(json, path, inner.as_ref(), main_namespace)
            }
        }
        Type::Pipeline(_) => Err(Error::value_error(path.clone(), "unexpected type"))?,
        Type::ShapeReference(shape_reference) => {
            let input = fetch_input(shape_reference, main_namespace).unwrap();
            json_to_teon(json, path, input.as_ref(), main_namespace)
        }
    }
}

pub fn json_to_teon_with_shapes(json: &serde_json::Value, path: &KeyPath, shapes: Vec<&Shape>, main_namespace: &Namespace) -> crate::path::Result<Value> {
    if let Some(object) = json.as_object() {
        let combined = if shapes.len() == 1 {
            Cow::Borrowed(*shapes.first().unwrap())
        } else {
            Cow::Owned(shapes.iter().fold(Shape::new(indexmap! {}), |mut item1, item2| {
                item1.extend((*item2).clone().into_iter());
                item1
            }))
        };
        let required_keys: BTreeSet<&str> = combined.as_ref().iter().filter_map(|(k, v)| if !v.is_optional() {
            Some(k.as_str())
        } else {
            None
        }).collect();
        let all_keys: BTreeSet<&str> = combined.as_ref().keys().map(AsRef::as_ref).collect();
        let passed_in_keys: BTreeSet<&str> = object.keys().map(AsRef::as_ref).collect();
        let unallowed_keys: Vec<&str> = passed_in_keys.difference(&all_keys).map(|s| *s).collect();
        if let Some(unallowed) = unallowed_keys.first() {
            return Err(Error::value_error(path + *unallowed, "unexpected key"));
        }
        let not_provided_keys: Vec<&str> = required_keys.difference(&passed_in_keys).map(|s| *s).collect();
        if let Some(not_provided) = not_provided_keys.first() {
            return Err(Error::value_error(path + *not_provided, "expect value"));
        }
        let map: IndexMap<String, Value> = object.iter().map(|(k, v)| Ok((k.to_owned(), json_to_teon(v, &(path + k), combined.as_ref().get(k).unwrap(), main_namespace)?))).collect::<crate::path::Result<IndexMap<String, Value>>>()?;
        Ok(Value::Dictionary(map))
    } else {
        Err(Error::value_error(path.clone(), "unexpected value"))
    }

}

pub fn json_to_teon(json: &serde_json::Value, path: &KeyPath, input: &Input, main_namespace: &Namespace) -> crate::path::Result<Value> {
    match input {
        Input::Undetermined => Ok(Value::from(json)),
        Input::Or(inputs) => {
            for i in inputs {
                if let Ok(result) = json_to_teon(json, path, i, main_namespace) {
                    return Ok(result);
                }
            }
            Err(Error::value_error(path.clone(), "unexpected value"))
        }
        Input::Shape(shape) => {
            json_to_teon_with_shapes(json, path, vec![shape], main_namespace)
        }
        Input::Type(t) => {
            json_to_teon_with_type(json, path, t, main_namespace)
        }
        Input::SynthesizedEnum(e) => {
            if let Some(str) = json.as_str() {
                if e.members.keys().contains(&str.to_owned()) {
                    Ok(Value::String(str.to_owned()))
                } else {
                    Err(Error::value_error(path.clone(), "unexpected value"))
                }
            } else {
                Err(Error::value_error(path.clone(), "unexpected value"))
            }
        }
    }
}

fn collect_interface_shapes<'a>(interface: &'a Interface, gens: &Vec<Type>, namespace: &'a Namespace) -> Vec<&'a Shape> {
    let mut result = vec![];
    let shape = interface.cache.shape.map.get(gens).unwrap().as_shape().unwrap();
    result.push(shape);
    let map = calculate_generics_map(&interface.generic_names, gens);
    for extend in &interface.extends {
        if let Some((_, inner_gens, path)) = extend.as_interface_object() {
            let inner_interface = namespace.interface_at_path(&path.iter().map(AsRef::as_ref).collect()).unwrap();
            result.extend(collect_interface_shapes(inner_interface, &inner_gens.iter().map(|t| t.replace_generics(&map)).collect(), namespace));
        }
    }
    result
}

fn calculate_generics_map(
    generics_names: &Vec<String>,
    types: &Vec<Type>,
) -> BTreeMap<String, Type> {
    if generics_names.len() == types.len() {
        return generics_names.iter().enumerate().map(|(index, identifier)| (identifier.to_owned(), types.get(index).unwrap().clone())).collect();
    }
    btreemap!{}
}
