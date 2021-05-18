use crate::error::ApiError;
use crate::types::Amount;

use log;
use rayon::prelude::*;
use reqwest::{
    blocking::{Client, Response as ReqwestResponse},
    header,
};
use serde::Serialize;

use std::sync::Arc;

const BINANCE_BASE_API_URL: &str = "https://api.binance.com";

lazy_static! {
    static ref CLIENT: Arc<Client> = Arc::new(Client::new());
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
    let res = req.send()?;

    Ok(res)
}

/// Get all the symbols and current price from Binance
pub async fn get_all_symbols_price() -> Result<SymbolsResponse, ApiError> {
    let res = binance_get_request::<()>(&SYMBOL_PRICE_URL, None, None).await?;
    if !res.status().is_success() {
        log::error!("get_all_symbols_price error: {}", res.text()?);
        return Err(ApiError::UnableToGetSymbolError);
    }
    let data: SymbolsRes = res.json()?;
    let data: SymbolsResponseVec = data.into();
    Ok(data.0)
}

/// Get single the symbol and current price from Binance
pub async fn get_symbol_price(symbol: &str) -> Result<SymbolResponse, ApiError> {
    let query = vec![("symbol", symbol)];
    let res = binance_get_request::<()>(&SYMBOL_PRICE_URL, Some(query), None).await?;
    if !res.status().is_success() {
        return if res.status() == 400 {
            let error = res.json::<serde_json::Value>()?;
            log::error!(
                "get_symbol_price Bad request: {}",
                error["msg"].as_str().unwrap()
            );
            Err(ApiError::InvalidSymbol {
                symbol: symbol.to_string(),
            })
        } else {
            log::error!("get_symbol_price error: {}", res.text()?);
            Err(ApiError::UnableToGetSymbolError)
        };
    }
    let data: SymbolRes = res.json()?;
    Ok(data.into())
}

/// Struct to hold symbol name and price
#[derive(Debug, Serialize, Deserialize)]
struct SymbolRes {
    //"ETHBTC"
    pub symbol: String,
    // "0.06036600"
    pub price: String,
}

type SymbolsRes = Vec<SymbolRes>;

/// Struct to hold symbol name and price
#[derive(Debug)]
pub struct SymbolResponse {
    //"ETHBTC"
    pub symbol: String,
    // 0.06036600
    pub price: Amount,
}

pub type SymbolsResponse = Vec<SymbolResponse>;
struct SymbolsResponseVec(pub Vec<SymbolResponse>);

impl From<SymbolRes> for SymbolResponse {
    fn from(symbol_res: SymbolRes) -> Self {
        SymbolResponse {
            symbol: symbol_res.symbol,
            price: symbol_res.price.parse::<Amount>().unwrap(),
        }
    }
}

impl From<SymbolsRes> for SymbolsResponseVec {
    fn from(symbols: SymbolsRes) -> Self {
        SymbolsResponseVec(
            symbols
                .into_par_iter()
                .map(|symbol| symbol.into())
                .collect(),
        )
    }
}
