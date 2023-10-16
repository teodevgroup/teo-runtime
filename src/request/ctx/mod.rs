use std::cell::{Ref, RefCell, RefMut};
use teo_teon::Value;
use crate::action::Action;
use crate::request::Request;
use super::local::Data;

#[derive(Debug)]
pub struct Ctx {
    request: Request,
    body: Value,
    // pub connection: Arc<dyn Connection>,
    // pub(crate) path_components: PathComponents,
    //pub action: Option<Action>,
    data: RefCell<Data>,
}

impl Ctx {

    pub fn request(&self) -> &Request {
        &self.request
    }

    pub fn data(&self) -> Ref<Data> {
        self.data.borrow()
    }

    pub fn data_mut(&self) -> RefMut<Data> {
        self.data.borrow_mut()
    }
}

unsafe impl Send for Ctx {}
