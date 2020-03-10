
# rust-bitcoin-exchange-rates

[![Build Status](https://travis-ci.com/jb55/rust-bitcoin-exchange-rates.svg?branch=master)](https://travis-ci.com/jb55/rust-bitcoin-exchange-rates)

A simple library from fetching exchange rates from a list of sources. New
sources can be added via the `Source` trait.

## Basic usage

This is the first pass at what you could call an api, it works like so:


```rust
// wasabi has an exchange api, so we provide a source for that
let wasabi = Wasabi::new("https://wasabiwallet.io");

// prepare a list of of sources given a ticker (just USD for now).
// any tickers not supported by a given source won't be added to the prepared
// requests list
let reqs = prepare_requests(vec![&wasabi], Ticker::USD);

// example hyper request. hyper is optional, you can use any library that can
// execute http::Requests

let ssl = NativeTlsClient::new().unwrap();

// use hyper_socks connector here instead for Tor support!
let connector = hyper::net::HttpsConnector::new(ssl);
let client = hyper::Client::with_connector(connector);

// the current implementation will return the first working result
// you could shuffle the request list to avoid getting rate limited
let res = hyper_fetch_requests(&client, &reqs);

assert!(req.is_ok());
```


return data:

```
Response {
    source_name: "wasabi",
    rates: NonEmpty(
        [
            TickerResponse {
                ticker: USD,
                rate: 7166.69,
            },
        ],
    )
}
```
