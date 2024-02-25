use crate::response::Response;

pub trait IntoResponseWithPathedError {
    fn into_response_with_pathed_error(self) -> teo_result::Result<Response>;
}

impl IntoResponseWithPathedError for teo_result::Result<Response> {

    fn into_response_with_pathed_error(self) -> teo_result::Result<Response> {
        self
    }
}
