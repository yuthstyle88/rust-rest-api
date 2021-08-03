use serde_json::Value;
use reqwest::{Error as ReqwestError, StatusCode};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub error: ErrorMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub errors: Option<Vec<Value>>,
    pub code: u16,
    pub message: String,
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Self {
            error: ErrorMessage {
                code: err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR).as_u16(),
                message: err.to_string(),
                errors: None
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(format!("code: {}, message: {}", self.error.code, self.error.message).as_str())
    }
}
