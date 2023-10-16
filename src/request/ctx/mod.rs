use teo_teon::Value;
use crate::action::Action;
use crate::request::Request;
use super::local::Data;

#[derive(Debug)]
pub struct ReqCtx {
    pub req: Request,
    pub body: Value,
    // pub connection: Arc<dyn Connection>,
    // pub(crate) path_components: PathComponents,
    //pub action: Option<Action>,
    pub req_local: Data,
}