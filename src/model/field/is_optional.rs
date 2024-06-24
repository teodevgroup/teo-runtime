pub trait IsOptional {

    fn is_optional(&self) -> bool;

    fn is_required(&self) -> bool;
}