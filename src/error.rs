use reqwest::Error as ReqwesetError;
use serde_json::Error as SerdeJsonError;
use snafu::Snafu;

/// Error enum to handle API related errors
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
    #[snafu(display("Invalid Symbol: {}", symbol))]
    InvalidSymbol { symbol: String },
}

/// Error enum to handle Application related errors
#[derive(Debug, Snafu)]
pub enum AppError {
    #[snafu(display("App error: Invalid symbol"))]
    InvalidSymbolError,
    #[snafu(display("API error: {}", message))]
    AppApiError { message: String },
    #[snafu(display("App error: {}", message))]
    DefaultError { message: String },
    #[snafu(display("App error: thread is already running"))]
    ThreadAlreadyRunning,
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

impl From<ApiError> for AppError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::BadRequest { .. }
            | ApiError::ParseError { .. }
            | ApiError::RequestError { .. } => {
                println!("from api error to app error: error: {:?}", error);
                AppError::AppApiError {
                    message: "Hell".to_string(),
                }
            }
            ApiError::UnableToGetSymbolError => AppError::AppApiError {
                message: "Unable to get symbol price".to_string(),
            },
            _ => AppError::DefaultError {
                message: "Unknown error".to_string(),
            },
        }
    }
}
