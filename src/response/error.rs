use crate::response::Response;

pub trait IntoResponseWithPathedError {
    fn into_response_with_pathed_error(self) -> crate::path::Result<Response>;
}

impl IntoResponseWithPathedError for crate::path::Result<Response> {

    fn into_response_with_pathed_error(self) -> crate::path::Result<Response> {
        self
    }
}

impl IntoResponseWithPathedError for teo_result::Result<Response> {

    fn into_response_with_pathed_error(self) -> crate::path::Result<Response> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into()),
        }
    }
}