use notify_rust::error::Error as NotifyError;
use reqwest::Error as ReqwesetError;
use serde_json::Error as SerdeJsonError;
use snafu::Snafu;

use std::io::Error as IoError;

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
    #[snafu(display("App error: Parsing error: {}", message))]
    ParsingError { message: String },
    #[snafu(display("App error: IO error: {}", message))]
    IOError { message: String },
    #[snafu(display("App error: Notification error: {}", message))]
    NotificationError { message: String },
}

impl From<SerdeJsonError> for ApiError {
    fn from(error: SerdeJsonError) -> Self {
        ApiError::ParseError {
            message: error.to_string(),
        }
    }
}

impl From<SerdeJsonError> for AppError {
    fn from(error: SerdeJsonError) -> Self {
        AppError::ParsingError {
            message: error.to_string(),
        }
    }
}

impl From<IoError> for AppError {
    fn from(error: IoError) -> Self {
        AppError::IOError {
            message: error.to_string(),
        }
    }
}

impl From<NotifyError> for AppError {
    fn from(error: NotifyError) -> Self {
        AppError::NotificationError {
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
