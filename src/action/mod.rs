use std::ops::{BitAnd, BitOr, BitXor, Neg, Not};
use serde::Serialize;
use crate::error::Error;
use crate::result::Result;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Action(u32);

// impl BitOr for Action {
//
//     type Output = Action;
//
//     const fn bitor(self, rhs: Self) -> Self::Output {
//         Self(self.0 | rhs.0)
//     }
// }
//
// impl BitAnd for Action {
//
//     type Output = Action;
//
//     const fn bitand(self, rhs: Self) -> Self::Output {
//         Self(self.0 & rhs.0)
//     }
// }
//
// impl BitXor for Action {
//
//     type Output = Action;
//
//     const fn bitxor(self, rhs: Self) -> Self::Output {
//         Self(self.0 & rhs.0)
//     }
// }
//
// impl Not for Action {
//
//     type Output = Action;
//
//     const fn not(self) -> Self::Output {
//         Self(!self.0)
//     }
// }
//
// impl Neg for Action {
//
//     type Output = Action;
//
//     fn neg(self) -> Self::Output {
//         let restore_name_bits = !self.contains_name_bits();
//         let restore_entry_nested_bits = !self.contains_position_bits();
//         let restore_single_many_bits = !self.contains_amount_bits();
//         let mut value = !self;
//         if restore_name_bits {
//             value = value & NOT_ALL_NAMES;
//         }
//         if restore_entry_nested_bits {
//             value = value & NOT_ENTRY_NESTED;
//         }
//         if restore_single_many_bits {
//             value = value & NOT_SINGLE_MANY;
//         }
//         value
//     }
// }
//
// pub(crate) const CREATE: Action = Action(1);
// pub(crate) const UPDATE: Action = Action(1 << 1);
// pub(crate) const DELETE: Action = Action(1 << 2);
// pub(crate) const COPY: Action = Action(1 << 3);
// pub(crate) const FIND: Action = Action(1 << 4);
// pub(crate) const FIRST: Action = Action(1 << 5);
// pub(crate) const CONNECT: Action = Action(1 << 6);
// pub(crate) const DISCONNECT: Action = Action(1 << 7);
// pub(crate) const SET: Action = Action(1 << 8);
// pub(crate) const JOIN: Action = Action(1 << 9);
// pub(crate) const COUNT: Action = Action(1 << 10);
// pub(crate) const AGGREGATE: Action = Action(1 << 11);
// pub(crate) const GROUP_BY: Action = Action(1 << 12);
// pub(crate) const CODE_NAME: Action = Action(1 << 13);
//
// pub(crate) const UPSERT: Action = CREATE | UPDATE;
// pub(crate) const CONNECT_OR_CREATE: Action = CONNECT | CREATE;
// pub(crate) const JOIN_CREATE: Action = JOIN | CREATE;
// pub(crate) const JOIN_DELETE: Action = JOIN | DELETE;
// pub(crate) const FIND_FIRST: Action = FIND | FIRST;
//
// pub(crate) const ENTRY: Action = Action(1 << 14);
// pub(crate) const NESTED: Action = Action(1 << 15);
// pub(crate) const CODE_POSITION: Action = Action(1 << 16);
//
// pub(crate) const SINGLE: Action = Action(1 << 17);
// pub(crate) const MANY: Action = Action(1 << 18);
// pub(crate) const CODE_AMOUNT: Action = Action(1 << 19);
//
// const ALL_NAMES: Action = CREATE | UPDATE | UPSERT | DELETE | COPY | FIND | FIND_FIRST | CONNECT | CONNECT_OR_CREATE | DISCONNECT | SET | JOIN_CREATE | JOIN_DELETE | COUNT | AGGREGATE | GROUP_BY | CODE_NAME;
// const ALL_POSITIONS: Action = ENTRY | NESTED | CODE_POSITION;
// const ALL_AMOUNTS: Action = SINGLE | MANY | CODE_AMOUNT;
//
// const NOT_ALL_NAMES: Action = !ALL_NAMES;
// const NOT_ENTRY_NESTED: Action = !ALL_POSITIONS;
// const NOT_SINGLE_MANY: Action = !ALL_AMOUNTS;
//
// pub(crate) const FIND_UNIQUE_HANDLER: Action = FIND | ENTRY | SINGLE;
// pub(crate) const FIND_FIRST_HANDLER: Action = FIND_FIRST | ENTRY | SINGLE;
// pub(crate) const FIND_MANY_HANDLER: Action = FIND | ENTRY | MANY;
// pub(crate) const CREATE_HANDLER: Action = CREATE | ENTRY | SINGLE;
// pub(crate) const UPDATE_HANDLER: Action = UPDATE | ENTRY | SINGLE;
// pub(crate) const UPSERT_HANDLER: Action = UPSERT | ENTRY | SINGLE;
// pub(crate) const DELETE_HANDLER: Action = DELETE | ENTRY | SINGLE;
// pub(crate) const CREATE_MANY_HANDLER: Action = CREATE | ENTRY | MANY;
// pub(crate) const UPDATE_MANY_HANDLER: Action = UPDATE | ENTRY | MANY;
// pub(crate) const DELETE_MANY_HANDLER: Action = DELETE | ENTRY | MANY;
// pub(crate) const COUNT_HANDLER: Action = COUNT | ENTRY;
// pub(crate) const AGGREGATE_HANDLER: Action = AGGREGATE | ENTRY;
// pub(crate) const GROUP_BY_HANDLER: Action = GROUP_BY | ENTRY;
//
// pub(crate) const NESTED_CREATE_ACTION: Action = CREATE | NESTED | SINGLE;
// pub(crate) const NESTED_UPDATE_ACTION: Action = UPDATE | NESTED | SINGLE;
// pub(crate) const NESTED_UPSERT_ACTION: Action = UPSERT | NESTED | SINGLE;
// pub(crate) const NESTED_DELETE_ACTION: Action = DELETE | NESTED | SINGLE;
// pub(crate) const NESTED_COPY_ACTION: Action = COPY | NESTED | SINGLE;
// pub(crate) const NESTED_CONNECT_OR_CREATE_ACTION: Action = CONNECT_OR_CREATE | NESTED | SINGLE;
// pub(crate) const NESTED_CONNECT_ACTION: Action = CONNECT | NESTED | SINGLE;
// pub(crate) const NESTED_DISCONNECT_ACTION: Action = DISCONNECT | NESTED | SINGLE;
// pub(crate) const NESTED_SET_ACTION: Action = SET | NESTED | SINGLE;
// pub(crate) const NESTED_CREATE_MANY_ACTION: Action = CREATE | NESTED | MANY;
// pub(crate) const NESTED_UPDATE_MANY_ACTION: Action = UPDATE | NESTED | MANY;
// pub(crate) const NESTED_DELETE_MANY_ACTION: Action = DELETE | NESTED | MANY;
//
// impl Default for Action {
//
//     fn default() -> Self {
//         Self::empty()
//     }
// }
//
// impl Action {
//
//     pub(crate) fn empty() -> Self {
//         Self(0)
//     }
//
//     pub(crate) fn is_empty(&self) -> bool {
//         self.0 == 0
//     }
//
//     pub(crate) fn from_name(name: &str) -> Result<Self> {
//         Ok(match name {
//             "create" => CREATE,
//             "update" => UPDATE,
//             "delete" => DELETE,
//             "copy" => COPY,
//             "find" => FIND,
//             "connect" => CONNECT,
//             "disconnect" => DISCONNECT,
//             "set" => SET,
//             "join" => JOIN,
//             "first" => FIRST,
//             "count" => COUNT,
//             "aggregate" => AGGREGATE,
//             "groupBy" => GROUP_BY,
//             "codeName" => CODE_NAME,
//             "entry" => ENTRY,
//             "nested" => NESTED,
//             "codePosition" => CODE_POSITION,
//             "single" => SINGLE,
//             "many" => MANY,
//             "codeAmount" => CODE_AMOUNT,
//             _ => Err(Error::new(format!("Unrecognized action name '{}'", name)))?
//         })
//     }
//
//     pub(crate) fn nested_from_name(name: &str) -> Result<Self> {
//         Ok(match name {
//             "create" => NESTED_CREATE_ACTION,
//             "update" => NESTED_UPDATE_ACTION,
//             "upsert" => NESTED_UPSERT_ACTION,
//             "delete" => NESTED_DELETE_ACTION,
//             "copy" => NESTED_COPY_ACTION,
//             "connect" => NESTED_CONNECT_ACTION,
//             "connectOrCreate" => NESTED_CONNECT_OR_CREATE_ACTION,
//             "disconnect" => NESTED_DISCONNECT_ACTION,
//             "set" => NESTED_SET_ACTION,
//             "createMany" => NESTED_CREATE_ACTION,
//             "updateMany" => NESTED_UPDATE_MANY_ACTION,
//             "deleteMany" => NESTED_DELETE_MANY_ACTION,
//             _ => Err(Error::new(format!("Unrecognized nested action name '{}'", name)))?
//         })
//     }
//
//     pub(crate) fn finalized(&self) -> Self {
//         let mut value = *self;
//         if !self.contains_name_bits() {
//             value = value | ALL_NAMES;
//         }
//         if !self.contains_position_bits() {
//             value = value | ALL_POSITIONS;
//         }
//         if !self.contains_amount_bits() {
//             value = value | ALL_AMOUNTS;
//         }
//         value
//     }
//
//     pub(crate) fn redirect(&self, action: Action) -> Self {
//         let mut result = *self;
//         let new_names_bits = action & ALL_NAMES;
//         if new_names_bits.0 != 0 {
//             result = (result & !ALL_NAMES) | new_names_bits;
//         }
//         let new_position_bits = action & ALL_POSITIONS;
//         if new_position_bits.0 != 0 {
//             result = (result & !ALL_POSITIONS) | new_position_bits;
//         }
//         let new_amount_bits = action & ALL_AMOUNTS;
//         if new_amount_bits.0 != 0 {
//             result = (result & !ALL_AMOUNTS) | new_amount_bits;
//         }
//         result
//     }
//
//     #[inline(always)]
//     fn contains_name_bits(&self) -> bool {
//         (*self & ALL_NAMES).0 != 0
//     }
//
//     #[inline(always)]
//     fn contains_position_bits(&self) -> bool {
//         (*self & ALL_POSITIONS).0 != 0
//     }
//
//     #[inline(always)]
//     fn contains_amount_bits(&self) -> bool {
//         (*self & ALL_AMOUNTS).0 != 0
//     }
//
//     pub(crate) fn passes(&self, matchers: &Vec<Action>) -> bool {
//         for matcher in matchers {
//             let copy = self.finalized();
//             let finalized_matcher = matcher.finalized();
//             let result = finalized_matcher & copy;
//             return ((result & ALL_NAMES).0 != 0) &&
//                 ((result & ALL_POSITIONS).0 != 0) &&
//                 ((result & ALL_AMOUNTS).0 != 0)
//         }
//         false
//     }
// }
//
// impl From<u32> for Action {
//
//     fn from(value: u32) -> Self {
//         Self(value)
//     }
// }
//
// impl From<Action> for u32 {
//
//     fn from(value: Action) -> Self {
//         value.0
//     }
// }