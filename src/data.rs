
use crate::non_empty::NonEmpty;
use crate::Source;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Currency {
    USD,
    BTC,
    LBTC,
    Other(String)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair((Currency, Currency));

impl Pair {
    pub fn new(c1: Currency, c2: Currency) -> Pair {
        Pair((c1, c2))
    }

    pub fn new_btc(c: Currency) -> Pair {
        Pair((Currency::BTC, c))
    }

    pub fn first(&self) -> &Currency {
        &(self.0).0
    }

    pub fn second(&self) -> &Currency {
        &(self.0).1
    }
}

#[derive(Debug)]
pub struct Ticker {
    pub pair: Pair,
    pub rate: f64
}

#[derive(Debug)]
pub struct Response {
    pub source_name: String,
    pub rates: NonEmpty<Ticker>
}

#[derive(Debug)]
pub struct BuildRequest {
    pub pairs: NonEmpty<Pair>
}

impl BuildRequest {
    pub fn from_pair(pair: Pair) -> BuildRequest {
        BuildRequest {
            pairs: pair.into()
        }
    }
}

pub struct PreparedRequest<'a> {
    pub http_request: http::Request<Vec<u8>>,
    pub pairs: NonEmpty<Pair>,
    pub source: &'a dyn Source
}
