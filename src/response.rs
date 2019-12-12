
use crate::non_empty::NonEmpty;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ticker {
    USD,
    Other(String)
}

#[derive(Debug)]
pub struct TickerResponse {
    pub ticker: Ticker,
    pub rate: f64
}

#[derive(Debug)]
pub struct Response {
    pub source_name: String,
    pub rates: NonEmpty<TickerResponse>
}
