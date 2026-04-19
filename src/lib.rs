//! # CoinDesk API Wrapper
//!
//! `ccdata-api` is a wrapper for CoinDesk REST API endpoints (Formerly CCData). This crate supports non-exhausitve list of CoinDesk endpoints,
//! you can check what endpoints are supported by checking the variants of the enum `APIEndpoint` - it contains all
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
//! use ccdata_api::CoinDesk;
//!
//! let mut backend: CoinDesk = CoinDesk::new();
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
//! use ccdata_api::CoinDesk;
//!
//! let mut backend: CoinDesk = CoinDesk::new();
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
//! use ccdata_api::{CoinDesk, Unit, SpotMarket};
//!
//! #[tokio::main]
//! async fn main() -> () {
//!
//!     let mut backend: CoinDesk = CoinDesk::new();
//!     // Provide API key as the environment variable called API_KEY
//!     backend.build(&"API_KEY").unwrap();
//!
//!     // Define the API parameters
//!     let market: SpotMarket = SpotMarket::KRAKEN;
//!     let limit: usize = 2000;
//!     let to_timestamp: Option<i64> = Some(1728860400);
//!
//!     // Make the API call
//!     let ohlcv = backend.get_spot_ohlcv("BTC-USD", to_timestamp, Some(limit), market, Unit::Day).await.unwrap();
//!     assert_eq!(ohlcv.data.unwrap().len(), limit);
//!
//! }
//! ```
//!
//! # General information
//! If you would like to add a commit or an issue, please do so using the GitHub link to the project:
//! - <https://github.com/rsadykhov/ccdata-api>


// Re-Exports
pub use self::backend::CoinDesk;
pub use self::utils::{Group, AssetLookupPriority, Param, call_api_endpoint};
// Min-API Re-Exports
pub use self::schemas::{CCMinResponse, CCMinWrapper, CCRateLimit, CCMaxCalls, CCCallsMade};
pub use self::schemas::min_api::{BalanceDistribution, SupplyBand};
// Data-API Re-Exports
pub use self::schemas::{CoinDeskResponse, CCError, CCErrorOtherInfo};
pub use self::schemas::data_api::indices_and_reference_rates::{IndicesMarket, IndicesOHLCV};
pub use self::schemas::data_api::spot::{SpotMarket, SpotInstrumentStatus, SpotOHLCV, SpotInstrumentMetdata, SpotMarkets, SpotMarketsInstruments};
pub use self::schemas::data_api::futures::{FuturesMarket, FuturesOHLCV, FuturesInstrumentMetadata, FuturesMarkets};
pub use self::schemas::data_api::options::{OptionsMarket, OptionsOHLCV, OptionsInstrumentMetadata, OptionsMarkets};
pub use self::schemas::data_api::derivatives_indices::{DerIndicesMarket, DerIndicesOHLCV, DerIndicesMarkets};
pub use self::schemas::data_api::on_chain_dex::{OCDEXMarket, OCDEXOHLCV, OCDEXMarkets};
pub use self::schemas::data_api::on_chain_core::{OCCoreETHBlock, OCCoreAssetByChain, OCCoreAssetByAddress, OCCoreSupply};
pub use self::schemas::data_api::asset::{AssetMetadata, AssetEvent, AssetCodeRepoMetrics, AssetDiscord, AssetReddit, AssetTelegram, AssetTwitter};
pub use self::schemas::data_api::news::{NewsStatus, NewsLang, NewsSourceID, NewsLatestArticle, NewsSourceType, NewsSource, NewsCategory};
pub use self::schemas::data_api::overview::OverviewMktCapOHLCV;


pub mod error;
pub mod schemas;
pub mod utils;
pub mod backend;


use std::fmt::Display;
use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
/// Unit of the interval between successive data points.
pub enum Unit {
    #[default]
    /// Daily data interval
    Day,
    /// Hourly data interval
    Hour,
    /// Minutely data interval
    Minute,
    /// Used for API endpoints that do not have a specified unit for data or where unit is unapplicable
    NA,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Day | Unit::NA => write!(f, "/days"),
            Self::Hour => write!(f, "/hours"),
            Self::Minute => write!(f, "/minutes"),
        }
    }
}


/// Trait that defines all required methods for API endpoint URL construction (prior to adding non-default parameters).
/// This is useful for defining custom API endpoints that are not included with the crate.
pub trait APIEndpointTrait {
    /// Returns a vector of default groups to apply to match the pre-defined schemas.
    /// 
    /// Note: If there are no default groups defined the `None` variant is returned.
    fn default_groups(&self) -> Option<Vec<Group>>;

    /// Returns a vector of default API endpoint parameters to apply to match the pre-defined schemas.
    /// 
    ///  Note: If there are no default parameters defined the `None` variant is returned.
    fn default_params(&self) -> Option<Vec<Param<'_>>>;

    /// Produces URL for a given API endpoint.
    ///
    /// # Input
    /// - `unit`: Unit of the interval between successive data points
    fn url(&self, unit: &Unit) -> String;
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// All supported API endpoints.
pub enum APIEndpoint {
    #[deprecated(since="1.0.6", note="Deprecated by CoinDesk")]
    // Legacy API Endpoint
    /// Description: Retrieves the balance distribution for a specified asset over a specified time range at a daily interval
    ///
    /// Note: This is a legacy API endpoint and is not actively supported. Only data for BTC (Bitcoin) is available.
    ///
    /// URL: https://min-api.cryptocompare.com/data/blockchain/balancedistribution/histo/day
    BalanceDistribution,
    // Indices & Reference Rates
    ///
    /// Description: Provides historical candlestick data for various indices
    ///
    /// URL: https://data-api.coindesk.com/index/cc/v1/historical
    IndicesOHLCV,
    // Spot
    /// Description: Provides candlestick data for specific cryptocurrency instruments across selected exchanges
    ///
    /// URL: https://data-api.coindesk.com/spot/v1/historical
    SpotOHLCV,
    /// Description: Delivers vital metadata about financial instruments traded on specified exchanges, focusing solely on non-price related information
    ///
    /// URL: https://data-api.coindesk.com/spot/v1/latest/instrument/metadata
    SpotInstrumentMetadata,
    /// Description: Provides comprehensive information about various cryptocurrency spot markets,
    /// featuring extensive exchange metadata and operational details.
    /// 
    /// URL: https://data-api.coindesk.com/spot/v2/markets
    SpotMarketsV2,
    /// Description: Retrieves a comprehensive dictionary of mapped instruments across one or more spot markets, filtered by a specified state or status
    ///
    /// URL: https://data-api.coindesk.com/spot/v1/markets/instruments
    SpotMarketsInstruments,
    // Futures
    /// Description: Provides aggregated candlestick data for specific futures instruments on designated exchanges
    ///
    /// URL: https://data-api.coindesk.com/futures/v1/historical
    FuturesOHLCV,
    /// Description: Provides essential metadata about futures instruments traded on various exchanges.
    /// 
    /// URL: https://data-api.coindesk.com/futures/v1/latest/instrument/metadata
    FuturesInstrumentMetadata,
    /// Description: Provides comprehensive information about various cryptocurrency futures markets,
    /// featuring extensive exchange metadata and derivatives-specific operational details.
    /// 
    /// URL: https://data-api.coindesk.com/futures/v2/markets
    FuturesMarketsV2,
    // Options
    /// Description: Provides historical OHLCV (open, high, low, close, volume) data for specified options instruments on a chosen exchange
    ///
    /// URL: https://data-api.coindesk.com/options/v1/historical
    OptionsOHLCV,
    /// Description: Provides detailed metadata about options instruments across various exchanges.
    /// 
    /// URL: https://data-api.coindesk.com/options/v1/latest/instrument/metadata
    OptionsInstrumentMetadata,
    /// Description: Provides comprehensive information about various cryptocurrency options markets,
    /// featuring extensive exchange metadata and derivatives-specific operational details.
    /// 
    /// URL: https://data-api.coindesk.com/options/v2/markets
    OptionsMarketsV2,
    // Derivatives Indices
    /// Description: Provides historical OHLC (open, high, low, close) data for specified index instruments on a selected market
    ///
    /// URL: https://data-api.coindesk.com/index/v1/historical
    DerIndicesOHLCV,
    /// Description: Provides comprehensive information about various derivatives index markets,
    /// featuring extensive exchange metadata and index-specific operational details.
    /// 
    /// URL: https://data-api.coindesk.com/index/v2/markets
    DerIndicesMarketsV2,
    // On-Chain DEX
    /// Description: Retrieves aggregated candlestick data for AMM swap transactions
    ///
    /// URL: https://data-api.coindesk.com/onchain/v1/amm/historical/swap
    OCDEXOHLCV,
    /// Description: Provides comprehensive information about various on-chain decentralized exchange markets,
    /// featuring extensive exchange metadata and DEX-specific operational details.
    /// 
    /// URL: https://data-api.coindesk.com/onchain/v2/amm/markets
    OCDEXMarketsV2,
    // On-Chain Core
    /// Description: Delivers exhaustive details on a specific Ethereum block in a meticulously processed format, complete with detailed explanations for each field
    ///
    /// URL: https://data-api.coindesk.com/onchain/v1/block/2
    OCCoreETHBlocks,
    /// Description: Retrieves a comprehensive summary of chain asset information for a specified blockchain, identified by its chain symbol
    ///
    /// URL: https://data-api.coindesk.com/onchain/v3/summary/by/chain
    OCCoreAssetsByChain,
    /// Description: Retrieves comprehensive asset information for a specific asset identified by its smart contract address and associated blockchain asset
    ///
    /// URL: https://data-api.coindesk.com/onchain/v2/data/by/address
    OCCoreAssetByAddress,
    /// Description: Retrieves comprehensive historical supply data for various digital assets identified by either their CoinDesk asset ID or unique asset symbol
    ///
    /// URL: https://data-api.coindesk.com/onchain/v2/historical/supply/days
    OCCoreSupply,
    /// Descriptions: Returns an object that provides detailed and comprehensive information about multiple cryptocurrency assets in response to a request
    /// 
    /// URL: https://data-api.coindesk.com/asset/v2/metadata
    AssetMetadataV2,
    /// Description: Retrieves an array of significant events related to digital assets, such as security incidents, rebrandings, blockchain forks, and other impactful developments
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/events
    AssetEvents,
    /// Description: Provides an in-depth, daily snapshot of a digital asset's code repositories
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/code-repository/days
    AssetCodeRepo,
    /// Description: Aggregates detailed daily metrics from all Discord servers related to a specific digital asset, offering a multifaceted view into community engagement and the asset's standing within Discord communities
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/discord/days
    AssetDiscord,
    /// Description: Aggregates key performance indicators from all the subreddits related to a specific digital asset, providing a comprehensive understanding of the asset's footprint on Reddit
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/reddit/days
    AssetReddit,
    /// Description: Collates essential data points across all Telegram groups affiliated with a particular cryptocurrency asset
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/telegram/days
    AssetTelegram,
    /// Description: Aggregates essential metrics from all X (Twitter) accounts associated with a specific cryptocurrency asset
    ///
    /// URL: https://data-api.coindesk.com/asset/v1/historical/twitter/days
    AssetTwitter,
    // News
    /// Description: Serves as the pulse of the crypto news landscape, providing users with instant access to the most recent articles across the industry
    ///
    /// URL: https://data-api.coindesk.com/news/v1/article/list
    NewsLatestArticles,
    /// Description: Offers a comprehensive listing of all news sources available through CoinDesk API
    ///
    /// URL: https://data-api.coindesk.com/news/v1/source/list
    NewsSources,
    /// Description: Provide a straightforward listing of all news categories available through CoinDesk API
    ///
    /// URL: https://data-api.coindesk.com/news/v1/category/list
    NewsCategories,
    // Overview
    /// Description: Presents a thorough historical daily overview of market capitalisation for digital assets that meet the volume and listing criteria
    ///
    /// URL: https://data-api.coindesk.com/overview/v1/historical/marketcap/all/assets/days
    OverviewMktCapOHLCV,
}

impl APIEndpoint {
    fn resolve_url(&self) -> String {
        match self {
            // Legacy API
            Self::BalanceDistribution => String::from("https://min-api.cryptocompare.com/data/blockchain/balancedistribution/histo/day"),
            // Indices & Reference Rates
            Self::IndicesOHLCV => String::from("https://data-api.coindesk.com/index/cc/v1/historical"),
            // Spot
            Self::SpotOHLCV => String::from("https://data-api.coindesk.com/spot/v1/historical"),
            Self::SpotInstrumentMetadata => String::from("https://data-api.coindesk.com/spot/v1/latest/instrument/metadata"),
            Self::SpotMarketsV2 => String::from("https://data-api.coindesk.com/spot/v2/markets"),
            Self::SpotMarketsInstruments => String::from("https://data-api.coindesk.com/spot/v1/markets/instruments"),
            // Futures
            Self::FuturesOHLCV => String::from("https://data-api.coindesk.com/futures/v1/historical"),
            Self::FuturesInstrumentMetadata => String::from("https://data-api.coindesk.com/futures/v1/latest/instrument/metadata"),
            Self::FuturesMarketsV2 => String::from("https://data-api.coindesk.com/futures/v2/markets"),
            // Options
            Self::OptionsOHLCV => String::from("https://data-api.coindesk.com/options/v1/historical"),
            Self::OptionsInstrumentMetadata => String::from("https://data-api.coindesk.com/options/v1/latest/instrument/metadata"),
            Self::OptionsMarketsV2 => String::from("https://data-api.coindesk.com/options/v2/markets"),
            // Derivatives Indices
            Self::DerIndicesOHLCV => String::from("https://data-api.coindesk.com/index/v1/historical"),
            Self::DerIndicesMarketsV2 => String::from("https://data-api.coindesk.com/index/v2/markets"),
            // On-Chain DEX
            Self::OCDEXOHLCV => String::from("https://data-api.coindesk.com/onchain/v1/amm/historical/swap"),
            Self::OCDEXMarketsV2 => String::from("https://data-api.coindesk.com/onchain/v2/amm/markets"),
            // On-Chain Core
            Self::OCCoreETHBlocks => String::from("https://data-api.coindesk.com/onchain/v1/block/2"),
            Self::OCCoreAssetsByChain => String::from("https://data-api.coindesk.com/onchain/v3/summary/by/chain"),
            Self::OCCoreAssetByAddress => String::from("https://data-api.coindesk.com/onchain/v2/data/by/address"),
            Self::OCCoreSupply => String::from("https://data-api.coindesk.com/onchain/v2/historical/supply/days"),
            // Asset
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

    fn add_unit_to_url(&self, url: &mut String, unit: &Unit) -> () {
        match self {
            Self::IndicesOHLCV | Self::SpotOHLCV | Self::FuturesOHLCV |
            Self::OptionsOHLCV | Self::DerIndicesOHLCV | Self::OCDEXOHLCV => {
                url.push_str(&unit.to_string());
            },
            _ => (),
        }
    }
}

impl APIEndpointTrait for APIEndpoint {
    fn default_groups(&self) -> Option<Vec<Group>> {
        match self {
            Self::FuturesOHLCV => Some(vec![ Group::Id, Group::Mapping, Group::OHLC, Group::Trade, Group::Volume, Group::MappingAdvanced, Group::OHLCTrade]),
            Self::FuturesInstrumentMetadata => Some(vec![Group::Status, Group::General]),
            Self::OptionsOHLCV => Some(vec![Group::Id, Group::Mapping, Group::OHLC, Group::Trade, Group::Volume, Group::MappingAdvanced, Group::OHLCTrade]),
            Self::OptionsInstrumentMetadata => Some(vec![Group::Status, Group::General]),
            Self::DerIndicesOHLCV => Some(vec![Group::Id, Group::OHLC, Group::OHLCMessage, Group::Message, Group::MappingAdvanced]),
            Self::OCDEXOHLCV => Some(vec![Group::Id, Group::Mapping, Group::MappingAdvanced, Group::OHLC, Group::OHLCSwap, Group::Swap, Group::Volume]),
            Self::OCCoreETHBlocks => Some(vec![Group::Id, Group::Metadata, Group::Transactions, Group::OrphanTraces, Group::Uncles, Group::Withdrawals]),
            Self::OCCoreAssetByAddress => Some(vec![
                Group::Id, Group::Basic, Group::SupportedPlatforms, Group::SecurityMetrics, Group::Supply, Group::SupplyAddresses,
                Group::AssetTypeSpecificMetrics, Group::ResourceLinks, Group::Classification, Group::Price, Group::MktCap, Group::Volume,
                Group::Change, Group::ToplistRank, Group::Description, Group::DescriptionSummary, Group::Contact, Group::SEO,
            ]),
            Self::AssetMetadataV2 => Some(vec![Group::Id, Group::Basic, Group::Supply, Group::SupplyAddresses, Group::Classification]),
            Self::AssetCodeRepo => Some(vec![Group::Id, Group::General, Group::Activity, Group::Source]),
            Self::AssetDiscord => Some(vec![Group::Id, Group::General, Group::Activity, Group::Source]),
            Self::AssetReddit => Some(vec![Group::Id, Group::General, Group::Activity, Group::Source]),
            Self::AssetTelegram => Some(vec![Group::Id, Group::General, Group::Source]),
            Self::AssetTwitter => Some(vec![Group::Id, Group::General, Group::Activity, Group::Source]),
            Self::OverviewMktCapOHLCV => Some(vec![Group::Id, Group::OHLC, Group::Volume]),
            _ => None,
        }
    }

    fn default_params(&self) -> Option<Vec<Param<'_>>> {
        match self {
            Self::FuturesInstrumentMetadata => Some(vec![Param::ApplyMapping { v: true, }]),
            Self::OptionsInstrumentMetadata => Some(vec![Param::ApplyMapping { v: true, }]),
            Self::OCCoreAssetsByChain => Some(vec![Param::AssetLookupPriority { v: AssetLookupPriority::Symbol, }]),
            Self::OCCoreAssetByAddress => Some(vec![Param::AssetLookupPriority { v: AssetLookupPriority::Symbol, }]),
            Self::AssetMetadataV2 => Some(vec![Param::AssetLookupPriority { v: AssetLookupPriority::Symbol, }, Param::QuoteAsset { v: "USD", }]),
            _ => None,
        }
    }

    fn url(&self, unit: &Unit) -> String {
        let mut url: String = self.resolve_url();
        self.add_unit_to_url(&mut url, unit);
        url
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn unit_test_add_unit_to_url() -> () {
        use crate::{Unit, APIEndpointTrait, APIEndpoint};
        let unit: Unit = Unit::Hour;
        let api_endpoint: APIEndpoint = APIEndpoint::IndicesOHLCV;
        assert_eq!(api_endpoint.url(&unit), String::from("https://data-api.coindesk.com/index/cc/v1/historical/hours"));
    }

    #[test]
    fn unit_test_custom_api_endpoint() -> () {
        use crate::{Unit, APIEndpointTrait, call_api_endpoint};

        enum CustomEndpoint { CustomSpotOHLCV, }

        impl APIEndpointTrait for CustomEndpoint {
            fn default_groups(&self) -> Option<Vec<crate::Group>> { None }

            fn default_params(&self) -> Option<Vec<crate::Param<'_>>> { None }

            fn url(&self, unit: &crate::Unit) -> String {
                format!("https://data-api.coindesk.com/spot/v1/historical{}", unit.to_string())
            }
        }

        assert_eq!(String::from("https://data-api.coindesk.com/spot/v1/historical/days"), CustomEndpoint::CustomSpotOHLCV.url(&Unit::Day))
    }
}