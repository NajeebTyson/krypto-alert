use crate::core::symbol::Symbol;
use crate::error::AppError;
use crate::types::*;
use crate::utils;

/// Enum for alert types
#[derive(Debug)]
pub enum AlertType {
    PriceAbove(Amount),
    PriceBelow(Amount),
    ChangeOver(Percentage),
    ChangeUnder(Percentage),
    Change24hOver(Percentage),
    Change24hUnder(Percentage),
}

#[derive(Debug)]
pub struct SimpleAlert {
    pub id: AlertId,
    pub alert_type: AlertType,
}

/// Main struct for alert
#[derive(Debug)]
pub struct Alert {
    pub symbol: Symbol,
    pub symbol_alerts: Vec<SimpleAlert>,
}

impl Alert {
    /// Create new alert against symbol
    pub fn new(pair_name: &str, alert_type: AlertType) -> Result<Self, AppError> {
        Ok(Alert {
            symbol: Symbol::new(pair_name)?,
            symbol_alerts: vec![SimpleAlert {
                id: utils::generate_new_alert_id(),
                alert_type,
            }],
        })
    }

    /// Create a new alert
    pub fn create_alert(&mut self, alert_type: AlertType) {
        self.symbol_alerts.push(SimpleAlert {
            id: utils::generate_new_alert_id(),
            alert_type,
        });
    }
}
