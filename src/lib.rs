//! # CCData API Wrapper
//!
//! `ccdata-api` is a wrapper for CCData REST API endpoints. This crate supports non-exhausitve list of CCData endpoints,
//! you can check what endpoints are supported by checking the variants of the enum `CCAPIEndpoint` - it contains all
//! supported endpoint URLs.
//!
//! For documentation on CCData REST API endpoints visit CCData online documentation:
//! - Min-API: <https://developers.ccdata.io/documentation/legacy/Price/SingleSymbolPriceEndpoint>
//! - Data-API: <https://developers.ccdata.io/documentation/data-api/introduction>
//!
//! **Disclaimer:** This crate is an unofficial CCData REST API wrapper, the maintainers of the crate are independent developers.
//! The developers of the crate do not accept any responsibility or liability for the accuracy, security, or completeness of the code,
//! or the information provided within the crate.
//!
//! # Errors
//!
//! The REST API functions in the crate will error if the data received does not fit into the pre-defined schemas provided
//! in the crate. In the case of missing fields, unexpected null values or unexpected data types, the error message will state:
//!
//! - `data did not match any variant of untagged enum EmptyObject`
//!
//! If you encounter this error, please open the issue on GitHub with the parameters that you have used (e.g., asset symbol,
//! timestamp, limit, etc.). **Do not provide your API key or any personal data!**
//!
//! # Examples
//!
//! To start making the REST API requests, define a data collection backend. This can be done by either directly passing an
//! API key to the data collection backend, or by defining a `.env` file with an API key as an environment variable (Preferred
//! method).
//!
//! ## Build Backend Explicitly Stating API Key (May expose API key)
//!
//! ```rust
//! use ccdata_api::CCData;
//!
//! let mut backend: CCData = CCData::new();
//!
//! let api_key: String = String::from("xxxxxxx");
//! backend.update_api_key(api_key);
//!
//! assert_eq!(backend.api_key().unwrap(), &String::from("xxxxxxx"));
//! ```
//!
//! ## Build Backend Using .env File (Preferred method)
//!
//! ```rust
//! use ccdata_api::CCData;
//!
//! let mut backend: CCData = CCData::new();
//! // Provide API key as the environment variable called API_KEY
//! backend.build(&"API_KEY").unwrap();
//!
//! println!("{}", backend.api_key().unwrap());
//! ```
//!
//! ## Making First API Call
//!
//! After the backend has been build, you can make API requests using the methods provided in the backend. For example, to
//! get a daily spot OHLCV data for Bitcoin you can do the following (note that the calls use Rust's `async` functionality):
//!
//! ```rust
//!
//! use ccdata_api::{CCData, CCUnit, DataUnwrap};
//! use ccdata_api::CCSpotMarket;
//!
//! #[tokio::main]
//! async fn main() -> () {
//!
//!     let mut backend: CCData = CCData::new();
//!     // Provide API key as the environment variable called API_KEY
//!     backend.build(&"API_KEY").unwrap();
//!
//!     // Define the API parameters
//!     let market: CCSpotMarket = CCSpotMarket::KRAKEN;
//!     let limit: usize = 2000;
//!     let to_timestamp: Option<i64> - Some(1728860400);
//!
//!     // Make the API call
//!     let ohlcv = backend.get_spot_ohlcv(&String::from("BTC-USD"), to_timestamp, Some(limit), market, CCUnit::Day).await.unwrap();
//!     assert_eq!(ohlcv.data_unwrap().unwrap().len(), limit);
//!
//! }
//! ```
//!
//! # General information
//! If you would like to add a commit or an issue, please do so using the GitHub link to the project:
//! - <https://github.com/rsadykhov/ccdata-api>


// Re-Exports
pub use self::backend::CCData;
pub use self::schemas::DataUnwrap;
// Min-API Re-Exports
pub use self::schemas::{CCMinResponse, CCMinWrapper, CCRateLimit, CCMaxCalls, CCCallsMade};
pub use self::schemas::min_api::{CCAvailableCoinList, CCHistoricalDaily, CCBalanceDistribution, CCSupplyBand};
// Data-API Re-Exports
pub use self::schemas::{CCDataResponse, CCError, CCErrorOtherInfo};
pub use self::schemas::data_api::indices_and_reference_rates::{CCIndicesMarket, CCIndicesOHLCV};
pub use self::schemas::data_api::spot::{CCSpotMarket, CCSpotInstrumentStatus, CCSpotOHLCV, CCSpotInstrumentMetdata, CCSpotMarkets, CCSpotMarketsInstruments};
pub use self::schemas::data_api::futures::{CCFuturesMarket, CCFuturesOHLCV, CCFuturesMarkets};
pub use self::schemas::data_api::options::{CCOptionsMarket, CCOptionsOHLCV, CCOptionsMarkets};
pub use self::schemas::data_api::derivatives_indices::{CCDerIndicesMarket, CCDerIndicesOHLCV, CCDerIndicesMarkets};
pub use self::schemas::data_api::on_chain_dex::{CCOCDEXMarket, CCOCDEXOHLCV, CCOCDEXMarkets};
pub use self::schemas::data_api::on_chain_core::{CCOCCoreETHBlock, CCOCCoreAssetByChain, CCOCCoreAssetByAddress, CCOCCoreSupply};
pub use self::schemas::data_api::asset::{CCAssetMetadata, CCAssetEvent, CCAssetCodeRepoMetrics, CCAssetDiscord, CCAssetReddit, CCAssetTelegram, CCAssetTwitter};
pub use self::schemas::data_api::news::{CCNewsStatus, CCNewsLang, CCNewsSourceID, CCNewsLatestArticle, CCNewsSourceType, CCNewsSource, CCNewsCategory};
pub use self::schemas::data_api::overview::CCOverviewMktCapOHLCV;


pub mod schemas;
pub mod utils;
pub mod backend;


/// Unit of historical data response.
pub enum CCUnit {
    Day,
    Hour,
    Minute,
    NA,
}


/// All supported API endpoints.
pub enum CCAPIEndpoint {
    // Min-API
    AvailableCoinList,
    HistoricalDaily,
    BalanceDistribution,
    // Data-API
    // Indices & Reference Rates
    IndicesOHLCV,
    // Spot
    SpotOHLCV,
    SpotInstrumentMetadata,
    SpotMarkets,
    SpotMarketsInstruments,
    // Futures
    FuturesOHLCV,
    FuturesMarkets,
    // Options
    OptionsOHLCV,
    OptionsMarkets,
    // Derivatives Indices
    DerIndicesOHLCV,
    DerIndicesMarkets,
    // On-Chain DEX
    OCDEXOHLCV,
    OCDEXMarkets,
    // On-Chain Core
    OCCoreETHBlocks,
    OCCoreAssetsByChain,
    OCCoreAssetByAddress,
    OCCoreSupply,
    // Asset
    AssetMetadata,
    AssetEvents,
    AssetCodeRepo,
    AssetDiscord,
    AssetReddit,
    AssetTelegram,
    AssetTwitter,
    // News
    NewsLatestArticles,
    NewsSources,
    NewsCategories,
    // Overview
    OverviewMktCapOHLCV,
}

impl CCAPIEndpoint {
    fn resolve_url(&self) -> String {
        match self {
            // Min-API
            CCAPIEndpoint::AvailableCoinList => String::from("https://min-api.cryptocompare.com/data/blockchain/list"),
            CCAPIEndpoint::HistoricalDaily => String::from("https://min-api.cryptocompare.com/data/blockchain/histo/day"),
            CCAPIEndpoint::BalanceDistribution => String::from("https://min-api.cryptocompare.com/data/blockchain/balancedistribution/histo/day"),
            // Data-API
            // Indices & Reference Rates
            CCAPIEndpoint::IndicesOHLCV => String::from("https://data-api.ccdata.io/index/cc/v1/historical"),
            // Spot
            CCAPIEndpoint::SpotOHLCV => String::from("https://data-api.ccdata.io/spot/v1/historical"),
            CCAPIEndpoint::SpotInstrumentMetadata => String::from("https://data-api.ccdata.io/spot/v1/latest/instrument/metadata"),
            CCAPIEndpoint::SpotMarkets => String::from("https://data-api.ccdata.io/spot/v1/markets"),
            CCAPIEndpoint::SpotMarketsInstruments => String::from("https://data-api.ccdata.io/spot/v1/markets/instruments"),
            // Futures
            CCAPIEndpoint::FuturesOHLCV => String::from("https://data-api.ccdata.io/futures/v1/historical"),
            CCAPIEndpoint::FuturesMarkets => String::from("https://data-api.ccdata.io/futures/v1/markets"),
            // Options
            CCAPIEndpoint::OptionsOHLCV => String::from("https://data-api.ccdata.io/options/v1/historical"),
            CCAPIEndpoint::OptionsMarkets => String::from("https://data-api.ccdata.io/options/v1/markets"),
            // Derivatives Indices
            CCAPIEndpoint::DerIndicesOHLCV => String::from("https://data-api.ccdata.io/index/v1/historical"),
            CCAPIEndpoint::DerIndicesMarkets => String::from("https://data-api.ccdata.io/index/v1/markets"),
            // On-Chain DEX
            CCAPIEndpoint::OCDEXOHLCV => String::from("https://data-api.ccdata.io/onchain/v1/amm/historical/swap"),
            CCAPIEndpoint::OCDEXMarkets => String::from("https://data-api.ccdata.io/onchain/v1/amm/markets"),
            // On-Chain Core
            CCAPIEndpoint::OCCoreETHBlocks => String::from("https://data-api.ccdata.io/onchain/v1/block/2"),
            CCAPIEndpoint::OCCoreAssetsByChain => String::from("https://data-api.ccdata.io/onchain/v3/summary/by/chain"),
            CCAPIEndpoint::OCCoreAssetByAddress => String::from("https://data-api.ccdata.io/onchain/v2/data/by/address"),
            CCAPIEndpoint::OCCoreSupply => String::from("https://data-api.ccdata.io/onchain/v2/historical/supply/days"),
            // Asset
            CCAPIEndpoint::AssetMetadata => String::from("https://data-api.ccdata.io/asset/v1/metadata"),
            CCAPIEndpoint::AssetEvents => String::from("https://data-api.ccdata.io/asset/v1/events"),
            CCAPIEndpoint::AssetCodeRepo => String::from("https://data-api.ccdata.io/asset/v1/historical/code-repository/days"),
            CCAPIEndpoint::AssetDiscord => String::from("https://data-api.ccdata.io/asset/v1/historical/discord/days"),
            CCAPIEndpoint::AssetReddit => String::from("https://data-api.ccdata.io/asset/v1/historical/reddit/days"),
            CCAPIEndpoint::AssetTelegram => String::from("https://data-api.ccdata.io/asset/v1/historical/telegram/days"),
            CCAPIEndpoint::AssetTwitter => String::from("https://data-api.ccdata.io/asset/v1/historical/twitter/days"),
            // News
            CCAPIEndpoint::NewsLatestArticles => String::from("https://data-api.ccdata.io/news/v1/article/list"),
            CCAPIEndpoint::NewsSources => String::from("https://data-api.ccdata.io/news/v1/source/list"),
            CCAPIEndpoint::NewsCategories => String::from("https://data-api.ccdata.io/news/v1/category/list"),
            // Overview
            CCAPIEndpoint::OverviewMktCapOHLCV => String::from("https://data-api.ccdata.io/overview/v1/historical/marketcap/all/assets/days"),
        }
    }

    fn add_unit_to_url(&self, url: &mut String, unit: &CCUnit) -> () {
        match self {
            CCAPIEndpoint::IndicesOHLCV | CCAPIEndpoint::SpotOHLCV | CCAPIEndpoint::FuturesOHLCV |
            CCAPIEndpoint::OptionsOHLCV | CCAPIEndpoint::DerIndicesOHLCV | CCAPIEndpoint::OCDEXOHLCV => {
                match unit {
                    CCUnit::Day | CCUnit::NA => url.push_str(&"/days"),
                    CCUnit::Hour => url.push_str(&"/hours"),
                    CCUnit::Minute => url.push_str(&"/minutes"),
                }
            },
            _ => (),
        }
    }

    /// Produces URL for a given API endpoint.
    pub fn url(&self, unit: &CCUnit) -> String {
        let mut url: String = self.resolve_url();
        self.add_unit_to_url(&mut url, unit);
        url
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn unit_test_add_unit_to_url() -> () {
        use crate::{CCUnit, CCAPIEndpoint};
        let unit: CCUnit = CCUnit::Hour;
        let api_endpoint: CCAPIEndpoint = CCAPIEndpoint::IndicesOHLCV;
        assert_eq!(api_endpoint.url(&unit), String::from("https://data-api.ccdata.io/index/cc/v1/historical/hours"));
    }
}