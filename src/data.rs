
use crate::non_empty::NonEmpty;
// use log::{debug, info};
use crate::Source;
use std::fmt;
use crate::error::Error;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Currency {
    BTC,
    USD,
    CAD,
    // LBTC,
    Other(String)
}

impl std::str::FromStr for Currency {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Error> {
        // println!("currency from_str {}", s);
        if s.len() < 3 {
            return Err(Error::InvalidResponse("ticker length less than 3".into()))
        }

        // TODO: support harder to parse pairs (LBTC?)
        match s {
            "USD" => Ok(Currency::USD),
            "CAD" => Ok(Currency::CAD),
            "BTC" => Ok(Currency::BTC),
            ""    => Err(Error::InvalidResponse("empty ticker".into())),
            other => Ok(Currency::Other(other.into()))
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s =
            match self {
                Currency::USD => "USD",
                Currency::CAD => "CAD",
                Currency::BTC => "BTC",
                // Currency::LBTC => "LBTC",
                Currency::Other(ref s) => s,
            };
        write!(f, "{}", s)
    }
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

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.first(), self.second())
    }
}

#[derive(Debug, Clone)]
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
