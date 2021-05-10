use crate::error::ApiError;

use reqwest::{header, Client, Response as ReqwestResponse};
use serde::Serialize;

use log;

const BINANCE_BASE_API_URL: &str = "https://api.binance.com";

lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref SYMBOL_PRICE_URL: String = format!("{}/api/v3/ticker/price", BINANCE_BASE_API_URL);
}

/// Tuple struct to attach query to request
type RequestQuery<'a> = (&'a str, &'a str);
type RequestQueries<'a> = Vec<RequestQuery<'a>>;

/// Helper function to call post request for Binance api
async fn binance_get_request<'a, T: Serialize>(
    url: &str,
    query: Option<RequestQueries<'a>>,
    payload: Option<&T>,
) -> Result<ReqwestResponse, ApiError> {
    let mut req = CLIENT
        .get(url)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(query) = query {
        req = req.query(&query);
    }
    if let Some(payload) = payload {
        req = req.json(&serde_json::to_value(payload)?);
    }
    let res = req.send().await?;

    Ok(res)
}

/// Get all the symbols and current price from Binance
pub async fn get_all_symbols_price() -> Result<SymbolsResponse, ApiError> {
    let res = binance_get_request::<()>(&SYMBOL_PRICE_URL, None, None).await?;
    if !res.status().is_success() {
        log::error!("get_all_symbols_price error: {}", res.text().await?);
        return Err(ApiError::UnableToGetSymbolError);
    }
    let data: SymbolsResponse = res.json().await?;
    Ok(data)
}

/// Get single the symbol and current price from Binance
pub async fn get_symbol_price(symbol: &str) -> Result<SymbolResponse, ApiError> {
    let query = vec![("symbol", symbol)];
    let res = binance_get_request::<()>(&SYMBOL_PRICE_URL, Some(query), None).await?;
    if !res.status().is_success() {
        log::error!("get_symbol_price error: {}", res.text().await?);
        return Err(ApiError::UnableToGetSymbolError);
    }
    let data: SymbolResponse = res.json().await?;
    Ok(data)
}

/// Struct to hold symbol name and price
#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolResponse {
    //"ETHBTC"
    pub symbol: String,
    // "0.06036600"
    pub price: String,
}

pub type SymbolsResponse = Vec<SymbolResponse>;
