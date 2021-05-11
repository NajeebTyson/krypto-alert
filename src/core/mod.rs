use std::collections::HashMap;

use crate::core::alert::{Alert, AlertType};
use crate::error::AppError;
use crate::types::PairName;

pub mod alert;
pub mod api;
pub mod notification;
pub mod symbol;

/// Main alerts struct which contains all the alerts
#[derive(Debug)]
pub struct Alerts {
    alerts: HashMap<PairName, Alert>,
}

impl Alerts {
    /// Initialize Alerts, it will load the alerts from data file if alerts exist
    pub fn init() -> Self {
        Alerts {
            alerts: Default::default(),
        }
    }

    /// Create a new alert for symbol/pair
    pub fn create_alert(
        &mut self,
        symbol_name: &str,
        alert_type: AlertType,
    ) -> Result<(), AppError> {
        match self.alerts.get_mut(symbol_name) {
            None => {
                let alert = Alert::new(symbol_name, alert_type)?;
                self.alerts.insert(symbol_name.to_string(), alert);
            }
            Some(alert) => {
                alert.create_alert(alert_type);
            }
        }
        Ok(())
    }
}

/// Main function to create/load alerts into memory
pub fn init() -> Alerts {
    Alerts::init()
}
