use crate::core::api;
use crate::core::symbol::Symbol;
use crate::error::AppError;
use crate::types::*;
use crate::utils;

use futures::executor::block_on;

/// Enum for alert types
#[derive(Debug)]
pub enum AlertType {
    PriceAbove(Amount),
    PriceBelow(Amount),
    ChangeOver(Percentage),
    ChangeUnder(Percentage),
    // not supporting these right now
    // Change24hOver(Percentage),
    // Change24hUnder(Percentage),
}

#[derive(Debug)]
pub struct SimpleAlert {
    pub id: AlertId,
    pub alert_type: AlertType,
    pub current_price: Amount,
    // flag to auto renew the current price while doing refresh
    pub auto_renew: bool,
}

/// Main struct for alert
#[derive(Debug)]
pub struct Alert {
    pub symbol: Symbol,
    pub symbol_alerts: Vec<SimpleAlert>,
}

impl Alert {
    /// Create new alert against symbol
    pub fn new(pair_name: &str, alert_type: AlertType, auto_renew: bool) -> Result<Self, AppError> {
        let symbol_res = block_on(api::get_symbol_price(pair_name))?;
        Ok(Alert {
            symbol: Symbol::new(pair_name),
            symbol_alerts: vec![SimpleAlert {
                id: utils::generate_new_alert_id(),
                alert_type,
                current_price: symbol_res.price,
                auto_renew,
            }],
        })
    }

    /// Create a new alert
    pub fn create_alert(
        &mut self,
        alert_type: AlertType,
        auto_renew: bool,
    ) -> Result<(), AppError> {
        let symbol_res = block_on(api::get_symbol_price(&self.symbol.name))?;
        self.symbol_alerts.push(SimpleAlert {
            id: utils::generate_new_alert_id(),
            alert_type,
            current_price: symbol_res.price,
            auto_renew,
        });
        Ok(())
    }
}
