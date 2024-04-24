use crate::action::Action;
use crate::action::action::*;

pub fn builtin_action_handler_from_name(name: &str) -> Option<Action> {
    Some(match name {
        "findUnique" => FIND_UNIQUE_HANDLER,
        "findFirst" => FIND_FIRST_HANDLER,
        "findMany" => FIND_MANY_HANDLER,
        "create" => CREATE_HANDLER,
        "update" => UPDATE_HANDLER,
        "upsert" => UPSERT_HANDLER,
        "delete" => DELETE_HANDLER,
        "copy" => COPY_HANDLER,
        "createMany" => CREATE_MANY_HANDLER,
        "updateMany" => UPDATE_MANY_HANDLER,
        "deleteMany" => DELETE_MANY_HANDLER,
        "copyMany" => COPY_MANY_HANDLER,
        "count" => COUNT_HANDLER,
        "aggregate" => AGGREGATE_HANDLER,
        "groupBy" => GROUP_BY_HANDLER,
        _ => None?
    })
}