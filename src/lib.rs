//! # CoinDesk API Wrapper
//!
//! `ccdata-api` is a wrapper for CoinDesk REST API endpoints (Formerly CCData). This crate supports non-exhausitve list of CoinDesk endpoints,
//! you can check what endpoints are supported by checking the variants of the enum `CCAPIEndpoint` - it contains all
//! supported endpoint URLs.
//!
//! For documentation on CoinDesk REST API endpoints visit CoinDesk online documentation:
//! - API Documentation: <https://developers.coindesk.com/documentation/data-api/introduction>
//! - Legacy API Documentation: <https://developers.coindesk.com/documentation/legacy/Price/SingleSymbolPriceEndpoint>
//!
//! **Disclaimer:** This crate is an unofficial CoinDesk REST API wrapper, the maintainers of the crate are independent developers.
//! The developers of the crate do not accept any responsibility or liability for the accuracy, security, or completeness of the code,
//! or the information provided within the crate.
//!
//! # Errors
//!
//! The REST API functions in the crate will error if the data received does not fit into the pre-defined schemas provided
//! in the crate. If you encounter any errors, please open an issue on GitHub with the parameters that you have used (e.g., asset symbol,
//! timestamp, limit, etc.). **Do not provide your API key or any personal data!**
//! 
//! # Deprecated API Endpoints
//! 
//! Some API endpoints have been deprecated by CoinDesk, who strongly recommend migrating to newer alternatives suggested in their API documentation.
//! 
//! # Features
//! - `debug`: If this feature is enabled, you can set `CCDATA_API_DEBUG` environment variable to `true`, which will print the response body
//! for every request to the command line.
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
//! assert_eq!(backend.api_key().unwrap(), "xxxxxxx");
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
//! use ccdata_api::{CCData, CCUnit, CCSpotMarket};
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
//!     let to_timestamp: Option<i64> = Some(1728860400);
//!
//!     // Make the API call
//!     let ohlcv = backend.get_spot_ohlcv("BTC-USD", to_timestamp, Some(limit), market, CCUnit::Day).await.unwrap();
//!     assert_eq!(ohlcv.data.unwrap().len(), limit);
//!
//! }
//! ```
//!
//! # General information
//! If you would like to add a commit or an issue, please do so using the GitHub link to the project:
//! - <https://github.com/rsadykhov/ccdata-api>


// Re-Exports
pub use self::backend::CCData;
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


pub mod error;
pub mod schemas;
pub mod utils;
pub mod backend;


/// Unit of the interval between successive data points.
pub enum CCUnit {
    /// Daily data interval
    Day,
    /// Hourly data interval
    Hour,
    /// Minutely data interval
    Minute,
    /// Used for API endpoints that do not have a specified unit for data or where unit is unapplicable
    NA,
}


/// All supported API endpoints.
pub enum CCAPIEndpoint {
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    // Min-API
    /// Min-API Endpoint
    ///
    /// Description: Returns a list of all coins for which CoinDesk currently gets the blockchain data
    ///
    /// URL: https://min-api.cryptocompare.com/data/blockchain/list
    AvailableCoinList,
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    /// Min-API Endpoint
    ///
    /// Description: Retrieves the daily aggregated blockchain data for the requested coin
    ///
    /// URL: https://min-api.cryptocompare.com/data/blockchain/histo/day
    HistoricalDaily,
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    /// Min-API Endpoint
    ///
    /// Description: Retrieves the balance distribution for a specified asset over a specified time range at a daily interval
    ///
    /// Note: Only data for BTC (Bitcoin) is currently available
    ///
    /// URL: https://min-api.cryptocompare.com/data/blockchain/balancedistribution/histo/day
    BalanceDistribution,
    // Data-API
    // Indices & Reference Rates
    /// Data-API Endpoint
    ///
    /// Description: Provides historical candlestick data for various indices
    ///
    /// URL: https://data-api.coindesk.com/index/cc/v1/historical
    IndicesOHLCV,
    // Spot
    /// Data-API Endpoint
    ///
    /// Description: Provides candlestick data for specific cryptocurrency instruments across selected exchanges
    ///
    /// URL: https://data-api.coindesk.com/spot/v1/historical
    SpotOHLCV,
    /// Data-API Endpoint
    ///
    /// Description: Delivers vital metadata about financial instruments traded on specified exchanges, focusing solely on non-price related information
    ///
    /// URL: https://data-api.coindesk.com/spot/v1/latest/instrument/metadata
    SpotInstrumentMetadata,
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    /// Data-API Endpoint
    ///
    /// Description: Provides comprehensive information about various cryptocurrency spot markets
    ///
    /// URL: https://data-api.coindesk.com/spot/v1/markets
    SpotMarkets,
    /// Data-API Endpoint
    ///
    /// Description: Retrieves a comprehensive dictionary of mapped instruments across one or more spot markets, filtered by a specified state or status
    ///
    /// URL: https://data-api.coindesk.com/spot/v1/markets/instruments
    SpotMarketsInstruments,
    // Futures
    /// Data-API Endpoint
    ///
    /// Description: Provides aggregated candlestick data for specific futures instruments on designated exchanges
    ///
    /// URL: https://data-api.coindesk.com/futures/v1/historical
    FuturesOHLCV,
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    /// Data-API Endpoint
    ///
    /// Description: Provides comprehensive information about various cryptocurrency futures markets
    ///
    /// URL: https://data-api.coindesk.com/futures/v1/markets
    FuturesMarkets,
    // Options
    /// Data-API Endpoint
    ///
    /// Description: Provides historical OHLCV (open, high, low, close, volume) data for specified options instruments on a chosen exchange
    ///
    /// URL: https://data-api.coindesk.com/options/v1/historical
    OptionsOHLCV,
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    /// Data-API Endpoint
    ///
    /// Description: Provides comprehensive information about the various options markets integrated by our platform
    ///
    /// URL: https://data-api.coindesk.com/options/v1/markets
    OptionsMarkets,
    // Derivatives Indices
    /// Data-API Endpoint
    ///
    /// Description: Provides historical OHLC (open, high, low, close) data for specified index instruments on a selected market
    ///
    /// URL: https://data-api.coindesk.com/index/v1/historical
    DerIndicesOHLCV,
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    /// Data-API Endpoint
    ///
    /// Description: Provides comprehensive information about various derivatives index markets
    ///
    /// URL: https://data-api.coindesk.com/index/v1/markets
    DerIndicesMarkets,
    // On-Chain DEX
    /// Data-API Endpoint
    ///
    /// Description: Retrieves aggregated candlestick data for AMM swap transactions
    ///
    /// URL: https://data-api.coindesk.com/onchain/v1/amm/historical/swap
    OCDEXOHLCV,
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    /// Data-API Endpoint
    ///
    /// Description: Provides comprehensive information about various decentralized exchange (DEX) markets within the blockchain ecosystem
    ///
    /// URL: https://data-api.coindesk.com/onchain/v1/amm/markets
    OCDEXMarkets,
    // On-Chain Core
    /// Data-API Endpoint
    ///
    /// Description: Delivers exhaustive details on a specific Ethereum block in a meticulously processed format, complete with detailed explanations for each field
    ///
    /// URL: https://data-api.coindesk.com/onchain/v1/block/2
    OCCoreETHBlocks,
    /// Data-API Endpoint
    ///
    /// Description: Retrieves a comprehensive summary of chain asset information for a specified blockchain, identified by its chain symbol
    ///
    /// URL: https://data-api.coindesk.com/onchain/v3/summary/by/chain
    OCCoreAssetsByChain,
    /// Data-API Endpoint
    ///
    /// Description: Retrieves comprehensive asset information for a specific asset identified by its smart contract address and associated blockchain asset
    ///
    /// URL: https://data-api.coindesk.com/onchain/v2/data/by/address
    OCCoreAssetByAddress,
    /// Data-API Endpoint
    ///
    /// Description: Retrieves comprehensive historical supply data for various digital assets identified by either their CoinDesk asset ID or unique asset symbol
    ///
    /// URL: https://data-api.coindesk.com/onchain/v2/historical/supply/days
    OCCoreSupply,
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    // Asset
    /// Data-API Endpoint
    ///
    /// Description: Returns an object that provides detailed and comprehensive information about multiple cryptocurrency assets in response to a request
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/metadata
    AssetMetadata,
    /// Data-API Endpoint
    /// 
    /// Descriptions: Returns an object that provides detailed and comprehensive information about multiple cryptocurrency assets in response to a request
    /// 
    /// URL: https://data-api.coindesk.com/asset/v2/metadata
    AssetMetadataV2,
    /// Data-API Endpoint
    ///
    /// Description: Retrieves an array of significant events related to digital assets, such as security incidents, rebrandings, blockchain forks, and other impactful developments
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/events
    AssetEvents,
    /// Data-API Endpoint
    ///
    /// Description: Provides an in-depth, daily snapshot of a digital asset's code repositories
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/code-repository/days
    AssetCodeRepo,
    /// Data-API Endpoint
    ///
    /// Description: Aggregates detailed daily metrics from all Discord servers related to a specific digital asset, offering a multifaceted view into community engagement and the asset's standing within Discord communities
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/discord/days
    AssetDiscord,
    /// Data-API Endpoint
    ///
    /// Description: Aggregates key performance indicators from all the subreddits related to a specific digital asset, providing a comprehensive understanding of the asset's footprint on Reddit
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/reddit/days
    AssetReddit,
    /// Data-API Endpoint
    ///
    /// Description: Collates essential data points across all Telegram groups affiliated with a particular cryptocurrency asset
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/telegram/days
    AssetTelegram,
    /// Data-API Endpoint
    ///
    /// Description: Aggregates essential metrics from all X (Twitter) accounts associated with a specific cryptocurrency asset
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/twitter/days
    AssetTwitter,
    // News
    /// Data-API Endpoint
    ///
    /// Description: Serves as the pulse of the crypto news landscape, providing users with instant access to the most recent articles across the industry
    ///
    /// URL: https://data-api.coindesk.com/news/v1/article/list
    NewsLatestArticles,
    /// Data-API Endpoint
    ///
    /// Description: Offers a comprehensive listing of all news sources available through CoinDesk API
    ///
    /// URL: https://data-api.coindesk.com/news/v1/source/list
    NewsSources,
    /// Data-API Endpoint
    ///
    /// Description: Provide a straightforward listing of all news categories available through CoinDesk API
    ///
    /// URL: https://data-api.coindesk.com/news/v1/category/list
    NewsCategories,
    // Overview
    /// Data-API Endpoint
    ///
    /// Description: Presents a thorough historical daily overview of market capitalisation for digital assets that meet the volume and listing criteria
    ///
    /// URL: https://data-api.coindesk.com/overview/v1/historical/marketcap/all/assets/days
    OverviewMktCapOHLCV,
}

impl CCAPIEndpoint {
    fn resolve_url(&self) -> String {
        match self {
            // Min-API
            Self::AvailableCoinList => String::from("https://min-api.cryptocompare.com/data/blockchain/list"),
            Self::HistoricalDaily => String::from("https://min-api.cryptocompare.com/data/blockchain/histo/day"),
            Self::BalanceDistribution => String::from("https://min-api.cryptocompare.com/data/blockchain/balancedistribution/histo/day"),
            // Data-API
            // Indices & Reference Rates
            Self::IndicesOHLCV => String::from("https://data-api.coindesk.com/index/cc/v1/historical"),
            // Spot
            Self::SpotOHLCV => String::from("https://data-api.coindesk.com/spot/v1/historical"),
            Self::SpotInstrumentMetadata => String::from("https://data-api.coindesk.com/spot/v1/latest/instrument/metadata"),
            Self::SpotMarkets => String::from("https://data-api.coindesk.com/spot/v1/markets"),
            Self::SpotMarketsInstruments => String::from("https://data-api.coindesk.com/spot/v1/markets/instruments"),
            // Futures
            Self::FuturesOHLCV => String::from("https://data-api.coindesk.com/futures/v1/historical"),
            Self::FuturesMarkets => String::from("https://data-api.coindesk.com/futures/v1/markets"),
            // Options
            Self::OptionsOHLCV => String::from("https://data-api.coindesk.com/options/v1/historical"),
            Self::OptionsMarkets => String::from("https://data-api.coindesk.com/options/v1/markets"),
            // Derivatives Indices
            Self::DerIndicesOHLCV => String::from("https://data-api.coindesk.com/index/v1/historical"),
            Self::DerIndicesMarkets => String::from("https://data-api.coindesk.com/index/v1/markets"),
            // On-Chain DEX
            Self::OCDEXOHLCV => String::from("https://data-api.coindesk.com/onchain/v1/amm/historical/swap"),
            Self::OCDEXMarkets => String::from("https://data-api.coindesk.com/onchain/v1/amm/markets"),
            // On-Chain Core
            Self::OCCoreETHBlocks => String::from("https://data-api.coindesk.com/onchain/v1/block/2"),
            Self::OCCoreAssetsByChain => String::from("https://data-api.coindesk.com/onchain/v3/summary/by/chain"),
            Self::OCCoreAssetByAddress => String::from("https://data-api.coindesk.com/onchain/v2/data/by/address"),
            Self::OCCoreSupply => String::from("https://data-api.coindesk.com/onchain/v2/historical/supply/days"),
            // Asset
            Self::AssetMetadata => String::from("https://data-api.coindesk.com/asset/v1/metadata"),
            Self::AssetMetadataV2 => String::from("https://data-api.coindesk.com/asset/v2/metadata"),
            Self::AssetEvents => String::from("https://data-api.coindesk.com/asset/v1/events"),
            Self::AssetCodeRepo => String::from("https://data-api.coindesk.com/asset/v1/historical/code-repository/days"),
            Self::AssetDiscord => String::from("https://data-api.coindesk.com/asset/v1/historical/discord/days"),
            Self::AssetReddit => String::from("https://data-api.coindesk.com/asset/v1/historical/reddit/days"),
            Self::AssetTelegram => String::from("https://data-api.coindesk.com/asset/v1/historical/telegram/days"),
            Self::AssetTwitter => String::from("https://data-api.coindesk.com/asset/v1/historical/twitter/days"),
            // News
            Self::NewsLatestArticles => String::from("https://data-api.coindesk.com/news/v1/article/list"),
            Self::NewsSources => String::from("https://data-api.coindesk.com/news/v1/source/list"),
            Self::NewsCategories => String::from("https://data-api.coindesk.com/news/v1/category/list"),
            // Overview
            Self::OverviewMktCapOHLCV => String::from("https://data-api.coindesk.com/overview/v1/historical/marketcap/all/assets/days"),
        }
    }

    fn add_unit_to_url(&self, url: &mut String, unit: &CCUnit) -> () {
        match self {
            Self::IndicesOHLCV | Self::SpotOHLCV | Self::FuturesOHLCV |
            Self::OptionsOHLCV | Self::DerIndicesOHLCV | Self::OCDEXOHLCV => {
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
    ///
    /// # Input
    /// - `unit`: Unit of the interval between successive data points
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
        assert_eq!(api_endpoint.url(&unit), String::from("https://data-api.coindesk.com/index/cc/v1/historical/hours"));
    }
}