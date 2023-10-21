use crate::model::field;
use crate::model::field::named::Named;

pub trait Indexable: Named {

    fn index(&self) -> Option<&field::Index>;
}

pub trait IndexableMut {

    fn set_index(&mut self, index: field::Index);
}