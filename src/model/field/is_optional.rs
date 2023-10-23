pub trait IsOptional {

    fn is_optional(&self) -> bool;

    fn is_required(&self) -> bool;

    fn set_optional(&mut self);

    fn set_required(&mut self);
}