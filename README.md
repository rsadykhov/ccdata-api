# CCData API Wrapper

`ccdata-api` is a wrapper for CCData REST API endpoints. This crate supports non-exhausitve list of CCData endpoints,
you can check what endpoints are supported by checking the variants of the enum `CCAPIEndpoint` - it contains all
supported endpoint URLs.

For documentation on CCData REST API endpoints visit CCData online documentation:
- Min-API: https://developers.ccdata.io/documentation/legacy/Price/SingleSymbolPriceEndpoint
- Data-API: https://developers.ccdata.io/documentation/data-api/introduction

**Disclaimer:** This crate is an unofficial CCData REST API wrapper, the maintainers of the crate are independent developers.
The developers of the crate do not accept any responsibility or liability for the accuracy, security, or completeness of the code,
or the information provided within the crate.

# Errors

The REST API functions in the crate will error if the data received does not fit into the pre-defined schemas provided
in the crate. In the case of missing fields, unexpected null values or unexpected data types, the error message will state:

- `data did not match any variant of untagged enum EmptyObject`

If you encounter this error, please open the issue on GitHub with the parameters that you have used (e.g., asset symbol,
timestamp, limit, etc.). **Do not provide your API key or any personal data!**

# Examples

To start making the REST API requests, define a data collection backend. This can be done by either directly passing an
API key to the data collection backend, or by defining a `.env` file with an API key as an environment variable (Preferred
method).

## Build Backend Explicitly Stating API Key (May expose API key)

```rust
use ccdata_api::CCData;

let mut backend: CCData = CCData::new();

let api_key: String = String::from("xxxxxxx");
backend.update_api_key(api_key);

assert_eq!(backend.api_key().unwrap(), &String::from("xxxxxxx"));
```

## Build Backend Using .env File (Preferred method)

```rust
use ccdata_api::CCData;

let mut backend: CCData = CCData::new();
// Provide API key as the environment variable called API_KEY
backend.build(&"API_KEY").unwrap();

println!("{}", backend.api_key().unwrap());
```

## Making First API Call

After the backend has been build, you can make API requests using the methods provided in the backend. For example, to
get a daily spot OHLCV data for Bitcoin you can do the following (note that the calls use Rust's `async` functionality):

```rust

use ccdata_api::{CCData, CCUnit, DataUnwrap};
use ccdata_api::CCSpotMarket;

#[tokio::main]
async fn main() -> () {

    let mut backend: CCData = CCData::new();
    // Provide API key as the environment variable called API_KEY
    backend.build(&"API_KEY").unwrap();

    // Define the API parameters
    let market: CCSpotMarket = CCSpotMarket::KRAKEN;
    let limit: usize = 2000;
    let to_timestamp: Option<i64> - Some(1728860400);

    // Make the API call
    let ohlcv = backend.get_spot_ohlcv(&String::from("BTC-USD"), to_timestamp, Some(limit), market, CCUnit::Day).await.unwrap();
    assert_eq!(ohlcv.data_unwrap().unwrap().len(), limit);

}
```

# General information
If you would like to add a commit or an issue, please do so using the GitHub link to the project:
- <https://github.com/rsadykhov/ccdata-api>