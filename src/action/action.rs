use std::ops::{BitAnd, BitOr, BitXor, Neg, Not};
use serde::Serialize;
use super::const_values::*;
use teo_result::Error;
use teo_result::Result;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Action(pub u32);

impl BitOr for Action {

    type Output = Action;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for Action {

    type Output = Action;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXor for Action {

    type Output = Action;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Not for Action {

    type Output = Action;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Neg for Action {

    type Output = Action;

    fn neg(self) -> Self::Output {
        let restore_name_bits = !self.contains_name_bits();
        let restore_entry_nested_bits = !self.contains_position_bits();
        let restore_single_many_bits = !self.contains_amount_bits();
        let mut value = !self;
        if restore_name_bits {
            value = value & NOT_ALL_NAMES;
        }
        if restore_entry_nested_bits {
            value = value & NOT_ENTRY_NESTED;
        }
        if restore_single_many_bits {
            value = value & NOT_SINGLE_MANY;
        }
        value
    }
}

pub const CREATE: Action = Action(CREATE_U32);
pub const UPDATE: Action = Action(UPDATE_U32);
pub const DELETE: Action = Action(DELETE_U32);
pub const COPY: Action = Action(COPY_U32);
pub const FIND: Action = Action(FIND_U32);
pub const FIRST: Action = Action(FIRST_U32);
pub const CONNECT: Action = Action(CONNECT_U32);
pub const DISCONNECT: Action = Action(DISCONNECT_U32);
pub const SET: Action = Action(SET_U32);
pub const JOIN: Action = Action(JOIN_U32);
pub const COUNT: Action = Action(COUNT_U32);
pub const AGGREGATE: Action = Action(AGGREGATE_U32);
pub const GROUP_BY: Action = Action(GROUP_BY_U32);
pub const CODE_NAME: Action = Action(CODE_NAME_U32);

pub const UPSERT: Action = Action(UPSERT_U32);
pub const CONNECT_OR_CREATE: Action = Action(CONNECT_OR_CREATE_U32);
pub const JOIN_CREATE: Action = Action(JOIN_CREATE_U32);
pub const JOIN_DELETE: Action = Action(JOIN_DELETE_U32);
pub const FIND_FIRST: Action = Action(FIND_FIRST_U32);

pub const ENTRY: Action = Action(ENTRY_U32);
pub const NESTED: Action = Action(NESTED_U32);
pub const CODE_POSITION: Action = Action(CODE_POSITION_U32);

pub const SINGLE: Action = Action(SINGLE_U32);
pub const MANY: Action = Action(MANY_U32);
pub const CODE_AMOUNT: Action = Action(CODE_AMOUNT_U32);

const ALL_NAMES: Action = Action(ALL_NAMES_U32);
const ALL_POSITIONS: Action = Action(ALL_POSITIONS_U32);
const ALL_AMOUNTS: Action = Action(ALL_AMOUNTS_U32);

const NOT_ALL_NAMES: Action = Action(NOT_ALL_NAMES_U32);
const NOT_ENTRY_NESTED: Action = Action(NOT_ENTRY_NESTED_U32);
const NOT_SINGLE_MANY: Action = Action(NOT_SINGLE_MANY_U32);

pub(crate) const FIND_UNIQUE_HANDLER: Action = Action(FIND_UNIQUE_HANDLER_U32);
pub(crate) const FIND_FIRST_HANDLER: Action = Action(FIND_FIRST_HANDLER_U32);
pub(crate) const FIND_MANY_HANDLER: Action = Action(FIND_MANY_HANDLER_U32);
pub(crate) const CREATE_HANDLER: Action = Action(CREATE_HANDLER_U32);
pub(crate) const UPDATE_HANDLER: Action = Action(UPDATE_HANDLER_U32);
pub(crate) const COPY_HANDLER: Action = Action(COPY_HANDLER_U32);
pub(crate) const UPSERT_HANDLER: Action = Action(UPSERT_HANDLER_U32);
pub(crate) const DELETE_HANDLER: Action = Action(DELETE_HANDLER_U32);
pub(crate) const CREATE_MANY_HANDLER: Action = Action(CREATE_MANY_HANDLER_U32);
pub(crate) const UPDATE_MANY_HANDLER: Action = Action(UPDATE_MANY_HANDLER_U32);
pub(crate) const COPY_MANY_HANDLER: Action = Action(COPY_MANY_HANDLER_U32);
pub(crate) const DELETE_MANY_HANDLER: Action = Action(DELETE_MANY_HANDLER_U32);
pub(crate) const COUNT_HANDLER: Action = Action(COUNT_HANDLER_U32);
pub(crate) const AGGREGATE_HANDLER: Action = Action(AGGREGATE_HANDLER_U32);
pub(crate) const GROUP_BY_HANDLER: Action = Action(GROUP_BY_HANDLER_U32);

pub(crate) const NESTED_CREATE_ACTION: Action = Action(NESTED_CREATE_ACTION_U32);
pub(crate) const NESTED_UPDATE_ACTION: Action = Action(NESTED_UPDATE_ACTION_U32);
pub(crate) const NESTED_UPSERT_ACTION: Action = Action(NESTED_UPSERT_ACTION_U32);
pub(crate) const NESTED_DELETE_ACTION: Action = Action(NESTED_DELETE_ACTION_U32);
pub(crate) const NESTED_COPY_ACTION: Action = Action(NESTED_COPY_ACTION_U32);
pub(crate) const NESTED_CONNECT_OR_CREATE_ACTION: Action = Action(NESTED_CONNECT_OR_CREATE_ACTION_U32);
pub(crate) const NESTED_CONNECT_ACTION: Action = Action(NESTED_CONNECT_ACTION_U32);
pub(crate) const NESTED_DISCONNECT_ACTION: Action = Action(NESTED_DISCONNECT_ACTION_U32);
pub(crate) const NESTED_SET_ACTION: Action = Action(NESTED_SET_ACTION_U32);
pub(crate) const NESTED_CREATE_MANY_ACTION: Action = Action(NESTED_CREATE_MANY_ACTION_U32);
pub(crate) const NESTED_UPDATE_MANY_ACTION: Action = Action(NESTED_UPDATE_MANY_ACTION_U32);
pub(crate) const NESTED_DELETE_MANY_ACTION: Action = Action(NESTED_DELETE_MANY_ACTION_U32);

impl Default for Action {

    fn default() -> Self {
        Self::empty()
    }
}

impl Action {

    pub(crate) fn empty() -> Self {
        Self(0)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub(crate) fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "create" => CREATE,
            "update" => UPDATE,
            "delete" => DELETE,
            "copy" => COPY,
            "find" => FIND,
            "connect" => CONNECT,
            "disconnect" => DISCONNECT,
            "set" => SET,
            "join" => JOIN,
            "first" => FIRST,
            "count" => COUNT,
            "aggregate" => AGGREGATE,
            "groupBy" => GROUP_BY,
            "codeName" => CODE_NAME,
            "entry" => ENTRY,
            "nested" => NESTED,
            "codePosition" => CODE_POSITION,
            "single" => SINGLE,
            "many" => MANY,
            "codeAmount" => CODE_AMOUNT,
            _ => Err(Error::new(format!("Unrecognized action name '{}'", name)))?
        })
    }

    pub(crate) fn nested_from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "create" => NESTED_CREATE_ACTION,
            "update" => NESTED_UPDATE_ACTION,
            "upsert" => NESTED_UPSERT_ACTION,
            "delete" => NESTED_DELETE_ACTION,
            "copy" => NESTED_COPY_ACTION,
            "connect" => NESTED_CONNECT_ACTION,
            "connectOrCreate" => NESTED_CONNECT_OR_CREATE_ACTION,
            "disconnect" => NESTED_DISCONNECT_ACTION,
            "set" => NESTED_SET_ACTION,
            "createMany" => NESTED_CREATE_ACTION,
            "updateMany" => NESTED_UPDATE_MANY_ACTION,
            "deleteMany" => NESTED_DELETE_MANY_ACTION,
            _ => Err(Error::new(format!("Unrecognized nested action name '{}'", name)))?
        })
    }

    pub(crate) fn finalized(&self) -> Self {
        let mut value = *self;
        if !self.contains_name_bits() {
            value = value | ALL_NAMES;
        }
        if !self.contains_position_bits() {
            value = value | ALL_POSITIONS;
        }
        if !self.contains_amount_bits() {
            value = value | ALL_AMOUNTS;
        }
        value
    }

    pub(crate) fn redirect(&self, action: Action) -> Self {
        let mut result = *self;
        let new_names_bits = action & ALL_NAMES;
        if new_names_bits.0 != 0 {
            result = (result & !ALL_NAMES) | new_names_bits;
        }
        let new_position_bits = action & ALL_POSITIONS;
        if new_position_bits.0 != 0 {
            result = (result & !ALL_POSITIONS) | new_position_bits;
        }
        let new_amount_bits = action & ALL_AMOUNTS;
        if new_amount_bits.0 != 0 {
            result = (result & !ALL_AMOUNTS) | new_amount_bits;
        }
        result
    }

    #[inline(always)]
    fn contains_name_bits(&self) -> bool {
        (*self & ALL_NAMES).0 != 0
    }

    #[inline(always)]
    fn contains_position_bits(&self) -> bool {
        (*self & ALL_POSITIONS).0 != 0
    }

    #[inline(always)]
    fn contains_amount_bits(&self) -> bool {
        (*self & ALL_AMOUNTS).0 != 0
    }

    pub(crate) fn passes(&self, matchers: &Vec<Action>) -> bool {
        for matcher in matchers {
            let copy = self.finalized();
            let finalized_matcher = matcher.finalized();
            let result = finalized_matcher & copy;
            return ((result & ALL_NAMES).0 != 0) &&
                ((result & ALL_POSITIONS).0 != 0) &&
                ((result & ALL_AMOUNTS).0 != 0)
        }
        false
    }
}

impl From<u32> for Action {

    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Action> for u32 {

    fn from(value: Action) -> Self {
        value.0
    }
}

impl From<i32> for Action {

    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}

impl From<Action> for i32 {

    fn from(value: Action) -> Self {
        value.0 as i32
    }
}