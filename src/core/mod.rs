pub mod alert;
pub mod api;
mod notification;
mod symbol;
pub mod worker;

use crate::core::alert::{Alert, AlertType, SimpleAlert};
use crate::core::api::{SymbolResponse, SymbolsResponse};
use crate::error::AppError;
use crate::types::PairName;
use crate::utils;

use futures::executor::block_on;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/// Main alerts struct which contains all the alerts
#[derive(Debug)]
pub struct Alerts {
    alerts: HashMap<PairName, Alert>,
}

impl Alerts {
    /// Initialize Alerts, it will load the alerts from data file if alerts exist
    pub fn init() -> Self {
        // Construct Alert
        let alerts = Alerts {
            alerts: Default::default(),
        };

        // return alert instance
        alerts
    }

    /// Create a new alert for symbol/pair
    pub fn create_alert(
        &mut self,
        symbol_name: &str,
        alert_type: AlertType,
        auto_renew: bool,
    ) -> Result<(), AppError> {
        match self.alerts.get_mut(symbol_name) {
            None => {
                let alert = Alert::new(symbol_name, alert_type, auto_renew)?;
                self.alerts.insert(symbol_name.to_string(), alert);
            }
            Some(alert) => {
                alert.create_alert(alert_type, auto_renew);
            }
        }
        Ok(())
    }

    /// Method check for prices and notify the alerts
    pub fn refresh(&mut self) {
        // get all the markets data
        let market_data = self.get_symbols_price().unwrap();

        for symbol in &market_data {
            // check alerts
            self.check_symbol_alerts(symbol);
            // renew alerts
            self.renew_prices(symbol);
        }
    }

    /// Check with the current price for alert
    fn check_symbol_alerts(&self, symbol: &SymbolResponse) {
        if let Some(alerts) = self.alerts.get(&symbol.symbol) {
            for alert in &alerts.symbol_alerts {
                self.check_alert(&symbol, &alert);
            }
        }
    }

    /// Check if the alert condition satisfies
    fn check_alert(&self, symbol: &SymbolResponse, alert: &SimpleAlert) -> bool {
        match alert.alert_type {
            AlertType::PriceAbove(price) => symbol.price > price,
            AlertType::PriceBelow(price) => symbol.price < price,
            AlertType::ChangeOver(percent) => {
                utils::calc_percentage_change(alert.current_price.borrow(), &symbol.price) > percent
            }

            AlertType::ChangeUnder(percent) => {
                utils::calc_percentage_change(alert.current_price.borrow(), &symbol.price) < percent
            }
        }
    }

    /// Refresh current prices of the alerts if auto-renew is true
    fn renew_prices(&mut self, symbol: &SymbolResponse) {
        if let Some(alerts) = self.alerts.get_mut(&symbol.symbol) {
            for alert in &mut alerts.symbol_alerts {
                if alert.auto_renew {
                    alert.current_price = symbol.price;
                }
            }
        }
    }

    /// Get the filtered symbols data, contains only the symbols
    /// which are in the alerts list
    fn get_symbols_price(&self) -> Result<SymbolsResponse, AppError> {
        let market_data = block_on(api::get_all_symbols_price())?;
        let market_data = market_data
            .into_iter()
            .filter(|symbol| self.alerts.contains_key(&symbol.symbol))
            .collect();

        Ok(market_data)
    }
}

/// Main function to create/load alerts into memory
pub fn init() -> Alerts {
    Alerts::init()
}
