
use serde::Deserialize;

use crate::non_empty::NonEmpty;
use crate::error::Error;
use crate::sources::Source;
use crate::Result;
use crate::response::{Response, Ticker, TickerResponse};
use crate::request::{BuildRequest, PreparedRequest};

pub struct Wasabi {
    endpoint: String
}

#[derive(Deserialize)]
pub struct WasabiTicker {
    pub ticker: String,
    pub rate: f64,
}

impl From<WasabiTicker> for TickerResponse {
    fn from(x: WasabiTicker) -> Self {
        TickerResponse{
            ticker: parse_wasabi_ticker(x.ticker),
            rate: x.rate
        }
    }
}


impl Wasabi {
    pub fn new(endpoint: &str) -> Wasabi {
        Wasabi {
            endpoint: String::from(endpoint)
        }
    }
}


fn parse_wasabi_ticker(s: String) -> Ticker {
    if s == "USD" {
        return Ticker::USD
    }

    Ticker::Other(s)
}


impl Source for Wasabi {
    fn name(&self) -> &str {
        "wasabi"
    }

    fn build_request(&self, req: BuildRequest) -> Result<PreparedRequest> {
        if !req.tickers.get_vec().contains(&Ticker::USD) {
            // wasabi only support USD at this time
            return Err(Error::UnsupportedTickers(req.tickers))
        }

        let request = http::Request::builder()
            .method("GET")
            .uri(String::from(&self.endpoint) + "/api/v3/btc/Offchain/exchange-rates")
            .body(vec![])
            .unwrap();

        Ok(PreparedRequest {
            http_request: request,
            tickers: Ticker::USD.into(),
            source: self
        })
    }

    fn parse_response(&self, res: &[u8]) -> Result<Response> {
        let values : Vec<WasabiTicker> = serde_json::from_slice(res)?;
        let tvalues : Vec<TickerResponse> = values.into_iter().map(|x| x.into()).collect();
        let rates = NonEmpty::new_or_err(tvalues, Error::InvalidResponse)?;
        let source_name = self.name().into();

        Ok(Response { rates, source_name })
    }
}
