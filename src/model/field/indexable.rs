use crate::model::field;
use crate::traits::named::Named;

pub trait Indexable: Named {

    fn index(&self) -> Option<&field::Index>;

    fn set_index(&mut self, index: field::Index);
}
