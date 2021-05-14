use std::collections::HashMap;

use crate::core::alert::{Alert, AlertType};
use crate::error::AppError;
use crate::types::PairName;

pub mod alert;
pub mod api;
mod notification;
mod symbol;
pub mod worker;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
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
        let mut alerts = Alerts {
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

    /// Method check for prices and notify the alerts
    pub fn refresh(&mut self) {
        info!("Alerts::refresh started");
        thread::sleep(Duration::from_secs(1));
        self.create_alert("ETHUSDT", AlertType::ChangeOver(1.));
        println!("refresh::alerts {:?}", self.alerts);
        info!("Alerts::refresh closing");
    }
}

/// Main function to create/load alerts into memory
pub fn init() -> Alerts {
    Alerts::init()
}
