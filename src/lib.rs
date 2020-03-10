
pub use http;

#[cfg(feature = "hyper")]
use std::io::Read;


mod non_empty;
mod error;
mod sources;
mod data;

pub use crate::non_empty::NonEmpty;
pub use crate::error::Error;
pub use crate::data::{Response, Ticker, BuildRequest, PreparedRequest, Pair, Currency};
pub use crate::sources::{Source, Wasabi};

pub type Result<T> = std::result::Result<T, Error>;


pub fn ticker_request(pair: Pair) -> BuildRequest {
    BuildRequest::from_pair(pair)
}

pub fn btc_ticker_request(currency: Currency) -> BuildRequest {
    ticker_request(Pair::new(Currency::BTC, currency))
}

pub fn prepare_requests(sources: Vec<&dyn Source>, pair: Pair) -> Vec<PreparedRequest>
{
    let mut prepared : Vec<PreparedRequest> = vec![];

    for source in sources {
        let req = source.build_request(ticker_request(pair.clone()));
        if let Ok(prep) = req {
            prepared.push(prep);
        }
    }

    prepared
}


#[cfg(feature = "hyper")]
pub fn hyper_fetch_requests(client: &hyper::Client, reqs: &[PreparedRequest]) -> Option<Response> {
    let mut buffer = Vec::new();

    for req in reqs {
        let mres = make_old_hyper_req(client, &req.http_request)
            .and_then(|mut res| res.read_to_end(&mut buffer).map_err(|err| err.into()))
            .and_then(|_res| req.source.parse_response(&buffer));

        // TODO: return when we've satisfied all requests
        match mres {
            Ok(res) => return Some(res),
            Err(err) => print!("err {:#?}", err)
        }
    }

    None
}


#[cfg(feature = "hyper")]
fn make_old_hyper_req(client: &hyper::Client, req: &http::Request<Vec<u8>>)
                      -> Result<hyper::client::Response> {
    let url : String = req.uri().to_string();
    let converted_uri = hyper::Url::parse(&url).map_err(|_err| Error::Other("failed to convert uri".into()))?;

    let old_req =
        match *req.method() {
            http::method::Method::GET => client.get(converted_uri),
            http::method::Method::POST => client.post(converted_uri),
            _ => unimplemented!("implement more method conversions for old hyper")
        };

    let body = &*req.body();
    old_req.body(&body[..]).send().map_err(|err| err.into())
}


#[cfg(test)]
mod tests {
    use crate::*;
    use std::assert;
    #[cfg(feature = "hyper-native-tls")]
    use hyper_native_tls::NativeTlsClient;

    #[test]
    #[cfg(feature = "hyper-native-tls")]
    fn it_works() {
        //let tor_endpoint = "http://wasabiukrxmkdgve5kynjztuovbg43uxcbcxn6y2okcrsg7gb6jdmbad.onion/";
        let endpoint = "https://wasabiwallet.io";
        let wasabi = Wasabi::new(endpoint);

        let req = wasabi.build_request(btc_ticker_request(Currency::Other("CAD".into())));
        assert!(req.is_err(), "Expected CAD to be unsupported by Wasabi");

        let req = wasabi.build_request(btc_ticker_request(Currency::USD));
        assert!(req.is_ok());

        let reqs = prepare_requests(vec![&wasabi], Pair::new_btc(Currency::USD));
        let ssl = NativeTlsClient::new().unwrap();
        let connector = hyper::net::HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);
        let res = hyper_fetch_requests(&client, &reqs);
        assert!(res.is_some());
        let res = res.unwrap();

        print!("{:#?}", res);
    }
}

