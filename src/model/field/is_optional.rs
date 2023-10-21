pub trait IsOptionalMut {

    fn set_optional(&mut self);

    fn set_required(&mut self);
}

pub trait IsOptional {

    fn is_optional(&self);

    fn is_required(&self);
}