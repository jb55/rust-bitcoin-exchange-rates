
use crate::Ticker;
use crate::non_empty::NonEmpty;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    #[cfg(feature = "hyper")]
    Hyper(hyper::error::Error),
    Io(std::io::Error),
    Uri(http::uri::InvalidUri),
    InvalidResponse,
    UnsupportedTickers(NonEmpty<Ticker>),
    Other(String),
}

impl From<http::uri::InvalidUri> for Error {
    fn from(uri_err: http::uri::InvalidUri) -> Self {
        Error::Uri(uri_err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(json_err: serde_json::Error) -> Self {
        Error::Json(json_err)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(hyper_err: hyper::error::Error) -> Self {
        Error::Hyper(hyper_err)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Self {
        Error::Io(io_err)
    }
}
