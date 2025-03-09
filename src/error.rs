use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrimlightError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("API error: {code} - {message}")]
    ApiError { code: i32, message: String },
}
