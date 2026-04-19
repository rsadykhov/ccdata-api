pub mod min_api;
pub mod data_api;


use serde::{Serialize, Deserialize, Deserializer};


#[derive(Clone, Debug, PartialEq, Serialize)]
/// Custom response type that may use different types for the same value.
pub enum StringOrInt {
    String(String),
    Int64(i64),
    UInt64(u64),
}

impl<'de> Deserialize<'de> for StringOrInt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StringOrInt;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a String, i64 or u64")
            }

            fn visit_string<E: serde::de::Error>(self, s: String) -> Result<Self::Value, E> {
                Ok(StringOrInt::String(s))
            }

            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(StringOrInt::Int64(v))
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(StringOrInt::UInt64(v))
            }
        }
        
        deserializer.deserialize_any(Visitor)
    }
}


// Min-API Wrappers


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CCCallsMade {
    pub second: i32,
    pub minute: i32,
    pub hour: i32,
    pub day: i32,
    pub month: i32,
    pub total_calls: i32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CCMaxCalls {
    pub second: i32,
    pub minute: i32,
    pub hour: i32,
    pub day: i32,
    pub month: i32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CCRateLimit {
    pub calls_made: Option<CCCallsMade>,
    pub max_calls: Option<CCMaxCalls>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub data: Option<T>,
    #[serde(rename = "RateLimit")]
    pub rate_limit: Option<CCRateLimit>,
}


// Data-API Wrappers


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CCErrorOtherInfo {
    /// The parameter that is responsible for the error.
    pub param: Option<String>,
    /// The values responsible for the error.
    pub values: Option<Vec<StringOrInt>>,
    // Instrument specific information
    /// Status of the instrument.
    pub instrument_status: Option<String>,
    /// First available timestamp.
    pub first: Option<i64>,
    /// Last available timestamp.
    pub last: Option<i64>,
    /// Earliest bucket timestamp.
    pub first_bucket: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinDeskResponse<T> {
    #[serde(rename = "Data")]
    pub data: Option<T>,
    #[serde(rename = "Err")]
    /// This object provides detailed information about an error encountered while processing the request. It includes an error code,
    /// a message explaining the error, and additional context about the parameters or values that caused the issue.
    /// This helps clients identify and resolve issues with their requests.
    pub error: Option<CCError>,
}


#[cfg(test)]
mod tests {

    #[test]
    fn unit_test_nullable_field() -> () {
        use serde_json;
        use crate::schemas;
        let d: String = String::from("{\"Data\":{}, \"Err\":{\"type\": 23, \"message\": \"hello\", \"other_info\":null}}");
        let response: schemas::CoinDeskResponse<String> = serde_json::from_str(&d.to_string().replace("{}", "null")).unwrap();
        assert_eq!(response.data, None);
    }
}