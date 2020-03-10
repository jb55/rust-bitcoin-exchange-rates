
use serde::Deserialize;

use crate::non_empty::NonEmpty;
use crate::error::Error;
use crate::sources::Source;
use crate::Result;
use crate::data::{Response, Ticker, Currency, BuildRequest, PreparedRequest, Pair};

pub struct Wasabi {
    endpoint: String
}

#[derive(Deserialize)]
pub struct WasabiTicker {
    pub ticker: String,
    pub rate: f64,
}

impl From<WasabiTicker> for Ticker {
    fn from(x: WasabiTicker) -> Self {
        Ticker {
            pair: Pair::new(Currency::BTC, parse_wasabi_currency(x.ticker)),
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


fn parse_wasabi_currency(s: String) -> Currency {
    if s == "USD" {
        return Currency::USD
    }

    Currency::Other(s)
}


impl Source for Wasabi {
    fn name(&self) -> &str {
        "wasabi"
    }

    fn build_request(&self, req: BuildRequest) -> Result<PreparedRequest> {
        let expected_pair = Pair::new(Currency::BTC, Currency::USD);

        if !req.pairs.get_vec().contains(&expected_pair) {
            // wasabi only supports BTCUSD at this time
            return Err(Error::UnsupportedPairs(req.pairs))
        }

        let request = http::Request::builder()
            .method("GET")
            .uri(String::from(&self.endpoint) + "/api/v3/btc/Offchain/exchange-rates")
            .body(vec![])
            .unwrap();

        Ok(PreparedRequest {
            http_request: request,
            pairs: expected_pair.into(),
            source: self
        })
    }

    fn parse_response(&self, res: &[u8]) -> Result<Response> {
        let values : Vec<WasabiTicker> = serde_json::from_slice(res)?;
        let tvalues : Vec<Ticker> = values.into_iter().map(|x| x.into()).collect();
        let rates = NonEmpty::new_or_err(tvalues, Error::InvalidResponse)?;
        let source_name = self.name().into();

        Ok(Response { rates, source_name })
    }
}
