
use crate::non_empty::NonEmpty;
use crate::Ticker;
use crate::Source;

#[derive(Debug)]
pub struct BuildRequest {
    pub tickers: NonEmpty<Ticker>
}

impl BuildRequest {
    pub fn from_ticker(ticker: Ticker) -> BuildRequest {
        BuildRequest {
            tickers: ticker.into()
        }
    }
}

pub struct PreparedRequest<'a> {
    pub http_request: http::Request<Vec<u8>>,
    pub tickers: NonEmpty<Ticker>,
    pub source: &'a dyn Source
}
