// Error types
use std::{error::Error as ErrorTrait, env::VarError};
use dotenv::Error as DotenvError;
use serde_json::Error as SerdeJSONError;
use reqwest::Error as ReqwestError;
// Dependencies
use std::{fmt::Display, convert::From};


#[derive(Debug)]
pub enum Error {
    NoAPIKey,
    // Std errors
    VarError(VarError),
    // Dotenv errors
    DotenvError(DotenvError),
    // Serde JSON errors
    SerdeJSONError(SerdeJSONError),
    // Reqwest errors
    ReqwestError(ReqwestError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoAPIKey => write!(f, "No API Key: No API key is defined."),
            // Std errors
            Self::VarError(e) => write!(f, "Var Error: {}", e.to_string()),
            // Dotenv errors
            Self::DotenvError(e) => write!(f, "Dotenv Error: {}", e.to_string()),
            // Serde JSON errors
            Self::SerdeJSONError(e) => write!(f, "Serde JSON Error: {}", e.to_string()), 
            // Reqwest errors
            Self::ReqwestError(e) => write!(f, "Reqwest Error: {}", e.to_string()),
        }
    }
}

impl ErrorTrait for Error {}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Self::VarError(value)
    }
}

impl From<DotenvError> for Error {
    fn from(value: DotenvError) -> Self {
        Self::DotenvError(value)
    }
}

impl From<SerdeJSONError> for Error {
    fn from(value: SerdeJSONError) -> Self {
        Self::SerdeJSONError(value)
    }
}

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        Self::ReqwestError(value)
    }
}