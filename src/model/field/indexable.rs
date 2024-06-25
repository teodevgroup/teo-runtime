use crate::model::field;
use crate::traits::named::Named;

pub trait Indexable: Named {

    fn index(&self) -> Option<&field::Index>;
}

pub trait SetIndex {
    fn set_index(&self, index: field::Index);
}
