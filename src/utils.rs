use std::fmt::Display;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use reqwest::Response;
use crate::error::Error;
use crate::{Unit, APIEndpointTrait, APIEndpoint};
use crate::schemas::data_api::spot::SpotInstrumentStatus;
use crate::schemas::data_api::news::{NewsLang, NewsSourceID, NewsSourceType, NewsStatus};


#[cfg(feature = "debug")]
/// Checks whether the `CCDATA_API_DEBUG` environment variable is enabled.
/// Note: If `CCDATA_API_DEBUG` variable exists, the reponses will be printed to command line.
fn debug() -> Result<bool, Error> {
    dotenv::dotenv()?;
    let debug: bool = serde_json::from_str(&std::env::var("CCDATA_API_DEBUG")?)?;
    Ok(debug)
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// Filtering groups.
/// 
/// # Description (CoinDesk Documentation) 
/// When requesting market metadata entries you can filter by specific groups of interest.
/// To do so just pass the groups of interest into the URL as a comma separated list.
/// If left empty it will get all data that your account is allowed to access.
pub enum Group {
    Activity,
    AssetTypeSpecificMetrics,
    Basic,
    Change,
    Classification,
    Contact,
    Description,
    DescriptionSummary,
    General,
    Id,
    InstrumentSummary,
    IntegrationFutures,
    Internal,
    Mapping,
    MappingAdvanced,
    Message,
    Metadata,
    Migration,
    MktCap,
    OHLC,
    OHLCMessage,
    OHLCSwap,
    OHLCTrade,
    OrphanTraces,
    Price,
    ResourceLinks,
    SecurityMetrics,
    SEO,
    Source,
    Status,
    Supply,
    SupplyAddresses,
    SupportedPlatforms,
    Swap,
    ToplistRank,
    Trade,
    Transactions,
    Uncles,
    Volume,
    Withdrawals,
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Activity => write!(f, "ACTIVITY"),
            Self::AssetTypeSpecificMetrics => write!(f, "ASSET_TYPE_SPECIFIC_METRICS"),
            Self::Basic => write!(f, "BASIC"),
            Self::Change => write!(f, "CHANGE"),
            Self::Classification => write!(f, "CLASSIFICATION"),
            Self::Contact => write!(f, "CONTACT"),
            Self::Description => write!(f, "DESCRIPTION"),
            Self::DescriptionSummary => write!(f, "DESCRIPTION_SUMMARY"),
            Self::General => write!(f, "GENERAL"),
            Self::Id => write!(f, "ID"),
            Self::InstrumentSummary => write!(f, "INSTRUMENT_SUMMARY"),
            Self::IntegrationFutures => write!(f, "INTEGRATION_FUTURES"),
            Self::Internal => write!(f, "INTERNAL"),
            Self::Mapping => write!(f, "MAPPING"),
            Self::MappingAdvanced => write!(f, "MAPPING_ADVANCED"),
            Self::Message => write!(f, "MESSAGE"),
            Self::Metadata => write!(f, "METADATA"),
            Self::Migration => write!(f, "MIGRATION"),
            Self::MktCap => write!(f, "MKT_CAP"),
            Self::OHLC => write!(f, "OHLC"),
            Self::OHLCMessage => write!(f, "OHLC_MESSAGE"),
            Self::OHLCSwap => write!(f, "OHLC_SWAP"),
            Self::OHLCTrade => write!(f, "OHLC_TRADE"),
            Self::OrphanTraces => write!(f, "ORPHAN_TRACES"),
            Self::Price => write!(f, "PRICE"),
            Self::ResourceLinks => write!(f, "RESOURCE_LINKS"),
            Self::SecurityMetrics => write!(f, "SECURITY_METRICS"),
            Self::SEO => write!(f, "SEO"),
            Self::Source => write!(f, "SOURCE"),
            Self::Status => write!(f, "STATUS"),
            Self::Supply => write!(f, "SUPPLY"),
            Self::SupplyAddresses => write!(f, "SUPPLY_ADDRESSES"),
            Self::SupportedPlatforms => write!(f, "SUPPORTED_PLATFORMS"),
            Self::Swap => write!(f, "SWAP"),
            Self::ToplistRank => write!(f, "TOPLIST_RANK"),
            Self::Trade => write!(f, "TRADE"),
            Self::Transactions => write!(f, "TRANSACTIONS"),
            Self::Uncles => write!(f, "UNCLES"),
            Self::Volume => write!(f, "VOLUME"),
            Self::Withdrawals => write!(f, "WITHDRAWALS"),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
/// Specifies the matching priority for the asset key provided in the asset parameter.
/// 
/// # Description (CoinDesk Documentation)
/// This parameter specifies the matching priority for the asset key provided in the asset parameter.
/// You can choose to match against the list of asset SYMBOLS, CoinDesk internal asset IDS, or asset URIs.
/// Note that asset SYMBOLS and URIs may change due to rebrands or token switches, but the CoinDesk internal asset ID remains consistent.
pub enum AssetLookupPriority {
    Symbol,
    Id,
    URI,
}

impl Display for AssetLookupPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol => write!(f, "SYMBOL"),
            Self::Id => write!(f, "ID"),
            Self::URI => write!(f, "URI"),
        }
    }
}


/// All possible parameter types for the REST API request.
pub enum Param<'a> {
    // Instrument parameters
    /// Asset symbol
    Symbol { v: &'a str, },
    /// Instrument symbol
    Instrument { v: &'a str, },
    /// List of instrument symbols
    Instruments { v: &'a Vec<String>, },
    /// Chain asset symbol
    ChainAsset { v: &'a str, },
    /// Asset symbol
    Asset { v: &'a str, },
    /// Assets' symbols
    Assets { v: Vec<String>, },
    // Overlapping parameters
    /// Final timestamp up to which the data will be extracted
    ToTs { v: Option<i64>, },
    /// Final timestamp up to which the data will be extracted
    ToTimestamp { v: Option<i64>, },
    /// Maximum number of datapoints per API endpoint call
    Limit { v: Option<usize>, },
    // Special parameters
    /// Market name
    Market { v: String, },
    /// Markets' names
    Markets { v: Vec<String>, },
    /// Status of the instrument (e.g., `ACTIVE`, `EXPIRED`)
    InstrumentStatus { v: SpotInstrumentStatus, },
    /// Block number on the blockchain
    OCCoreBlockNumber { v: i64, },
    /// Blockchain address
    OCCoreAddress { v: &'a str, },
    /// Asset to quote data in
    OCCoreQuoteAsset { v: &'a str, },
    /// Language of the news
    NewsLanguage { v: NewsLang, },
    /// Source ID of the news stream
    NewsSourceID { v: NewsSourceID, },
    /// List of news categories
    NewsCategories { v: Option<Vec<String>>, },
    /// List of news categories to exclude
    NewsExcludeCategories { v: Option<Vec<String>>, },
    /// Type of news stream
    NewsSourceType { v: NewsSourceType, },
    /// Status of the news stream (e.g., `ACTIVE`, `INACTIVE`)
    NewsStatus { v: NewsStatus, },
    /// Filtering groups
    Groups { v: Option<Vec<Group>> },
    /// Determines if provided instrument values are converted according to internal mappings.
    ApplyMapping { v: bool, },
    /// Specifies the matching priority for the asset key provided in the asset parameter.
    AssetLookupPriority { v: AssetLookupPriority, },
    /// Specify the digital asset for the quote values by providing either the CoinDesk internal asset ID, its unique SYMBOL,
    /// or the CoinDesk recommened URI.
    QuoteAsset { v: &'a str, },
}

impl<'a> Param<'a> {
    fn add_param_to_url(&self, url: &mut String) -> () {
        let url_param: String = match self {
            // Instrument parameters
            Self::Symbol { v } => format!("&fsym={v}"),
            Self::Instrument { v } => format!("&instrument={v}"),
            Self::Instruments { v } => format!("&instruments={}", v.join(",")),
            Self::ChainAsset { v } => format!("&chain_asset={v}"),
            Self::Asset { v } => format!("&asset={v}"),
            Self::Assets { v } => format!("&assets={}", v.join(",")),
            // Overlapping parameters
            Self::ToTs { v } => v.map_or("".to_owned(), |i| format!("&toTs={i}") ),
            Self::ToTimestamp { v } => v.map_or("".to_owned(), |i| format!("&to_ts={i}") ),
            Self::Limit { v } => format!("&limit={}", v.unwrap_or(2_000)),
            // Special parameters
            Self::Market { v } => format!("&market={v}"),
            Self::Markets { v } => format!("&markets={}", v.join(",")),
            Self::InstrumentStatus { v } => format!("&instrument_status={v}"),
            Self::OCCoreBlockNumber { v } => format!("&block_number={v}"),
            Self::OCCoreAddress { v } => format!("&address={v}"),
            Self::OCCoreQuoteAsset { v } => format!("&quote_asset={v}"),
            Self::NewsLanguage { v } => format!("&lang={v}"),
            Self::NewsSourceID { v } => format!("&source_ids={v}"),
            Self::NewsCategories { v } => format!("&categories={}", v.as_ref().unwrap_or(&vec![]).join(",")),
            Self::NewsExcludeCategories { v } => format!("&exclude_categories={}", v.as_ref().unwrap_or(&vec![]).join(",")),
            Self::NewsSourceType { v } => format!("&source_type={v}"),
            Self::NewsStatus { v } => format!("&status={v}"),
            Self::Groups { v } => v.as_ref().map_or("".to_owned(), |i|
                format!("&groups={}", i.iter().map(|v| v.to_string() ).collect::<Vec<String>>().join(","))
            ),
            Self::ApplyMapping { v } => format!("&apply_mapping={v}"),
            Self::AssetLookupPriority { v } => format!("&asset_lookup_priority={v}"),
            Self::QuoteAsset { v } => format!("&quote_asset={v}"),
        };
        url.push_str(&url_param);
    }
}


/// Make a request to the provided URL, validate the status code of the response, and return deserialized data.
async fn process_request<T: DeserializeOwned>(url: String) -> Result<T, Error> {
    let response: Response = reqwest::get(url).await?;
    let response_body: String = response.text().await?;
    // Print response body to the command line
    #[cfg(feature = "debug")]
    if debug()? { println!("{:?}", response_body) }
    // Replace empty object and empty list placeholders
    let response_body: String = response_body.replace("{}", "null");
    // let response_body: String = response_body.replace("[]", "null");
    let data: T = serde_json::from_str(&response_body)?;
    Ok(data)
}


/// Constructs a URL for API request, sends the request, and returns the deserialzied response.
///
/// # Input
/// - `api_key`: CoinDesk API key
/// - `endpoint`: Enum that represents the CoinDesk API endpoint for function to send the request to
/// - `unit`: Unit for data to be binned by (e.g., `Day`, `Hour`)
/// - `params`: List of parameters expected by the CoinDesk API endpoint
/// - `additional_params`: Additional parameters to add to the request
pub async fn call_api_endpoint<'a, R: DeserializeOwned>(
    api_key: &str, endpoint: APIEndpoint, unit: Unit, mut params: Vec<Param<'a>>, additional_params: Option<String>) -> Result<R, Error>
{
    // Set up a URL for the API endpoint
    let mut url: String = endpoint.url(&unit);
    // Add API key, default parameters and groups to the URL
    url.push_str(&format!("?api_key={}", api_key));
    params.push(Param::Groups { v: endpoint.default_groups() });
    if let Some(default_params) = endpoint.default_params() {
        params.extend(default_params);
    }
    // Add parameters to the URL
    params.iter().for_each(|v| v.add_param_to_url(&mut url) );
    // Add additional parameters to the URL
    additional_params.map(|v| url.push_str(&v) );
    // Process API response
    process_request::<R>(url).await
}