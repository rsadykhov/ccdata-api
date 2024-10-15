use std::io::{Error, ErrorKind};
use reqwest::{Error as RequestError, Response};
use serde::de::DeserializeOwned;
use crate::{CCUnit, CCAPIEndpoint};
use crate::schemas::data_api::spot::CCSpotInstrumentStatus;
use crate::schemas::data_api::news::{CCNewsLang, CCNewsSourceID, CCNewsSourceType, CCNewsStatus};


/// Returns the value inside the Option or an Error with the message.
pub fn some_or_error<T>(input: Option<T>, message: &str) -> Result<T, Error> {
    match input {
        Some(v) => Ok(v),
        None => Err(Error::new(ErrorKind::InvalidData, message))
    }
}


/// Market interface that ensures all market enums implement .to_string().
pub trait Market {
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
    Symbol { v: &'a String },
    Instrument { v: &'a String },
    Instruments { v: &'a Vec<String> },
    ChainAsset { v: &'a String },
    Asset {v: &'a String},
    // Overlapping parameters
    ToTs {v: Option<i64>},
    ToTimestamp { v: Option<i64> },
    Limit { v: Option<usize> },
    // Special parameters
    Market {v: String},
    InstrumentStatus { v: CCSpotInstrumentStatus },
    OCCoreBlockNumber { v: i64 },
    OCCoreAddress { v: &'a String },
    OCCoreQuoteAsset { v: &'a String },
    NewsLanguage { v: CCNewsLang },
    NewsSourceID { v: CCNewsSourceID },
    NewsCategories { v: Option<Vec<String>> },
    NewsExcludeCategories { v: Option<Vec<String>> },
    NewsSourceType { v: CCNewsSourceType },
    NewsStatus { v: CCNewsStatus },
}

impl<'a> Param<'a> {
    fn add_param_to_url(&self, url: &mut String) -> () {
        let url_param: String = match self {
            // Instrument parameters
            Param::Symbol { v } => format!("&fsym={}", v),
            Param::Instrument { v } => format!("&instrument={}", v),
            Param::Instruments { v } => format!("&instruments={}", vec_to_str(v.to_owned())),
            Param::ChainAsset { v } => format!("&chain_asset={}", v),
            Param::Asset { v } => format!("&asset={}", v),
            // Overlapping parameters
            Param::ToTs { v } => match v {
                Some(v_) => format!("&toTs={}", v_),
                None => String::from(""),
            },
            Param::ToTimestamp { v } => match v {
                Some(v_) => format!("&to_ts={}", v_),
                None => String::from(""),
            },
            Param::Limit { v } => match v {
                Some(v_) => format!("&limit={}", v_),
                None => String::from("&limit=2000"),
            },
            // Special parameters
            Param::Market { v } => format!("&market={}", v),
            Param::InstrumentStatus { v } => format!("&instrument_status={}", v.to_string()),
            Param::OCCoreBlockNumber { v } => format!("&block_number={}", v),
            Param::OCCoreAddress { v } => format!("&address={}", v),
            Param::OCCoreQuoteAsset { v } => format!("&quote_asset={}", v),
            Param::NewsLanguage { v } => format!("&lang={}", v.to_string()),
            Param::NewsSourceID { v } => format!("&source_ids={}", v.to_string()),
            Param::NewsCategories { v } => optional_vec_arguments(v.to_owned(), String::from("&categories=")),
            Param::NewsExcludeCategories { v } => optional_vec_arguments(v.to_owned(), String::from("&exclude_categories=")),
            Param::NewsSourceType { v } => format!("&source_type={}", v.to_string()),
            Param::NewsStatus { v } => format!("&status={}", v.to_string()),
        };
        url.push_str(&url_param);
    }
}


/// Converts reqwest::Error to std::io::Error.
fn request_err_to_io_err<T>(res: Result<T, RequestError>) -> Result<T, Error> {
    match res {
        Ok(r) => Ok(r),
        Err(e) => Err(Error::new(ErrorKind::Other, e.without_url().to_string()))
    }
}


/// Make a request to the provided URL, validate the status code of the response, and return deserialized data.
async fn process_request<T: DeserializeOwned>(url: String) -> Result<T, Error> {
    let response: Response = request_err_to_io_err(reqwest::get(url).await)?;
    let response_body: String = request_err_to_io_err(response.text().await)?;
    let data: T = serde_json::from_str(&response_body)?;
    Ok(data)
}


/// Constructs a URL for API request, sends the request, and returns the deserialzied response.
pub async fn call_api_endpoint<'a, R: DeserializeOwned>(api_key: &String, endpoint: CCAPIEndpoint, unit: CCUnit, params: Vec<Param<'a>>,
                                                        additional_params: Option<String>) -> Result<R, Error> {
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