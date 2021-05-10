use reqwest::Error as ReqwesetError;
use serde_json::Error as SerdeJsonError;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum ApiError {
    #[snafu(display("API bad request error: {}", message))]
    BadRequest { message: String },
    #[snafu(display("API Parse error: {}", message))]
    ParseError { message: String },
    #[snafu(display("API request error: {}", message))]
    RequestError { message: String },
    #[snafu(display("API error: Unable to get symbols price"))]
    UnableToGetSymbolError,
}

impl From<SerdeJsonError> for ApiError {
    fn from(error: SerdeJsonError) -> Self {
        ApiError::ParseError {
            message: error.to_string(),
        }
    }
}

impl From<ReqwesetError> for ApiError {
    fn from(error: ReqwesetError) -> Self {
        ApiError::RequestError {
            message: error.to_string(),
        }
    }
}
