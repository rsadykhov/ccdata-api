pub mod min_api;
pub mod data_api;


use serde::Deserialize;


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
    pub data: Option<T>,
    #[serde(rename = "RateLimit")]
    pub rate_limit: Option<CCRateLimit>,
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
        let response: schemas::CCDataResponse<String> = serde_json::from_str(&d.to_string().replace("{}", "null")).unwrap();
        assert_eq!(response.data, None);
    }
}