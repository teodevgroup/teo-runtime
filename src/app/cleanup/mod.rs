use crate::app::data::AppData;

pub trait Cleanup {
    fn call(&self, app_data: AppData) -> ();
}

impl<F> Cleanup for F where F: Fn(AppData) -> () {
    fn call(&self, app_data: AppData) -> () {
        self(app_data)
    }
}
