pub mod min_api;
pub mod data_api;


use std::io::Error;
use serde::Deserialize;
use crate::utils::other_error;


// Helper wrapper to parse empty JSON objects.
// Implemented as per: https://github.com/serde-rs/json/issues/660


/// Enum to parse empty JSON objects as Option::None.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum EmptyObject<T> {
    Some(T),
    None {},
}

impl<T> From<EmptyObject<T>> for Option<T> {
    fn from(empty_object: EmptyObject<T>) -> Option<T> {
        match empty_object {
            EmptyObject::Some(v) => Some(v),
            EmptyObject::None {  } => None,
        }
    }
}

impl<T> From<Option<T>> for EmptyObject<T> {
    fn from(option: Option<T>) -> EmptyObject <T> {
        match option {
            Some(v) => EmptyObject::Some(v),
            None => EmptyObject::None {  },
        }
    }
}

impl<T> EmptyObject<T> {
    pub fn into_option(self) -> Option<T> {
        self.into()
    }

    pub fn as_option(&self) -> Option<&T> {
        match self {
            EmptyObject::Some(v) => Some(v),
            EmptyObject::None {} => None,
        }
    }
}


/// Trait that unwraps data or propagates the error.
pub trait DataUnwrap<T> {
    fn data_unwrap(self) -> Result<T, Error>;
}


// Min-API Wrappers


#[derive(Deserialize, Debug)]
pub struct CCCallsMade {
    pub second: i32,
    pub minute: i32,
    pub hour: i32,
    pub day: i32,
    pub month: i32,
    pub total_calls: i32,
}

#[derive(Deserialize, Debug)]
pub struct CCMaxCalls {
    pub second: i32,
    pub minute: i32,
    pub hour: i32,
    pub day: i32,
    pub month: i32,
}

#[derive(Deserialize, Debug)]
pub struct CCRateLimit {
    pub calls_made: Option<CCCallsMade>,
    pub max_calls: Option<CCMaxCalls>,
}

#[derive(Deserialize, Debug)]
pub struct CCMinWrapper<T> {
    #[serde(rename = "Aggregated")]
    pub aggregated: Option<bool>,
    #[serde(rename = "TimeFrom")]
    pub time_from: Option<i64>,
    #[serde(rename = "TimeTo")]
    pub time_to: Option<i64>,
    #[serde(rename = "Data")]
    pub data: Option<T>,
}

impl<T> DataUnwrap<T> for CCMinWrapper<T> {
    fn data_unwrap(self) -> Result<T, Error> {
        self.data.ok_or(other_error("Data Unwrap: No data inside the CCMinWrapper."))
    }
}

#[derive(Deserialize, Debug)]
pub struct CCMinResponse<T> {
    #[serde(rename = "Response")]
    pub response: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "HasWarning")]
    pub has_warning: bool,
    #[serde(rename = "Type")]
    pub type_: i32,
    #[serde(rename = "Data")]
    pub data: EmptyObject<T>,
    #[serde(rename = "RateLimit")]
    pub rate_limit: Option<CCRateLimit>,
}

impl<T> DataUnwrap<T> for CCMinResponse<T> {
    fn data_unwrap(self) -> Result<T, Error> {
        self.data.into_option().ok_or(other_error("Data Unwrap: No data inside the CCMinResponse."))
    }
}


// Data-API Wrappers


#[derive(Deserialize, Debug)]
pub struct CCErrorOtherInfo {
    /// The parameter that is responsible for the error.
    pub param: Option<String>,
    /// The values responsible for the error.
    pub values: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
/// This object provides detailed information about an error encountered while processing the request. It includes an error code, a message explaining the error,
/// and additional context about the parameters or values that caused the issue. This helps clients identify and resolve issues with their requests.
pub struct CCError {
    #[serde(rename = "type")]
    /// A public facing error type. If you want to treat a specific error use the type.
    pub type_: i32,
    /// A message describing the error.
    pub message: String,
    pub other_info: Option<CCErrorOtherInfo>,
}

#[derive(Deserialize, Debug)]
pub struct CCDataResponse<T> {
    #[serde(rename = "Data")]
    pub data: EmptyObject<T>,
    #[serde(rename = "Err")]
    /// This object provides detailed information about an error encountered while processing the request. It includes an error code,
    /// a message explaining the error, and additional context about the parameters or values that caused the issue.
    /// This helps clients identify and resolve issues with their requests.
    pub error: EmptyObject<CCError>,
}

impl<T> DataUnwrap<T> for CCDataResponse<T> {
    fn data_unwrap(self) -> Result<T, Error> {
        self.data.into_option().ok_or(other_error("Data Unwrap: No data inside the CCDataResponse."))
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn unit_test_nullable_field() -> () {
        use serde_json;
        use crate::schemas;
        let d: String = String::from("{\"Data\":{}, \"Err\":{\"type\": 23, \"message\": \"hello\", \"other_info\":null}}");
        let response: schemas::CCDataResponse<String> = serde_json::from_str(&d.to_string()).unwrap();
        assert_eq!(response.data.into_option(), None);
    }
}