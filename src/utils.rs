use serde::de::DeserializeOwned;
use reqwest::Response;
use crate::error::Error;
use crate::{CCUnit, CCAPIEndpoint};
use crate::schemas::data_api::spot::CCSpotInstrumentStatus;
use crate::schemas::data_api::news::{CCNewsLang, CCNewsSourceID, CCNewsSourceType, CCNewsStatus};


#[cfg(feature = "debug")]
/// Checks whether the `CCDATA_API_DEBUG` environment variable is enabled.
/// Note: If `CCDATA_API_DEBUG` variable exists, the reponses will be printed to command line.
fn debug() -> Result<bool, Error> {
    dotenv::dotenv()?;
    let debug: bool = serde_json::from_str(&std::env::var("CCDATA_API_DEBUG")?)?;
    Ok(debug)
}


/// Market interface that ensures all market enums implement .to_string().
pub trait Market {
    /// Converts enum value to `String`.
    fn to_string(&self) -> String;
}


/// Convert a vector of inputs into a string of inputs suitable for use in REST API requests.
fn vec_to_str(vec: &Vec<String>) -> String {
    let mut s: String = String::from("");
    for v in vec {
        s.push_str(&v);
        s.push_str(&",");
    }
    s.pop();
    s
}


/// Converts a vector of optional parameters into a string.
fn optional_vec_arguments(o: Option<Vec<String>>, api_argument: String) -> String {
    let mut s: String = String::from("");
    match o {
        Some(v) => {
            s.push_str(&api_argument);
            s.push_str(&vec_to_str(&v));
        },
        None => (),
    }
    s
}


/// All possible parameter types for the REST API request.
pub enum Param<'a> {
    // Instrument parameters
    /// Asset symbol
    Symbol { v: &'a str },
    /// Instrument symbol
    Instrument { v: &'a str },
    /// List of instrument symbols
    Instruments { v: &'a Vec<String> },
    /// Chain asset symbol
    ChainAsset { v: &'a str },
    /// Asset symbol
    Asset {v: &'a str},
    /// Assets' symbols
    Assets {v: Vec<String>},
    // Overlapping parameters
    /// Final timestamp up to which the data will be extracted
    ToTs {v: Option<i64>},
    /// Final timestamp up to which the data will be extracted
    ToTimestamp { v: Option<i64> },
    /// Maximum number of datapoints per API endpoint call
    Limit { v: Option<usize> },
    // Special parameters
    /// Market name
    Market {v: String},
    /// Status of the instrument (e.g., `ACTIVE`, `EXPIRED`)
    InstrumentStatus { v: CCSpotInstrumentStatus },
    /// Block number on the blockchain
    OCCoreBlockNumber { v: i64 },
    /// Blockchain address
    OCCoreAddress { v: &'a str },
    /// Asset to quote data in
    OCCoreQuoteAsset { v: &'a str },
    /// Language of the news
    NewsLanguage { v: CCNewsLang },
    /// Source ID of the news stream
    NewsSourceID { v: CCNewsSourceID },
    /// List of news categories
    NewsCategories { v: Option<Vec<String>> },
    /// List of news categories to exclude
    NewsExcludeCategories { v: Option<Vec<String>> },
    /// Type of news stream
    NewsSourceType { v: CCNewsSourceType },
    /// Status of the news stream (e.g., `ACTIVE`, `INACTIVE`)
    NewsStatus { v: CCNewsStatus },
}

impl<'a> Param<'a> {
    fn add_param_to_url(&self, url: &mut String) -> () {
        let url_param: String = match self {
            // Instrument parameters
            Self::Symbol { v } => format!("&fsym={}", v),
            Self::Instrument { v } => format!("&instrument={}", v),
            Self::Instruments { v } => format!("&instruments={}", vec_to_str(v.to_owned())),
            Self::ChainAsset { v } => format!("&chain_asset={}", v),
            Self::Asset { v } => format!("&asset={}", v),
            Self::Assets { v } => format!("&assets={}", v.join(",")),
            // Overlapping parameters
            Self::ToTs { v } => match v {
                Some(v_) => format!("&toTs={}", v_),
                None => String::from(""),
            },
            Self::ToTimestamp { v } => match v {
                Some(v_) => format!("&to_ts={}", v_),
                None => String::from(""),
            },
            Self::Limit { v } => match v {
                Some(v_) => format!("&limit={}", v_),
                None => String::from("&limit=2000"),
            },
            // Special parameters
            Self::Market { v } => format!("&market={}", v),
            Self::InstrumentStatus { v } => format!("&instrument_status={}", v.to_string()),
            Self::OCCoreBlockNumber { v } => format!("&block_number={}", v),
            Self::OCCoreAddress { v } => format!("&address={}", v),
            Self::OCCoreQuoteAsset { v } => format!("&quote_asset={}", v),
            Self::NewsLanguage { v } => format!("&lang={}", v.to_string()),
            Self::NewsSourceID { v } => format!("&source_ids={}", v.to_string()),
            Self::NewsCategories { v } => optional_vec_arguments(v.to_owned(), String::from("&categories=")),
            Self::NewsExcludeCategories { v } => optional_vec_arguments(v.to_owned(), String::from("&exclude_categories=")),
            Self::NewsSourceType { v } => format!("&source_type={}", v.to_string()),
            Self::NewsStatus { v } => format!("&status={}", v.to_string()),
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
    api_key: &str, endpoint: CCAPIEndpoint, unit: CCUnit, params: Vec<Param<'a>>, additional_params: Option<String>) -> Result<R, Error>
{
    // Set up a URL for the API endpoint
    let mut url: String = endpoint.url(&unit);
    // Add parameters to the URL
    url.push_str(&format!("?api_key={}", api_key));
    for param in params {
        param.add_param_to_url(&mut url);
    }
    // Add additional parameters to the URL
    match additional_params {
        Some(v) => url.push_str(&v),
        None => (),
    }
    // Process API response
    process_request::<R>(url).await
}