use crate::core::api;
use crate::error::AppError;
use crate::types::*;

use futures::executor::block_on;

/// Symbol struct which contains all the symbol related attributes
#[derive(Debug)]
pub struct Symbol {
    pub name: PairName,
    pub current_price: Amount,
}

impl Symbol {
    /// Create new symbol
    pub fn new(symbol_name: &str) -> Result<Self, AppError> {
        let symbol_res = block_on(api::get_symbol_price(symbol_name))?;
        Ok(Symbol {
            name: symbol_res.symbol,
            current_price: symbol_res.price.parse::<Amount>().unwrap(),
        })
    }
}
