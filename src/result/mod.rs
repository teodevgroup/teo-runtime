use super::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) trait ResultExt<T> {

    fn err_prefix(self, prefix: impl AsRef<str>) -> Result<T>;
}

impl<T> ResultExt<T> for std::result::Result<T, Error> {

    fn err_prefix(self, prefix: impl AsRef<str>) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.prefix(prefix)),
        }
    }
}

impl<T> ResultExt<T> for std::result::Result<T, teo_teon::error::Error> {

    fn err_prefix(self, prefix: impl AsRef<str>) -> Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::from(e).prefix(prefix)),
        }
    }
}